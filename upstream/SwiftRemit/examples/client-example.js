/**
 * SwiftRemit Client Example
 * 
 * This example demonstrates how to interact with the SwiftRemit smart contract
 * from an external JavaScript/TypeScript client using the Stellar SDK.
 * 
 * Prerequisites:
 * - Node.js 18+
 * - npm or yarn
 * - Stellar account with testnet/friendbot funds
 * - Contract deployed to testnet
 * 
 * Installation:
 * npm install @stellar/stellar-sdk @stellar/freighter-api
 * 
 * Usage:
 * node client-example.js
 */

// === Configuration ===
const config = require('./config');
const StellarSdk = require('@stellar/stellar-sdk');

// Create keypairs from secrets if provided
const adminKeypair = config.adminSecret ? StellarSdk.Keypair.fromSecret(config.adminSecret) : null;
const senderKeypair = config.senderSecret ? StellarSdk.Keypair.fromSecret(config.senderSecret) : null;
const agentKeypair = config.agentSecret ? StellarSdk.Keypair.fromSecret(config.agentSecret) : null;

// === Helper Functions ===

/**
 * Convert amount to stroops (smallest unit)
 */
function toStroops(amount) {
  return BigInt(Math.floor(amount * config.usdcMultiplier));
}

/**
 * Convert stroops to amount
 */
function fromStroops(stroops) {
  return Number(stroops) / config.usdcMultiplier;
}

/**
 * Build a Soroban transaction
 */
async function buildSorobanTransaction(source, contractId, method, args = []) {
  const contract = new StellarSdk.Contract(contractId);
  
  const transaction = new StellarSdk.TransactionBuilder(source, {
    fee: config.transactionFee,
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(config.transactionTimeout)
    .build();
  
  return transaction;
}

/**
 * Sign and simulate a Soroban transaction
 */
async function simulateTransaction(transaction, sourceKeypair) {
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  
  // Prepare the transaction
  const preparedTx = await server.prepareTransaction(transaction);
  
  // Sign with the source account
  preparedTx.sign(sourceKeypair);
  
  // Send to network
  const response = await server.sendTransaction(preparedTx);
  
  console.log('Transaction response:', response);
  
  // Wait for status
  if (response.status === 'pending') {
    let txResponse = await server.getTransaction(response.hash);
    while (txResponse.status === 'not_found') {
      await new Promise(resolve => setTimeout(resolve, config.pollIntervalMs));
      txResponse = await server.getTransaction(response.hash);
    }
    
    if (txResponse.status === 'success') {
      console.log('Transaction successful!');
      return txResponse.returnValue;
    } else {
      console.error('Transaction failed:', txResponse);
      throw new Error('Transaction failed');
    }
  }
  
  return response;
}

/**
 * Build and invoke a contract method
 */
async function invokeContract(sourceKeypair, contractId, method, args = []) {
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  
  // Get account
  const account = await server.getAccount(sourceKeypair.publicKey());
  
  // Build transaction
  const transaction = await buildSorobanTransaction(account, contractId, method, args);
  
  // Prepare, sign, and send
  const preparedTx = await server.prepareTransaction(transaction);
  preparedTx.sign(sourceKeypair);
  
  const response = await server.sendTransaction(preparedTx);
  
  // Poll for result
  if (response.status === 'pending') {
    let txResponse = await server.getTransaction(response.hash);
    while (txResponse.status === 'not_found') {
      await new Promise(resolve => setTimeout(resolve, config.pollIntervalMs));
      txResponse = await server.getTransaction(response.hash);
    }
    
    return txResponse;
  }
  
  return response;
}

// === Contract Interaction Functions ===

/**
 * Initialize the SwiftRemit contract
 * This should be called once by the admin
 */
async function initializeContract() {
  console.log('\n=== Initializing Contract ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const admin = adminKeypair.publicKey();
  const feeBps = config.defaultFeeBps;
  
  // The initialize function takes:
  // - admin: Address
  // - usdc_token: Address
  // - fee_bps: u32
  
  const args = [
    new StellarSdk.Address(admin).toScVal(),
    new StellarSdk.Address(config.usdcTokenId).toScVal(),
    StellarSdk.xdr.ScVal.scvU32(feeBps),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    config.contractId,
    'initialize',
    args
  );
  
  console.log('Initialize response:', response);
  console.log('✅ Contract initialized with fee_bps:', feeBps);
  
  return response;
}

/**
 * Register an agent who can receive payouts
 */
async function registerAgent(agentAddress) {
  console.log('\n=== Registering Agent ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  // The register_agent function takes:
  // - agent: Address
  
  const args = [
    new StellarSdk.Address(agentAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    config.contractId,
    'register_agent',
    args
  );
  
  console.log('Register agent response:', response);
  console.log('✅ Agent registered:', agentAddress);
  
  return response;
}

/**
 * Remove an agent from the approved list
 */
async function removeAgent(agentAddress) {
  console.log('\n=== Removing Agent ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const args = [
    new StellarSdk.Address(agentAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    config.contractId,
    'remove_agent',
    args
  );
  
  console.log('Remove agent response:', response);
  console.log('✅ Agent removed:', agentAddress);
  
  return response;
}

/**
 * Update the platform fee
 */
async function updateFee(feeBps) {
  console.log('\n=== Updating Platform Fee ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const args = [
    StellarSdk.xdr.ScVal.scvU32(feeBps),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    config.contractId,
    'update_fee',
    args
  );
  
  console.log('Update fee response:', response);
  console.log('✅ Fee updated to:', feeBps, 'bps (', feeBps / 100, '%)');
  
  return response;
}

/**
 * Create a new remittance
 * This is called by the sender who wants to send money
 */
async function createRemittance(senderKeypair, agentAddress, amount) {
  console.log('\n=== Creating Remittance ===');
  
  const sender = senderKeypair.publicKey();
  const amountStroops = toStroops(amount);
  
  // First, sender needs to approve the contract to spend their USDC
  // This requires a token approval operation (not shown here)
  // For testing, you can use the Stellar laboratory or set up a mock
  
  // The create_remittance function takes:
  // - sender: Address
  // - agent: Address
  // - amount: i128
  // - expiry: Option<u64>
  
  const args = [
    new StellarSdk.Address(sender).toScVal(),
    new StellarSdk.Address(agentAddress).toScVal(),
    StellarSdk.xdr.ScVal.scvI128(amountStroops),
    StellarSdk.xdr.ScVal.scvVoid(), // No expiry
  ];
  
  const response = await invokeContract(
    senderKeypair,
    config.contractId,
    'create_remittance',
    args
  );
  
  console.log('Create remittance response:', response);
  
  // Parse the returned remittance ID
  if (response.returnValue) {
    const remittanceId = StellarSdk.xdr.ScVal.fromScVal(response.returnValue).u64().low;
    console.log('✅ Remittance created with ID:', remittanceId);
    console.log('   Amount:', amount, 'USDC');
    console.log('   Agent:', agentAddress);
    return remittanceId;
  }
  
  return null;
}

/**
 * Confirm payout - called by agent after they've paid the recipient
 */
async function confirmPayout(agentKeypair, remittanceId) {
  console.log('\n=== Confirming Payout ===');
  
  // The confirm_payout function takes:
  // - remittance_id: u64
  
  const args = [
    StellarSdk.xdr.ScVal.scvU64(remittanceId),
  ];
  
  const response = await invokeContract(
    agentKeypair,
    config.contractId,
    'confirm_payout',
    args
  );
  
  console.log('Confirm payout response:', response);
  console.log('✅ Payout confirmed for remittance:', remittanceId);
  
  return response;
}

/**
 * Cancel a pending remittance - called by sender
 */
async function cancelRemittance(senderKeypair, remittanceId) {
  console.log('\n=== Cancelling Remittance ===');
  
  const args = [
    StellarSdk.xdr.ScVal.scvU64(remittanceId),
  ];
  
  const response = await invokeContract(
    senderKeypair,
    config.contractId,
    'cancel_remittance',
    args
  );
  
  console.log('Cancel remittance response:', response);
  console.log('✅ Remittance cancelled:', remittanceId);
  
  return response;
}

/**
 * Withdraw accumulated fees - called by admin
 */
async function withdrawFees(adminKeypair, recipientAddress) {
  console.log('\n=== Withdrawing Fees ===');
  
  const args = [
    new StellarSdk.Address(recipientAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    config.contractId,
    'withdraw_fees',
    args
  );
  
  console.log('Withdraw fees response:', response);
  console.log('✅ Fees withdrawn to:', recipientAddress);
  
  return response;
}

// === Query Functions (read-only) ===

/**
 * Get remittance details
 */
async function getRemittance(remittanceId) {
  console.log('\n=== Getting Remittance ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const contract = new StellarSdk.Contract(config.contractId);
  
  const args = [StellarSdk.xdr.ScVal.scvU64(remittanceId)];
  
  // Build a simulated call (no signature needed for reads)
  const account = await server.getAccount(adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: config.transactionFee,
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call('get_remittance', ...args))
    .setTimeout(config.transactionTimeout)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  console.log('Get remittance result:', result);
  
  if (result.results && result.results[0]) {
    const returnValue = result.results[0].returnValue;
    // Parse the Remittance struct
    // This would need custom parsing based on the struct definition
    console.log('✅ Remittance details retrieved');
    return returnValue;
  }
  
  return null;
}

/**
 * Get accumulated fees
 */
async function getAccumulatedFees() {
  console.log('\n=== Getting Accumulated Fees ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const contract = new StellarSdk.Contract(config.contractId);
  
  const args = [];
  
  const account = await server.getAccount(adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: config.transactionFee,
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call('get_accumulated_fees'))
    .setTimeout(config.transactionTimeout)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const fees = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).i128();
    const feesNum = Number(fees.low) / config.usdcMultiplier;
    console.log('Accumulated fees:', feesNum, 'USDC');
    return feesNum;
  }
  
  return 0;
}

/**
 * Check if agent is registered
 */
async function isAgentRegistered(agentAddress) {
  console.log('\n=== Checking Agent Registration ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const contract = new StellarSdk.Contract(config.contractId);
  
  const args = [new StellarSdk.Address(agentAddress).toScVal()];
  
  const account = await server.getAccount(adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: config.transactionFee,
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call('is_agent_registered', ...args))
    .setTimeout(config.transactionTimeout)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const registered = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).bool();
    console.log('Agent registered:', registered);
    return registered;
  }
  
  return false;
}

/**
 * Get platform fee in basis points
 */
async function getPlatformFeeBps() {
  console.log('\n=== Getting Platform Fee ===');
  
  if (!adminKeypair) {
    throw new Error('Admin keypair not configured. Set ADMIN_SECRET in .env');
  }
  
  const server = new StellarSdk.SorobanRpc.Server(config.rpcUrl);
  const contract = new StellarSdk.Contract(config.contractId);
  
  const account = await server.getAccount(adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: config.transactionFee,
    networkPassphrase: config.networkPassphrase,
  })
    .addOperation(contract.call('get_platform_fee_bps'))
    .setTimeout(config.transactionTimeout)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const feeBps = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).u32();
    console.log('Platform fee:', feeBps, 'bps (', feeBps / 100, '%)');
    return feeBps;
  }
  
  return 0;
}

// === Main Execution Flow ===

async function main() {
  console.log('╔════════════════════════════════════════╗');
  console.log('║     SwiftRemit Client Example          ║');
  console.log('╚════════════════════════════════════════╝');
  
  console.log('\n📋 Configuration:');
  console.log('   Network:', config.network);
  console.log('   Contract ID:', config.contractId);
  console.log('   USDC Token:', config.usdcTokenId);
  
  if (!adminKeypair || !senderKeypair || !agentKeypair) {
    console.error('\n❌ Error: Missing required keypairs');
    console.error('   Please set ADMIN_SECRET, SENDER_SECRET, and AGENT_SECRET in .env');
    process.exit(1);
  }
  
  console.log('   Admin:', adminKeypair.publicKey().slice(0, 8) + '...');
  console.log('   Sender:', senderKeypair.publicKey().slice(0, 8) + '...');
  console.log('   Agent:', agentKeypair.publicKey().slice(0, 8) + '...');
  
  try {
    // === Step 1: Initialize Contract (run once) ===
    // await initializeContract();
    
    // === Step 2: Register Agent ===
    const agentAddress = agentKeypair.publicKey();
    await registerAgent(agentAddress);
    
    // === Step 3: Check Agent Registration ===
    const isRegistered = await isAgentRegistered(agentAddress);
    console.log('   Agent is registered:', isRegistered);
    
    // === Step 4: Get Platform Fee ===
    const feeBps = await getPlatformFeeBps();
    console.log('   Current fee:', feeBps, 'bps');
    
    // === Step 5: Create Remittance ===
    const amountToSend = 100; // 100 USDC
    const remittanceId = await createRemittance(
      senderKeypair,
      agentAddress,
      amountToSend
    );
    
    // === Step 6: Query Remittance ===
    await getRemittance(remittanceId);
    
    // === Step 7: Confirm Payout (Agent) ===
    await confirmPayout(agentKeypair, remittanceId);
    
    // === Step 8: Check Accumulated Fees ===
    const accumulatedFees = await getAccumulatedFees();
    console.log('   Total accumulated fees:', accumulatedFees, 'USDC');
    
    // === Step 9: Withdraw Fees (Admin) ===
    // Uncomment to withdraw fees:
    // await withdrawFees(adminKeypair, adminKeypair.publicKey());
    
    // === Step 10: Check Fees After Withdrawal ===
    const feesAfter = await getAccumulatedFees();
    console.log('   Fees after withdrawal:', feesAfter, 'USDC');
    
    console.log('\n╔════════════════════════════════════════╗');
    console.log('║     Example Completed Successfully!    ║');
    console.log('╚════════════════════════════════════════╝');
    
  } catch (error) {
    console.error('\n❌ Error:', error.message);
    console.error(error.stack);
    process.exit(1);
  }
}

// Export functions for use in other modules
module.exports = {
  config,
  adminKeypair,
  senderKeypair,
  agentKeypair,
  initializeContract,
  registerAgent,
  removeAgent,
  updateFee,
  createRemittance,
  confirmPayout,
  cancelRemittance,
  withdrawFees,
  getRemittance,
  getAccumulatedFees,
  isAgentRegistered,
  getPlatformFeeBps,
  toStroops,
  fromStroops,
};

// Run if executed directly
if (require.main === module) {
  main();
}
