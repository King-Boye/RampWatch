import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import request from 'supertest';
import app from '../api';
import { initDatabase } from '../database';

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
});
