import { Pool } from 'pg';

export interface AdminConfirmation {
  id: string;
  admin_id: string;
  action: string;
  payload: Record<string, unknown>;
  confirmed: boolean;
  created_at: Date;
  confirmed_at?: Date;
}

export class AdminConfirmationService {
  constructor(private readonly pool: Pool) {}

  /**
   * Create the admin_confirmations table if it does not already exist.
   * Must be called during application startup before any other methods.
   */
  async initTable(): Promise<void> {
    await this.pool.query(`
      CREATE TABLE IF NOT EXISTS admin_confirmations (
        id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        admin_id     VARCHAR(255) NOT NULL,
        action       VARCHAR(255) NOT NULL,
        payload      JSONB        NOT NULL DEFAULT '{}',
        confirmed    BOOLEAN      NOT NULL DEFAULT FALSE,
        created_at   TIMESTAMP    NOT NULL DEFAULT NOW(),
        confirmed_at TIMESTAMP
      )
    `);
  }

  async initiate(adminId: string, action: string, payload: Record<string, unknown>): Promise<AdminConfirmation> {
    const result = await this.pool.query<AdminConfirmation>(
      `INSERT INTO admin_confirmations (admin_id, action, payload)
       VALUES ($1, $2, $3)
       RETURNING *`,
      [adminId, action, JSON.stringify(payload)]
    );
    return result.rows[0];
  }

  async confirm(id: string, adminId: string): Promise<AdminConfirmation> {
    const result = await this.pool.query<AdminConfirmation>(
      `UPDATE admin_confirmations
       SET confirmed = TRUE, confirmed_at = NOW()
       WHERE id = $1 AND admin_id = $2 AND confirmed = FALSE
       RETURNING *`,
      [id, adminId]
    );
    if (result.rows.length === 0) {
      throw new Error(`No pending confirmation found for id=${id} and admin=${adminId}`);
    }
    return result.rows[0];
  }

  async getPending(adminId: string): Promise<AdminConfirmation[]> {
    const result = await this.pool.query<AdminConfirmation>(
      `SELECT * FROM admin_confirmations
       WHERE admin_id = $1 AND confirmed = FALSE
       ORDER BY created_at DESC`,
      [adminId]
    );
    return result.rows;
  }
}

let instance: AdminConfirmationService | null = null;

export function getAdminConfirmationService(pool: Pool): AdminConfirmationService {
  if (!instance) {
    instance = new AdminConfirmationService(pool);
  }
  return instance;
}
