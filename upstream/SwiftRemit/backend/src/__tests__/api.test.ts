import { describe, it, expect, beforeAll, afterAll, vi } from 'vitest';
import request from 'supertest';
import app from '../api';
import { initDatabase, getPool } from '../database';
import { WebhookHandler } from '../webhook-handler';

describe('API Endpoints', () => {
  beforeAll(async () => {
    await initDatabase();
  });

  describe('GET /health', () => {
    it('should return health status', async () => {
      const response = await request(app).get('/health');
      expect(response.status).toBe(200);
      expect(response.body.status).toBe('ok');
    });
  });

  describe('GET /api/verification/:assetCode/:issuer', () => {
    it('should return 400 for invalid asset code', async () => {
      const response = await request(app).get(
        '/api/verification/TOOLONGASSETCODE/GXXX'
      );
      expect(response.status).toBe(400);
    });

    it('should return 400 for invalid issuer', async () => {
      const response = await request(app).get('/api/verification/USDC/INVALID');
      expect(response.status).toBe(400);
    });

    it('should return 404 for non-existent asset', async () => {
      const response = await request(app).get(
        '/api/verification/NOTFOUND/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN'
      );
      expect(response.status).toBe(404);
    });
  });

  describe('POST /api/verification/verify', () => {
    it('should verify an asset', async () => {
      const response = await request(app)
        .post('/api/verification/verify')
        .send({
          assetCode: 'USDC',
          issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
        });

      expect(response.status).toBe(200);
      expect(response.body.success).toBe(true);
      expect(response.body.verification).toBeDefined();
    });

    it('should reject invalid input', async () => {
      const response = await request(app)
        .post('/api/verification/verify')
        .send({
          assetCode: 'TOOLONGASSETCODE',
          issuer: 'INVALID',
        });

      expect(response.status).toBe(400);
    });
  });

  describe('POST /api/verification/report', () => {
    it('should require reason', async () => {
      const response = await request(app)
        .post('/api/verification/report')
        .send({
          assetCode: 'USDC',
          issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
        });

      expect(response.status).toBe(400);
    });

    it('should reject too long reason', async () => {
      const response = await request(app)
        .post('/api/verification/report')
        .send({
          assetCode: 'USDC',
          issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
          reason: 'x'.repeat(501),
        });

      expect(response.status).toBe(400);
    });
  });

  describe('GET /api/verification/verified', () => {
    it('should return verified assets', async () => {
      const response = await request(app).get('/api/verification/verified');
      expect(response.status).toBe(200);
      expect(response.body.assets).toBeDefined();
      expect(Array.isArray(response.body.assets)).toBe(true);
    });

    it('should respect limit parameter', async () => {
      const response = await request(app).get('/api/verification/verified?limit=10');
      expect(response.status).toBe(200);
      expect(response.body.assets.length).toBeLessThanOrEqual(10);
    });
  });

  describe('POST /api/verification/batch', () => {
    it('should handle batch requests', async () => {
      const response = await request(app)
        .post('/api/verification/batch')
        .send({
          assets: [
            {
              assetCode: 'USDC',
              issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
            },
          ],
        });

      expect(response.status).toBe(200);
      expect(response.body.results).toBeDefined();
      expect(Array.isArray(response.body.results)).toBe(true);
    });

    it('should reject too many assets', async () => {
      const assets = Array(51).fill({
        assetCode: 'USDC',
        issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
      });

      const response = await request(app)
        .post('/api/verification/batch')
        .send({ assets });

      expect(response.status).toBe(400);
    });
  });

  describe('GET /api/kyc/status', () => {
    it('should reject unauthenticated requests', async () => {
      const response = await request(app).get('/api/kyc/status');
      expect(response.status).toBe(401);
    });

    it('should return pending for user with no KYC records', async () => {
      const response = await request(app)
        .get('/api/kyc/status')
        .set('x-user-id', 'user-no-kyc');

      expect(response.status).toBe(200);
      expect(response.body.overall_status).toBe('pending');
      expect(response.body.can_transfer).toBe(false);
      expect(response.body.reason).toBe('no_kyc_record');
      expect(Array.isArray(response.body.anchors)).toBe(true);
    });
  });

  describe('POST /api/transfer', () => {
    it('should reject unauthenticated requests', async () => {
      const response = await request(app).post('/api/transfer').send({});
      expect(response.status).toBe(401);
    });

    it('should reject when KYC not approved', async () => {
      const response = await request(app)
        .post('/api/transfer')
        .set('x-user-id', 'user-no-kyc')
        .send({});

      expect(response.status).toBe(403);
      expect(response.body.error).toBeDefined();
      expect(response.body.error.code).toBe('KYC_PENDING');
    });
  });

  describe('WebhookHandler KYC update flow', () => {
    it('should update both transactions and KYC status store', async () => {
      const pool = getPool();
      const webhookHandler = new WebhookHandler(pool);

      const updateSpy = vi.fn();
      const upsertSpy = vi.fn();

      // Replace internals with test doubles
      (webhookHandler as any).stateManager = { updateKYCStatus: updateSpy };
      (webhookHandler as any).kycUpsertService = { upsert: upsertSpy };

      const payload = {
        transaction_id: 'tx-abc',
        kyc_status: 'approved',
        kyc_fields: { name: 'Jane Doe' },
        user_id: 'user-abc',
        anchor_id: 'anchor-abc',
        verified_at: new Date().toISOString(),
        expires_at: new Date(Date.now() + 24 * 3600 * 1000).toISOString(),
      };

      await (webhookHandler as any).handleKYCUpdate(payload, 'anchor-abc');

      expect(updateSpy).toHaveBeenCalledWith({
        transaction_id: 'tx-abc',
        kyc_status: 'approved',
        kyc_fields: { name: 'Jane Doe' },
        rejection_reason: undefined,
      });

      expect(upsertSpy).toHaveBeenCalledWith(expect.objectContaining({
        user_id: 'user-abc',
        anchor_id: 'anchor-abc',
        kyc_status: 'approved',
      }));
    });
  });
});
