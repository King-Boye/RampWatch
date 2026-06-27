import { Router, Request, Response } from 'express';
import { ErrorResponse } from '../types';
import { Pool } from 'pg';
import { AdminConfirmationService, HighRiskOperation } from '../admin-confirmation';
import { Readable } from 'stream';

function timestamp(): string {
  return new Date().toISOString();
}

function sendError(res: Response, status: number, message: string, code: string): Response<ErrorResponse> {
  return res.status(status).json({ success: false, error: { message, code }, timestamp: timestamp() });
}

/** Validate a 32-byte WASM hash supplied as a 64-char hex string */
function isValidWasmHash(value: unknown): value is string {
  return typeof value === 'string' && /^[0-9a-fA-F]{64}$/.test(value);
}

/**
 * Validate admin API key from the x-api-key header.
 * Returns true if the key matches the configured admin key.
 */
function isAdminAuthorized(req: Request): boolean {
  const adminKey = process.env.ADMIN_API_KEY;
  if (!adminKey) return false;
  return req.headers['x-api-key'] === adminKey;
}

export interface IntegratorFeeEntry {
  integrator: string;
  accumulated_fees: number;
}

export interface FeeTimeSeries {
  period: 'daily' | 'weekly' | 'monthly';
  label: string;
  amount: number;
}

export interface FeeBreakdownData {
  total_accumulated_fees: number;
  pending_withdrawal: number;
  integrator_breakdown: IntegratorFeeEntry[];
  time_series: FeeTimeSeries[];
}

/**
 * Stub: in production this queries the contract via RPC and/or the event DB.
 */
function fetchFeeBreakdown(): FeeBreakdownData {
  return {
    total_accumulated_fees: 0,
    pending_withdrawal: 0,
    integrator_breakdown: [],
    time_series: [],
  };
}

/**
 * Simulate what a contract upgrade would do without applying any state changes.
 *
 * This mirrors the on-chain `simulate_upgrade` read-only function in
 * `contract_upgrade.rs`.  The API layer performs the same heuristic so callers
 * can preview migration impact before submitting a proposal.
 */
function simulateUpgrade(wasmHashHex: string): {
  current_schema_version: number;
  new_schema_version: number;
  schema_version_delta: number;
  estimated_migration_steps: number;
  affected_storage_keys: string[];
  requires_migration: boolean;
} {
  // In a production deployment this would query the live contract via RPC.
  // Here we use the same deterministic heuristic as the on-chain function so
  // the REST response is always consistent with what the contract would return.
  const CURRENT_SCHEMA_VERSION = parseInt(process.env.CONTRACT_SCHEMA_VERSION ?? '0', 10);
  const firstByte = parseInt(wasmHashHex.slice(0, 2), 16);
  const newSchemaVersion = CURRENT_SCHEMA_VERSION + 1 + (firstByte % 3);
  const delta = newSchemaVersion - CURRENT_SCHEMA_VERSION;
  const requiresMigration = delta > 0;

  const affectedKeys = requiresMigration
    ? ['schema_v', 'UpgradeKey::NextId', 'UpgradeKey::PendingCount']
    : [];

  return {
    current_schema_version: CURRENT_SCHEMA_VERSION,
    new_schema_version: newSchemaVersion,
    schema_version_delta: delta,
    estimated_migration_steps: Math.abs(delta),
    affected_storage_keys: affectedKeys,
    requires_migration: requiresMigration,
  };
}

const HIGH_RISK_OPS: HighRiskOperation[] = ['withdraw_fees', 'remove_agent', 'update_fee'];

function getConfirmationService(): AdminConfirmationService | null {
  const dbUrl = process.env.DATABASE_URL;
  if (!dbUrl) return null;
  const pool = new Pool({ connectionString: dbUrl });
  return new AdminConfirmationService(pool);
}

function escapeCsvField(field: string | number | null | undefined): string {
  if (field === null || field === undefined) return '';
  const str = String(field);
  if (str.includes(',') || str.includes('"') || str.includes('\n')) {
    return `"${str.replace(/"/g, '""')}"`;
  }
  return str;
}

async function* streamRemittancesCsv(
  pool: Pool,
  fromDate?: Date,
  toDate?: Date,
  status?: string
): AsyncGenerator<string> {
  const headers = [
    'id',
    'sender',
    'recipient',
    'agent',
    'amount',
    'fee',
    'currency',
    'status',
    'corridor',
    'created_at',
    'updated_at',
    'memo',
  ];
  
  yield headers.map(escapeCsvField).join(',') + '\n';

  const query = `
    SELECT id, sender, recipient, agent, amount, fee, currency, status, corridor, created_at, updated_at, memo
    FROM remittances
    WHERE 1=1
      ${fromDate ? 'AND created_at >= $1' : ''}
      ${toDate ? `AND created_at <= ${fromDate ? '$2' : '$1'}` : ''}
      ${status ? `AND status = ${toDate ? '$3' : fromDate ? '$2' : '$1'}` : ''}
    ORDER BY created_at ASC
  `;

  const params: (Date | string)[] = [];
  if (fromDate) params.push(fromDate);
  if (toDate) params.push(toDate);
  if (status) params.push(status);

  const client = await pool.connect();
  try {
    const query_text = `
      SELECT id, sender, recipient, agent, amount, fee, currency, status, corridor, created_at, updated_at, memo
      FROM remittances
      ${fromDate || toDate || status ? 'WHERE' : ''}
      ${fromDate ? 'created_at >= $1' : ''}
      ${toDate ? (fromDate ? 'AND' : '') + ' created_at <= $' + (params.length + 1) : ''}
      ${status ? (fromDate || toDate ? 'AND' : '') + ' status = $' + (params.length + 1) : ''}
      ORDER BY created_at ASC
    `;

    const stream = client.query(query_text, params);
    
    for await (const row of stream) {
      const csvRow = [
        row.id,
        row.sender,
        row.recipient,
        row.agent,
        row.amount,
        row.fee,
        row.currency,
        row.status,
        row.corridor,
        row.created_at,
        row.updated_at,
        row.memo || '',
      ];
      yield csvRow.map(escapeCsvField).join(',') + '\n';
    }
  } finally {
    client.release();
  }
}

export function createAdminRouter(): Router {
  const router = Router();

  /**
   * @openapi
   * /api/admin/remittances/export:
   *   get:
   *     summary: Export remittances to CSV (admin only)
   *     description: >
   *       Stream remittance records as CSV for compliance reporting.
   *       Supports filtering by date range and status. Uses cursor streaming to avoid OOM.
   *       Requires admin authentication via x-api-key header.
   *     tags:
   *       - Admin
   *     security:
   *       - ApiKeyAuth: []
   *     parameters:
   *       - name: from
   *         in: query
   *         required: false
   *         description: Start date (ISO 8601)
   *         schema:
   *           type: string
   *       - name: to
   *         in: query
   *         required: false
   *         description: End date (ISO 8601)
   *         schema:
   *           type: string
   *       - name: status
   *         in: query
   *         required: false
   *         description: Filter by status
   *         schema:
   *           type: string
   *           enum: [Pending, Processing, Completed, Cancelled, Failed, Disputed]
   *     responses:
   *       200:
   *         description: CSV stream
   *         content:
   *           text/csv:
   *             schema:
   *               type: string
   *       401:
   *         description: Unauthorized
   */
  router.get('/remittances/export', async (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const { from, to, status } = req.query as Record<string, string | undefined>;
    
    let fromDate: Date | undefined;
    let toDate: Date | undefined;

    if (from) {
      fromDate = new Date(from);
      if (isNaN(fromDate.getTime())) {
        return sendError(res, 400, 'Invalid from date format', 'INVALID_FROM_DATE');
      }
    }

    if (to) {
      toDate = new Date(to);
      if (isNaN(toDate.getTime())) {
        return sendError(res, 400, 'Invalid to date format', 'INVALID_TO_DATE');
      }
    }

    if (status && !['Pending', 'Processing', 'Completed', 'Cancelled', 'Failed', 'Disputed'].includes(status)) {
      return sendError(res, 400, 'Invalid status', 'INVALID_STATUS');
    }

    const dbUrl = process.env.DATABASE_URL;
    if (!dbUrl) {
      return sendError(res, 503, 'Database not configured', 'DB_UNAVAILABLE');
    }

    const pool = new Pool({ connectionString: dbUrl });

    res.setHeader('Content-Type', 'text/csv');
    res.setHeader('Content-Disposition', `attachment; filename="remittances-${new Date().toISOString()}.csv"`);

    try {
      const generator = streamRemittancesCsv(pool, fromDate, toDate, status);
      for await (const chunk of generator) {
        res.write(chunk);
      }
    } finally {
      res.end();
      await pool.end();
    }
  });

  /**
   * @openapi
   * /api/admin/fees:
   *   get:
   *     summary: Get accumulated fee breakdown (admin only)
   *     description: >
   *       Returns total accumulated platform fees, per-integrator breakdown,
   *       daily/weekly/monthly time-series, and pending withdrawal amount.
   *       Requires admin authentication via x-api-key header.
   *     tags:
   *       - Admin
   *     security:
   *       - ApiKeyAuth: []
   *     responses:
   *       200:
   *         description: Fee breakdown data
   *       401:
   *         description: Unauthorized
   */
  router.get('/fees', (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const data = fetchFeeBreakdown();

    return res.json({
      success: true,
      data,
      timestamp: timestamp(),
    });
  });

  /**
   * @openapi
   * /api/admin/simulate-upgrade:
   *   post:
   *     summary: Simulate a contract upgrade (read-only, requires 2FA)
   *     description: >
   *       Returns a preview of the storage migrations that would be applied if
   *       the supplied WASM hash were used in a real upgrade proposal.  No
   *       on-chain state is modified. Requires admin API key and a valid
   *       confirmation token from a second admin.
   *     tags:
   *       - Admin
   *     security:
   *       - ApiKeyAuth: []
   *     requestBody:
   *       required: true
   *       content:
   *         application/json:
   *           schema:
   *             type: object
   *             required:
   *               - wasm_hash
   *               - confirmation_token
   *             properties:
   *               wasm_hash:
   *                 type: string
   *                 description: 64-character hex-encoded 32-byte WASM hash
   *                 example: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
   *               confirmation_token:
   *                 type: string
   *                 description: Confirmation token from second admin
   *     responses:
   *       200:
   *         description: Simulation result
   *         content:
   *           application/json:
   *             schema:
   *               type: object
   *               properties:
   *                 success:
   *                   type: boolean
   *                   example: true
   *                 data:
   *                   type: object
   *                   properties:
   *                     current_schema_version:
   *                       type: integer
   *                     new_schema_version:
   *                       type: integer
   *                     schema_version_delta:
   *                       type: integer
   *                     estimated_migration_steps:
   *                       type: integer
   *                     affected_storage_keys:
   *                       type: array
   *                       items:
   *                         type: string
   *                     requires_migration:
   *                       type: boolean
   *                 timestamp:
   *                   type: string
   *                   format: date-time
   *       400:
   *         description: Invalid wasm_hash or missing confirmation_token
   *       401:
   *         description: Unauthorized or invalid confirmation token
   */
  router.post('/simulate-upgrade', async (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const { wasm_hash, confirmation_token } = req.body as Record<string, unknown>;

    if (!confirmation_token || typeof confirmation_token !== 'string') {
      return sendError(
        res,
        400,
        'confirmation_token is required for this high-risk operation',
        'MISSING_CONFIRMATION_TOKEN',
      );
    }

    if (!isValidWasmHash(wasm_hash)) {
      return sendError(
        res,
        400,
        'wasm_hash must be a 64-character hex string (32 bytes)',
        'INVALID_WASM_HASH',
      );
    }

    // Verify the confirmation token
    const svc = getConfirmationService();
    if (!svc) {
      return sendError(res, 503, 'Database not configured', 'DB_UNAVAILABLE');
    }

    try {
      await svc.initTable();
      const action = await svc.verify(confirmation_token);

      if (!action) {
        return sendError(res, 401, 'Invalid or expired confirmation token', 'INVALID_CONFIRMATION_TOKEN');
      }

      // Token is valid, proceed with simulation
      const result = simulateUpgrade(wasm_hash);

      res.json({
        success: true,
        data: result,
        timestamp: timestamp(),
      });
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Confirmation verification failed';
      return sendError(res, 401, msg, 'CONFIRMATION_VERIFICATION_FAILED');
    }
  });

  // ── Multi-step admin confirmation (#481) ──────────────────────────────────

  /**
   * POST /api/admin/actions
   * Initiate a high-risk operation requiring a second admin to confirm.
   */
  router.post('/actions', async (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const { operation, initiated_by, params } = req.body as Record<string, unknown>;

    if (!operation || !HIGH_RISK_OPS.includes(operation as HighRiskOperation)) {
      return sendError(res, 400, `operation must be one of: ${HIGH_RISK_OPS.join(', ')}`, 'INVALID_OPERATION');
    }
    if (typeof initiated_by !== 'string' || !initiated_by) {
      return sendError(res, 400, 'initiated_by is required', 'MISSING_FIELD');
    }

    const svc = getConfirmationService();
    if (!svc) return sendError(res, 503, 'Database not configured', 'DB_UNAVAILABLE');

    try {
      await svc.initTable();
      const action = await svc.initiate(
        operation as HighRiskOperation,
        initiated_by,
        (params as Record<string, unknown>) ?? {}
      );
      return res.status(201).json({ success: true, data: action, timestamp: timestamp() });
    } catch (err) {
      return sendError(res, 500, err instanceof Error ? err.message : 'Failed to initiate action', 'INITIATE_FAILED');
    }
  });

  /**
   * GET /api/admin/actions
   * List all pending (unconfirmed, non-expired) high-risk actions.
   */
  router.get('/actions', async (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const svc = getConfirmationService();
    if (!svc) return sendError(res, 503, 'Database not configured', 'DB_UNAVAILABLE');

    try {
      await svc.initTable();
      const actions = await svc.listPending();
      return res.json({ success: true, data: actions, timestamp: timestamp() });
    } catch (err) {
      return sendError(res, 500, err instanceof Error ? err.message : 'Failed to list actions', 'LIST_FAILED');
    }
  });

  /**
   * POST /api/admin/actions/:id/confirm
   * Second admin confirms a pending high-risk action.
   */
  router.post('/actions/:id/confirm', async (req: Request, res: Response) => {
    if (!isAdminAuthorized(req)) {
      return sendError(res, 401, 'Admin authentication required', 'UNAUTHORIZED');
    }

    const { confirmed_by } = req.body as Record<string, unknown>;
    if (typeof confirmed_by !== 'string' || !confirmed_by) {
      return sendError(res, 400, 'confirmed_by is required', 'MISSING_FIELD');
    }

    const svc = getConfirmationService();
    if (!svc) return sendError(res, 503, 'Database not configured', 'DB_UNAVAILABLE');

    try {
      await svc.initTable();
      const action = await svc.confirm(req.params.id, confirmed_by);
      return res.json({ success: true, data: action, timestamp: timestamp() });
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Confirmation failed';
      const isNotFound = msg.includes('not found');
      const isExpired = msg.includes('expired');
      const isSelf = msg.includes('cannot confirm');
      const status = isNotFound ? 404 : isExpired || isSelf ? 409 : 500;
      return sendError(res, status, msg, 'CONFIRM_FAILED');
    }
  });

  return router;
}
