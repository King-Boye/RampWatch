"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.storeVerificationOnChain = storeVerificationOnChain;
const stellar_sdk_1 = require("@stellar/stellar-sdk");
const types_1 = require("./types");
const server = new stellar_sdk_1.SorobanRpc.Server(process.env.HORIZON_URL || 'https://soroban-testnet.stellar.org');
async function storeVerificationOnChain(verification) {
    const contractId = process.env.CONTRACT_ID;
    if (!contractId) {
        throw new Error('CONTRACT_ID not configured');
    }
    const adminSecret = process.env.ADMIN_SECRET_KEY;
    if (!adminSecret) {
        throw new Error('ADMIN_SECRET_KEY not configured');
    }
    const adminKeypair = stellar_sdk_1.Keypair.fromSecret(adminSecret);
    const contract = new stellar_sdk_1.Contract(contractId);
    // Get admin account
    const account = await server.getAccount(adminKeypair.publicKey());
    // Map status to contract enum
    let statusValue;
    switch (verification.status) {
        case types_1.VerificationStatus.Verified:
            statusValue = stellar_sdk_1.xdr.ScVal.scvSymbol('Verified');
            break;
        case types_1.VerificationStatus.Suspicious:
            statusValue = stellar_sdk_1.xdr.ScVal.scvSymbol('Suspicious');
            break;
        default:
            statusValue = stellar_sdk_1.xdr.ScVal.scvSymbol('Unverified');
    }
    // Build transaction
    const tx = new stellar_sdk_1.TransactionBuilder(account, {
        fee: '1000',
        networkPassphrase: stellar_sdk_1.Networks.TESTNET,
    })
        .addOperation(contract.call('set_asset_verification', (0, stellar_sdk_1.nativeToScVal)(verification.asset_code, { type: 'string' }), new stellar_sdk_1.Address(verification.issuer).toScVal(), statusValue, (0, stellar_sdk_1.nativeToScVal)(verification.reputation_score, { type: 'u32' }), (0, stellar_sdk_1.nativeToScVal)(verification.trustline_count, { type: 'u64' }), (0, stellar_sdk_1.nativeToScVal)(verification.has_toml, { type: 'bool' })))
        .setTimeout(30)
        .build();
    // Simulate transaction
    const simulated = await server.simulateTransaction(tx);
    if (stellar_sdk_1.SorobanRpc.Api.isSimulationError(simulated)) {
        throw new Error(`Simulation failed: ${simulated.error}`);
    }
    // Prepare and sign transaction
    const prepared = stellar_sdk_1.SorobanRpc.assembleTransaction(tx, simulated).build();
    prepared.sign(adminKeypair);
    // Submit transaction
    const result = await server.sendTransaction(prepared);
    // Wait for confirmation
    let status = await server.getTransaction(result.hash);
    while (status.status === 'NOT_FOUND') {
        await new Promise(resolve => setTimeout(resolve, 1000));
        status = await server.getTransaction(result.hash);
    }
    if (status.status === 'FAILED') {
        throw new Error(`Transaction failed: ${status.resultXdr}`);
    }
    console.log(`Stored verification on-chain for ${verification.asset_code}-${verification.issuer}`);
}
