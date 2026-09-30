/**
 * SwiftRemit Proof Validation Example
 *
 * Demonstrates how to use the off-chain proof validation feature:
 * 1. Create a remittance with proof validation required
 * 2. Compute the proof (settlement commitment) off-chain
 * 3. Confirm payout with the valid proof
 * 4. Handle proof validation errors
 *
 * Prerequisites:
 * - Node.js 18+
 * - npm install @stellar/stellar-sdk
 * - Contract deployed to testnet
 *
 * Usage:
 * node proof-validation-example.js
 */

const StellarSdk = require('@stellar/stellar-sdk');
const xdr = StellarSdk.xdr;

// === Configuration ===
const config = {
  rpcUrl: process.env.RPC_URL || 'https://soroban-testnet.stellar.org:443',
  networkPassphrase: process.env.NETWORK_PASSPHRASE || StellarSdk.Networks.TESTNET,
  contractId: process.env.SWIFTREMIT_CONTRACT_ID,
  usdcTokenId: process.env.USDC_TOKEN_ID,
  adminSecret: process.env.ADMIN_SECRET,
  senderSecret: process.env.SENDER_SECRET,
  agentSecret: process.env.AGENT_SECRET,
  oracleAddress: process.env.ORACLE_ADDRESS,
};

// === Helper Functions ===

function toStroops(amount) {
  return BigInt(Math.floor(amount * 1e7));
}

function getKeypair(secret, name) {
  if (!secret) {
    throw new Error(`${name} secret not configured. Set ${name.toUpperCase()}_SECRET in .env`);
  }
  return StellarSdk.Keypair.fromSecret(secret);
}

function buildTx(source, method, args) {
  const contract = new StellarSdk.Contract(config.contractId);
  return new StellarSdk.TransactionBuilder(source, {
    fee: '100',
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(30)
    .build();
}

async function invoke(sourceKeypair, method, args) {
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const account = await server.getAccount(sourceKeypair.publicKey());
  const tx = buildTx(account, method, args);
  const prepared = await server.prepareTransaction(tx);
  prepared.sign(sourceKeypair);
  const response = await server.sendTransaction(prepared);

  if (response.status === 'pending') {
    let txResponse = await server.getTransaction(response.hash);
    while (txResponse.status === 'not_found') {
      await new Promise((r) => setTimeout(r, 1000));
      txResponse = await server.getTransaction(response.hash);
    }
    return txResponse;
  }
  return response;
}

// === Proof Validation Example ===

async function main() {
  if (!config.contractId) {
    throw new Error('SWIFTREMIT_CONTRACT_ID not set in .env');
  }

  const adminKeypair = config.adminSecret ? getKeypair(config.adminSecret, 'admin') : null;
  const senderKeypair = getKeypair(config.senderSecret, 'sender');
  const agentKeypair = getKeypair(config.agentSecret, 'agent');

  const senderAddress = senderKeypair.publicKey();
  const agentAddress = agentKeypair.publicKey();
  const oracleAddress = config.oracleAddress;

  console.log('=== SwiftRemit Proof Validation Example ===\n');

  // Step 1: Create a remittance with proof validation required
  console.log('Step 1: Creating remittance with proof validation...');

  const settlementConfig = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol('require_proof'),
      val: xdr.ScVal.scvBool(true),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol('oracle_address'),
      val: new StellarSdk.Address(oracleAddress).toScVal(),
    }),
  ]);

  const amount = toStroops(100); // 100 USDC

  const createResult = await invoke(senderKeypair, 'create_remittance', [
    new StellarSdk.Address(senderAddress).toScVal(),
    new StellarSdk.Address(agentAddress).toScVal(),
    StellarSdk.xdr.ScVal.scvI128(
      new StellarSdk.xdr.Int128Parts({ hi: 0n, lo: amount })
    ),
    xdr.ScVal.scvVoid(), // expiry (None)
    xdr.ScVal.scvVoid(), // token (None = default USDC)
    xdr.ScVal.scvVoid(), // idempotency_key
    settlementConfig,
    xdr.ScVal.scvVoid(), // recipient_hash
    xdr.ScVal.scvVoid(), // integrator
  ]);

  const remittanceId = createResult.returnValue;
  console.log(`  Remittance created with ID: ${remittanceId}`);

  // Step 2: Compute the proof using the contract's settlement hash query
  console.log('\nStep 2: Computing proof (settlement commitment)...');

  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const agentAccount = await server.getAccount(agentAddress);
  const hashTx = buildTx(agentAccount, 'compute_settlement_hash', [
    xdr.ScVal.scvUint64(remittanceId),
  ]);
  const hashResponse = await server.simulateTransaction(hashTx);
  const proof = hashResponse.returnValue;

  console.log(`  Proof computed: ${proof}`);

  // Step 3: Confirm payout with the valid proof
  console.log('\nStep 3: Confirming payout with valid proof...');

  try {
    await invoke(agentKeypair, 'confirm_payout', [
      new StellarSdk.Address(agentAddress).toScVal(),
      xdr.ScVal.scvUint64(remittanceId),
      proof,
      xdr.ScVal.scvVoid(), // recipient_details_hash (None)
    ]);
    console.log('  Payout confirmed successfully!');
  } catch (error) {
    if (String(error).includes('InvalidProof')) {
      console.error('  ERROR: Invalid proof — does not match expected commitment');
    } else if (String(error).includes('MissingProof')) {
      console.error('  ERROR: Missing proof — require_proof is true but no proof supplied');
    } else if (String(error).includes('DuplicateSettlement')) {
      console.error('  ERROR: Duplicate settlement — this remittance was already settled');
    } else {
      console.error('  ERROR:', error.message || error);
    }
    throw error;
  }

  // Step 4: Demonstrate error handling for invalid proof
  console.log('\nStep 4: Demonstrating invalid proof rejection...');

  // Create a second remittance
  const createResult2 = await invoke(senderKeypair, 'create_remittance', [
    new StellarSdk.Address(senderAddress).toScVal(),
    new StellarSdk.Address(agentAddress).toScVal(),
    StellarSdk.xdr.ScVal.scvI128(
      new StellarSdk.xdr.Int128Parts({ hi: 0n, lo: amount })
    ),
    xdr.ScVal.scvVoid(),
    xdr.ScVal.scvVoid(),
    xdr.ScVal.scvVoid(),
    settlementConfig,
    xdr.ScVal.scvVoid(),
    xdr.ScVal.scvVoid(),
  ]);

  const remittanceId2 = createResult2.returnValue;
  console.log(`  Second remittance created with ID: ${remittanceId2}`);

  // Try to confirm with an invalid proof (all zeros)
  const invalidProof = xdr.ScVal.scvBytes(Buffer.alloc(32, 0));

  try {
    await invoke(agentKeypair, 'confirm_payout', [
      new StellarSdk.Address(agentAddress).toScVal(),
      xdr.ScVal.scvUint64(remittanceId2),
      invalidProof,
      xdr.ScVal.scvVoid(),
    ]);
    console.error('  ERROR: Invalid proof was accepted (should have been rejected)');
  } catch (error) {
    if (String(error).includes('InvalidProof')) {
      console.log('  Invalid proof correctly rejected with InvalidProof error');
    } else {
      console.log('  Rejected with error:', error.message || error);
    }
  }

  console.log('\n=== Example Complete ===');
}

main().catch((error) => {
  console.error('\nExample failed:', error.message || error);
  process.exit(1);
});
