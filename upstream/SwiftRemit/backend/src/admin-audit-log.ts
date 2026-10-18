import { Pool } from 'pg';
import { AdminAuditLogRepository } from './repositories/AdminAuditLogRepository';

export interface AuditLogEntry {
  id: number;
  admin_address: string;
  action: string;
  target: string | null;
  params_json: Record<string, unknown> | null;
  tx_hash: string | null;
  ip_address: string | null;
  created_at: Date;
}

export interface AuditLogFilter {
  admin_address?: string;
  action?: string;
  from?: Date;
  to?: Date;
  limit?: number;
  offset?: number;
}

/**
 * Well-defined audit-log action vocabulary.
 *
 * Actions:
 * - `update_payout_address`: An agent's payout address was changed. The
 *   `params_json` payload carries `agent_id`, `old_payout_address`, and
 *   `new_payout_address`. The `target` field is set to the agent id.
 */
export const AuditAction = {
  UPDATE_PAYOUT_ADDRESS: 'update_payout_address',
} as const;

export type AuditActionType = (typeof AuditAction)[keyof typeof AuditAction];

/**
 * Typed payload for the `update_payout_address` audit action.
 */
export interface UpdatePayoutAddressParams {
  agent_id: string;
  old_payout_address: string | null;
  new_payout_address: string;
}

export class AdminAuditLogService {
  private repo: AdminAuditLogRepository;

  constructor(pool: Pool) {
    this.repo = new AdminAuditLogRepository(pool);
  }

  async log(entry: Omit<AuditLogEntry, 'id' | 'created_at'>): Promise<void> {
    await this.repo.insert(entry);
  }

  /**
   * Record an agent payout-address change.
   *
   * Writes an audit entry with action `update_payout_address` and a
   * `params_json` payload containing `agent_id`, `old_payout_address`, and
   * `new_payout_address`.
   */
  async logPayoutAddressChange(
    entry: {
      admin_address: string;
      target?: string | null;
      tx_hash?: string | null;
      ip_address?: string | null;
    },
    params: UpdatePayoutAddressParams,
  ): Promise<void> {
    await this.log({
      admin_address: entry.admin_address,
      action: AuditAction.UPDATE_PAYOUT_ADDRESS,
      target: entry.target ?? params.agent_id,
      params_json: {
        agent_id: params.agent_id,
        old_payout_address: params.old_payout_address,
        new_payout_address: params.new_payout_address,
      },
      tx_hash: entry.tx_hash ?? null,
      ip_address: entry.ip_address ?? null,
    });
  }

  async query(filter: AuditLogFilter = {}): Promise<{ entries: AuditLogEntry[]; total: number }> {
    return this.repo.query(filter);
  }

  async purgeOlderThan(days: number): Promise<number> {
    return this.repo.purgeOlderThan(days);
  }
}