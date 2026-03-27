"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const express_rate_limit_1 = __importDefault(require("express-rate-limit"));
const helmet_1 = __importDefault(require("helmet"));
const cors_1 = __importDefault(require("cors"));
const verifier_1 = require("./verifier");
const database_1 = require("./database");
const stellar_1 = require("./stellar");
const types_1 = require("./types");
const kyc_upsert_service_1 = require("./kyc-upsert-service");
const transfer_guard_1 = require("./transfer-guard");
const app = (0, express_1.default)();
const verifier = new verifier_1.AssetVerifier();
// Security middleware
app.use((0, helmet_1.default)());
app.use((0, cors_1.default)());
app.use(express_1.default.json());
const pool = (0, database_1.getPool)();
const kycUpsertService = new kyc_upsert_service_1.KycUpsertService(pool);
const transferGuard = (0, transfer_guard_1.createTransferGuard)(kycUpsertService);
// Rate limiting
const limiter = (0, express_rate_limit_1.default)({
    windowMs: parseInt(process.env.RATE_LIMIT_WINDOW_MS || '900000'),
    max: parseInt(process.env.RATE_LIMIT_MAX_REQUESTS || '100'),
    message: 'Too many requests from this IP, please try again later.',
});
app.use('/api/', limiter);
// Input validation middleware
function validateAssetParams(req, res, next) {
    const { assetCode, issuer } = req.body;
    if (!assetCode || typeof assetCode !== 'string' || assetCode.length > 12) {
        return res.status(400).json({ error: 'Invalid asset code' });
    }
    if (!issuer || typeof issuer !== 'string' || issuer.length !== 56) {
        return res.status(400).json({ error: 'Invalid issuer address' });
    }
    next();
}
function authMiddleware(req, res, next) {
    const userId = req.headers['x-user-id'] || '';
    if (!userId || typeof userId !== 'string') {
        return res.status(401).json({ error: 'Unauthorized' });
    }
    req.user = { id: userId };
    next();
}
// Health check
app.get('/health', (req, res) => {
    res.json({ status: 'ok', timestamp: new Date().toISOString() });
});
// Get asset verification status
app.get('/api/verification/:assetCode/:issuer', async (req, res) => {
    try {
        const { assetCode, issuer } = req.params;
        // Input validation
        if (!assetCode || assetCode.length > 12) {
            return res.status(400).json({ error: 'Invalid asset code' });
        }
        if (!issuer || issuer.length !== 56) {
            return res.status(400).json({ error: 'Invalid issuer address' });
        }
        const verification = await (0, database_1.getAssetVerification)(assetCode, issuer);
        if (!verification) {
            return res.status(404).json({ error: 'Asset verification not found' });
        }
        res.json(verification);
    }
    catch (error) {
        console.error('Error fetching verification:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
});
// Verify asset (trigger new verification)
app.post('/api/verification/verify', validateAssetParams, async (req, res) => {
    try {
        const { assetCode, issuer } = req.body;
        // Perform verification
        const result = await verifier.verifyAsset(assetCode, issuer);
        // Save to database
        const verification = {
            asset_code: result.asset_code,
            issuer: result.issuer,
            status: result.status,
            reputation_score: result.reputation_score,
            last_verified: new Date(),
            trustline_count: result.trustline_count,
            has_toml: result.has_toml,
            stellar_expert_verified: result.sources.find(s => s.name === 'Stellar Expert')?.verified,
            toml_data: result.sources.find(s => s.name === 'Stellar TOML')?.details,
            community_reports: 0,
        };
        await (0, database_1.saveAssetVerification)(verification);
        // Store on-chain
        try {
            await (0, stellar_1.storeVerificationOnChain)(verification);
        }
        catch (error) {
            console.error('Failed to store on-chain:', error);
            // Continue even if on-chain storage fails
        }
        res.json({
            success: true,
            verification: result,
        });
    }
    catch (error) {
        console.error('Error verifying asset:', error);
        res.status(500).json({ error: 'Verification failed' });
    }
});
// Report suspicious asset
app.post('/api/verification/report', validateAssetParams, async (req, res) => {
    try {
        const { assetCode, issuer, reason } = req.body;
        if (!reason || typeof reason !== 'string' || reason.length > 500) {
            return res.status(400).json({ error: 'Invalid or missing reason' });
        }
        // Check if asset exists
        const existing = await (0, database_1.getAssetVerification)(assetCode, issuer);
        if (!existing) {
            return res.status(404).json({ error: 'Asset not found' });
        }
        // Increment report count
        await (0, database_1.reportSuspiciousAsset)(assetCode, issuer);
        // If reports exceed threshold, mark as suspicious
        const updated = await (0, database_1.getAssetVerification)(assetCode, issuer);
        if (updated && updated.community_reports && updated.community_reports >= 5) {
            updated.status = types_1.VerificationStatus.Suspicious;
            updated.reputation_score = Math.min(updated.reputation_score, 30);
            await (0, database_1.saveAssetVerification)(updated);
            // Update on-chain
            try {
                await (0, stellar_1.storeVerificationOnChain)(updated);
            }
            catch (error) {
                console.error('Failed to update on-chain:', error);
            }
        }
        res.json({
            success: true,
            message: 'Report submitted successfully',
        });
    }
    catch (error) {
        console.error('Error reporting asset:', error);
        res.status(500).json({ error: 'Failed to submit report' });
    }
});
// List verified assets
app.get('/api/verification/verified', async (req, res) => {
    try {
        const limit = Math.min(parseInt(req.query.limit) || 100, 500);
        const assets = await (0, database_1.getVerifiedAssets)(limit);
        res.json({
            count: assets.length,
            assets,
        });
    }
    catch (error) {
        console.error('Error fetching verified assets:', error);
        res.status(500).json({ error: 'Failed to fetch verified assets' });
    }
});
// Batch verification status
app.post('/api/verification/batch', async (req, res) => {
    try {
        const { assets } = req.body;
        if (!Array.isArray(assets) || assets.length === 0 || assets.length > 50) {
            return res.status(400).json({ error: 'Invalid assets array (max 50)' });
        }
        const results = await Promise.all(assets.map(async ({ assetCode, issuer }) => {
            try {
                const verification = await (0, database_1.getAssetVerification)(assetCode, issuer);
                return {
                    assetCode,
                    issuer,
                    verification: verification || null,
                };
            }
            catch (error) {
                return {
                    assetCode,
                    issuer,
                    verification: null,
                    error: 'Failed to fetch',
                };
            }
        }));
        res.json({ results });
    }
    catch (error) {
        console.error('Error in batch verification:', error);
        res.status(500).json({ error: 'Batch verification failed' });
    }
});
// KYC status endpoint
app.get('/api/kyc/status', authMiddleware, async (req, res) => {
    try {
        const userId = req.user?.id;
        if (!userId) {
            return res.status(401).json({ error: 'Unauthorized' });
        }
        const status = await kycUpsertService.getStatusForUser(userId);
        return res.status(200).json(status);
    }
    catch (error) {
        console.error('Error fetching KYC status:', error);
        return res.status(500).json({ error: 'Internal server error' });
    }
});
// Transfer endpoint (guarded)
app.post('/api/transfer', authMiddleware, transferGuard, async (req, res) => {
    return res.status(200).json({ success: true, message: 'Transfer allowed' });
});
// Store FX rate for transaction
app.post('/api/fx-rate', async (req, res) => {
    try {
        const { transactionId, rate, provider, fromCurrency, toCurrency } = req.body;
        if (!transactionId || typeof transactionId !== 'string') {
            return res.status(400).json({ error: 'Invalid transaction ID' });
        }
        if (!rate || typeof rate !== 'number' || rate <= 0) {
            return res.status(400).json({ error: 'Invalid rate' });
        }
        if (!provider || typeof provider !== 'string') {
            return res.status(400).json({ error: 'Invalid provider' });
        }
        if (!fromCurrency || !toCurrency) {
            return res.status(400).json({ error: 'Invalid currencies' });
        }
        await (0, database_1.saveFxRate)({
            transaction_id: transactionId,
            rate,
            provider,
            timestamp: new Date(),
            from_currency: fromCurrency,
            to_currency: toCurrency,
        });
        res.json({ success: true, message: 'FX rate stored successfully' });
    }
    catch (error) {
        console.error('Error storing FX rate:', error);
        res.status(500).json({ error: 'Failed to store FX rate' });
    }
});
// Get FX rate for transaction
app.get('/api/fx-rate/:transactionId', async (req, res) => {
    try {
        const { transactionId } = req.params;
        if (!transactionId) {
            return res.status(400).json({ error: 'Invalid transaction ID' });
        }
        const fxRate = await (0, database_1.getFxRate)(transactionId);
        if (!fxRate) {
            return res.status(404).json({ error: 'FX rate not found for this transaction' });
        }
        res.json(fxRate);
    }
    catch (error) {
        console.error('Error fetching FX rate:', error);
        res.status(500).json({ error: 'Failed to fetch FX rate' });
    }
});
exports.default = app;
