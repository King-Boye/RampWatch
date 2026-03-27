"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const vitest_1 = require("vitest");
const supertest_1 = __importDefault(require("supertest"));
const api_1 = __importDefault(require("../api"));
const database_1 = require("../database");
const webhook_handler_1 = require("../webhook-handler");
(0, vitest_1.describe)('API Endpoints', () => {
    (0, vitest_1.beforeAll)(async () => {
        await (0, database_1.initDatabase)();
    });
    (0, vitest_1.describe)('GET /health', () => {
        (0, vitest_1.it)('should return health status', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/health');
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.status).toBe('ok');
        });
    });
    (0, vitest_1.describe)('GET /api/verification/:assetCode/:issuer', () => {
        (0, vitest_1.it)('should return 400 for invalid asset code', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/verification/TOOLONGASSETCODE/GXXX');
            (0, vitest_1.expect)(response.status).toBe(400);
        });
        (0, vitest_1.it)('should return 400 for invalid issuer', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/verification/USDC/INVALID');
            (0, vitest_1.expect)(response.status).toBe(400);
        });
        (0, vitest_1.it)('should return 404 for non-existent asset', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/verification/NOTFOUND/GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN');
            (0, vitest_1.expect)(response.status).toBe(404);
        });
    });
    (0, vitest_1.describe)('POST /api/verification/verify', () => {
        (0, vitest_1.it)('should verify an asset', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/verify')
                .send({
                assetCode: 'USDC',
                issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
            });
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.success).toBe(true);
            (0, vitest_1.expect)(response.body.verification).toBeDefined();
        });
        (0, vitest_1.it)('should reject invalid input', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/verify')
                .send({
                assetCode: 'TOOLONGASSETCODE',
                issuer: 'INVALID',
            });
            (0, vitest_1.expect)(response.status).toBe(400);
        });
    });
    (0, vitest_1.describe)('POST /api/verification/report', () => {
        (0, vitest_1.it)('should require reason', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/report')
                .send({
                assetCode: 'USDC',
                issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
            });
            (0, vitest_1.expect)(response.status).toBe(400);
        });
        (0, vitest_1.it)('should reject too long reason', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/report')
                .send({
                assetCode: 'USDC',
                issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
                reason: 'x'.repeat(501),
            });
            (0, vitest_1.expect)(response.status).toBe(400);
        });
    });
    (0, vitest_1.describe)('GET /api/verification/verified', () => {
        (0, vitest_1.it)('should return verified assets', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/verification/verified');
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.assets).toBeDefined();
            (0, vitest_1.expect)(Array.isArray(response.body.assets)).toBe(true);
        });
        (0, vitest_1.it)('should respect limit parameter', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/verification/verified?limit=10');
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.assets.length).toBeLessThanOrEqual(10);
        });
    });
    (0, vitest_1.describe)('POST /api/verification/batch', () => {
        (0, vitest_1.it)('should handle batch requests', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/batch')
                .send({
                assets: [
                    {
                        assetCode: 'USDC',
                        issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
                    },
                ],
            });
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.results).toBeDefined();
            (0, vitest_1.expect)(Array.isArray(response.body.results)).toBe(true);
        });
        (0, vitest_1.it)('should reject too many assets', async () => {
            const assets = Array(51).fill({
                assetCode: 'USDC',
                issuer: 'GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN',
            });
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/verification/batch')
                .send({ assets });
            (0, vitest_1.expect)(response.status).toBe(400);
        });
    });
    (0, vitest_1.describe)('GET /api/kyc/status', () => {
        (0, vitest_1.it)('should reject unauthenticated requests', async () => {
            const response = await (0, supertest_1.default)(api_1.default).get('/api/kyc/status');
            (0, vitest_1.expect)(response.status).toBe(401);
        });
        (0, vitest_1.it)('should return pending for user with no KYC records', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .get('/api/kyc/status')
                .set('x-user-id', 'user-no-kyc');
            (0, vitest_1.expect)(response.status).toBe(200);
            (0, vitest_1.expect)(response.body.overall_status).toBe('pending');
            (0, vitest_1.expect)(response.body.can_transfer).toBe(false);
            (0, vitest_1.expect)(response.body.reason).toBe('no_kyc_record');
            (0, vitest_1.expect)(Array.isArray(response.body.anchors)).toBe(true);
        });
    });
    (0, vitest_1.describe)('POST /api/transfer', () => {
        (0, vitest_1.it)('should reject unauthenticated requests', async () => {
            const response = await (0, supertest_1.default)(api_1.default).post('/api/transfer').send({});
            (0, vitest_1.expect)(response.status).toBe(401);
        });
        (0, vitest_1.it)('should reject when KYC not approved', async () => {
            const response = await (0, supertest_1.default)(api_1.default)
                .post('/api/transfer')
                .set('x-user-id', 'user-no-kyc')
                .send({});
            (0, vitest_1.expect)(response.status).toBe(403);
            (0, vitest_1.expect)(response.body.error).toBeDefined();
            (0, vitest_1.expect)(response.body.error.code).toBe('KYC_PENDING');
        });
    });
    (0, vitest_1.describe)('WebhookHandler KYC update flow', () => {
        (0, vitest_1.it)('should update both transactions and KYC status store', async () => {
            const pool = (0, database_1.getPool)();
            const webhookHandler = new webhook_handler_1.WebhookHandler(pool);
            const updateSpy = vitest_1.vi.fn();
            const upsertSpy = vitest_1.vi.fn();
            // Replace internals with test doubles
            webhookHandler.stateManager = { updateKYCStatus: updateSpy };
            webhookHandler.kycUpsertService = { upsert: upsertSpy };
            const payload = {
                transaction_id: 'tx-abc',
                kyc_status: 'approved',
                kyc_fields: { name: 'Jane Doe' },
                user_id: 'user-abc',
                anchor_id: 'anchor-abc',
                verified_at: new Date().toISOString(),
                expires_at: new Date(Date.now() + 24 * 3600 * 1000).toISOString(),
            };
            await webhookHandler.handleKYCUpdate(payload, 'anchor-abc');
            (0, vitest_1.expect)(updateSpy).toHaveBeenCalledWith({
                transaction_id: 'tx-abc',
                kyc_status: 'approved',
                kyc_fields: { name: 'Jane Doe' },
                rejection_reason: undefined,
            });
            (0, vitest_1.expect)(upsertSpy).toHaveBeenCalledWith(vitest_1.expect.objectContaining({
                user_id: 'user-abc',
                anchor_id: 'anchor-abc',
                kyc_status: 'approved',
            }));
        });
    });
});
