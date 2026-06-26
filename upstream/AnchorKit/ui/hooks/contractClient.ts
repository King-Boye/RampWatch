/**
 * Contract layer for AnchorKit's Soroban smart contract.
 *
 * Each function here maps to a read-only view on the on-chain contract.
 * In production these would be wired to a Soroban RPC endpoint via
 * @stellar/stellar-sdk (or a hosted serverless proxy).  The signatures are
 * stable so callers and hooks can import the types without importing the SDK.
 *
 * Stellar ledger context
 * ─────────────────────
 * Stellar targets a ~5-second ledger close time.  The rate-limit window is
 * measured in ledgers (e.g. 100 ledgers ≈ 8 minutes).  `RateLimitStatusRaw`
 * carries both ledger numbers and the most-recent ledger's Unix timestamp so
 * that callers can project wall-clock deadlines without a separate RPC round-
 * trip.
 */

/** Raw values returned directly from the Soroban contract. */
export interface RateLimitStatusRaw {
  /** Submissions used in the current window (maps to `submission_count`). */
  submissionCount: number;
  /** Maximum submissions allowed per window (maps to `max_submissions`). */
  maxSubmissions: number;
  /** Ledger number when the current rate-limit window opened. */
  windowStartLedger: number;
  /** Window duration in ledgers (maps to `window_length`). */
  windowLength: number;
  /** Ledger sequence number at the time of the RPC call. */
  currentLedger: number;
  /**
   * Unix timestamp (seconds) of the most-recently closed ledger.
   * Used to project `windowResetsAt` into wall-clock time without an
   * additional RPC call.
   */
  ledgerTimestamp: number;
}

/** Typed error thrown when the contract call fails. */
export class ContractError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
  ) {
    super(message);
    this.name = 'ContractError';
  }
}

/**
 * Fetch the current rate-limit status for `attestor`.
 *
 * Combines `RateLimiter::get_state` and `RateLimiter::get_effective_config`
 * from the Rust contract into a single typed response.
 *
 * @throws {ContractError} when the RPC call fails or `attestor` is unknown.
 *
 * Production wiring (not included — no SDK peer-dep in this package):
 * ```ts
 * import { Contract, rpc } from '@stellar/stellar-sdk';
 * const server = new rpc.Server(process.env.STELLAR_RPC_URL);
 * const contract = new Contract(process.env.CONTRACT_ID);
 * const result = await server.simulateTransaction(
 *   buildGetRateLimitStatusTx(contract, attestor)
 * );
 * return parseRateLimitStatusRaw(result);
 * ```
 */
export async function getRateLimitStatus(
  _attestor: string,
): Promise<RateLimitStatusRaw> {
  throw new ContractError(
    'getRateLimitStatus: production contract client is not configured. ' +
    'Wire this function to a Soroban RPC endpoint or inject a mock via the ' +
    '`getStatus` option when using useRateLimitStatus.',
    'NOT_CONFIGURED',
  );
}
