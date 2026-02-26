import { describe, it, expect } from 'vitest';
import request from 'supertest';
import { createApp } from '../app';

describe('Anchors API', () => {
  const app = createApp();

  describe('GET /api/anchors', () => {
    it('should return all active anchors', async () => {
      const response = await request(app)
        .get('/api/anchors?status=active')
        .expect(200);

      expect(response.body.success).toBe(true);
      expect(response.body.data).toBeInstanceOf(Array);
      expect(response.body.count).toBeGreaterThan(0);
      expect(response.body.timestamp).toBeDefined();
    });

    it('should filter anchors by currency', async () => {
      const response = await request(app)
        .get('/api/anchors?currency=USD')
        .expect(200);

      expect(response.body.success).toBe(true);
      response.body.data.forEach((anchor: any) => {
        expect(anchor.supported_currencies).toContain('USD');
      });
    });

    it('should return anchor with complete structure', async () => {
      const response = await request(app)
        .get('/api/anchors')
        .expect(200);

      const anchor = response.body.data[0];
      expect(anchor).toHaveProperty('id');
      expect(anchor).toHaveProperty('name');
      expect(anchor).toHaveProperty('fees');
      expect(anchor).toHaveProperty('limits');
      expect(anchor).toHaveProperty('compliance');
      expect(anchor.fees).toHaveProperty('deposit_fee_percent');
      expect(anchor.fees).toHaveProperty('withdrawal_fee_percent');
      expect(anchor.limits).toHaveProperty('min_amount');
      expect(anchor.limits).toHaveProperty('max_amount');
      expect(anchor.compliance).toHaveProperty('kyc_required');
      expect(anchor.compliance).toHaveProperty('kyc_level');
    });
  });

  describe('GET /api/anchors/:id', () => {
    it('should return specific anchor by id', async () => {
      const response = await request(app)
        .get('/api/anchors/anchor-1')
        .expect(200);

      expect(response.body.success).toBe(true);
      expect(response.body.data.id).toBe('anchor-1');
    });

    it('should return 404 for non-existent anchor', async () => {
      const response = await request(app)
        .get('/api/anchors/non-existent')
        .expect(404);

      expect(response.body.success).toBe(false);
      expect(response.body.error.code).toBe('ANCHOR_NOT_FOUND');
    });
  });
});
