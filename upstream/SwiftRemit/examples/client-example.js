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

const { createLogger } = require('./logger');
const logger = createLogger('client-example');

// === Configuration ===
const CONFIG = {
  // Network configuration
  network: 'testnet', // Use 'testnet' or 'mainnet'
  networkPassphrase: StellarSdk.Networks.TESTNET,
  rpcUrl: 'https://soroban-testnet.stellar.org:443',
  
  // Contract addresses (replace with your deployed addresses)
  contractId: process.env.SWIFTREMIT_CONTRACT_ID || 'CD4YWKVCM3HPLWP6XR5OOCJOH6HGZUL6DUM3E5VUWWZCB5MZ7V7B3N4G',
  usdcTokenId: process.env.USDC_TOKEN_ID || 'CDOM7Z3LHSWDC5IHEYC5GM6NR6X7C5MVX7C5MVX7C5MVX7C5MVX7C5MV',
  
  // Account addresses (replace with your test accounts)
  adminKeypair: StellarSdk.Keypair.fromSecret(process.env.ADMIN_SECRET || 'SCZMWGZS4B4CS7GZDCGQTIS5U5MO6WVP2QRZWC9NJXMR7X7JH6Q6USHK'),
  senderKeypair: StellarSdk.Keypair.fromSecret(process.env.SENDER_SECRET || 'SAV76USXIJOBMEQFXANQ3JJV7JVHR3X3JK5KDHLFI2E2NA==='),
  agentKeypair: StellarSdk.Keypair.fromSecret(process.env.AGENT_SECRET || 'SASIE2DL22HS5NDYTR3F5VLMBE4NQXL5X7FDBNVNAAZG6XHHPXU2===='),
};

// USDC has 7 decimal places
const USDC_DECIMALS = 7;
const USDC_MULTIPLIER = Math.pow(10, USDC_DECIMALS);

// === Helper Functions ===

/**
 * Convert amount to stroops (smallest unit)
 */
function toStroops(amount) {
  return BigInt(Math.floor(amount * USDC_MULTIPLIER));
}

/**
 * Convert stroops to amount
 */
function fromStroops(stroops) {
  return Number(stroops) / USDC_MULTIPLIER;
}

/**
 * Build a Soroban transaction
 */
async function buildSorobanTransaction(source, contractId, method, args = []) {
  const contract = new StellarSdk.Contract(contractId);
  
  const transaction = new StellarSdk.TransactionBuilder(source, {
    fee: '100000',
    networkPassphrase: CONFIG.networkPassphrase,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(30)
    .build();
  
  return transaction;
}

/**
 * Sign and simulate a Soroban transaction
 */
async function simulateTransaction(transaction, sourceKeypair) {
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  
  // Prepare the transaction
  const preparedTx = await server.prepareTransaction(transaction);
  
  // Sign with the source account
  preparedTx.sign(sourceKeypair);
  
  // Send to network
  const response = await server.sendTransaction(preparedTx);
  
  logger.info({ response }, 'Transaction response');
  
  // Wait for status
  if (response.status === 'pending') {
    let txResponse = await server.getTransaction(response.hash);
    while (txResponse.status === 'not_found') {
      await new Promise(resolve => setTimeout(resolve, 1000));
      txResponse = await server.getTransaction(response.hash);
    }
    
    if (txResponse.status === 'success') {
      logger.info('Transaction successful!');
      return txResponse.returnValue;
    } else {
      logger.error({ txResponse }, 'Transaction failed');
      throw new Error('Transaction failed');
    }
  }
  
  return response;
}

/**
 * Build and invoke a contract method
 */
async function invokeContract(sourceKeypair, contractId, method, args = []) {
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  
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
      await new Promise(resolve => setTimeout(resolve, 1000));
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
  logger.info('=== Initializing Contract ===');
  
  const admin = CONFIG.adminKeypair.publicKey();
  const feeBps = 250; // 2.5% fee
  
  // The initialize function takes:
  // - admin: Address
  // - usdc_token: Address
  // - fee_bps: u32
  
  const args = [
    new StellarSdk.Address(admin).toScVal(),
    new StellarSdk.Address(CONFIG.usdcTokenId).toScVal(),
    StellarSdk.xdr.ScVal.scvU32(feeBps),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
    'initialize',
    args
  );
  
  logger.info({ response, feeBps }, 'Contract initialized');
  
  return response;
}

/**
 * Register an agent who can receive payouts
 */
async function registerAgent(agentAddress) {
  logger.info({ agentAddress }, '=== Registering Agent ===');
  
  // The register_agent function takes:
  // - agent: Address
  
  const args = [
    new StellarSdk.Address(agentAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
    'register_agent',
    args
  );
  
  logger.info({ response, agentAddress }, 'Agent registered');
  
  return response;
}

/**
 * Remove an agent from the approved list
 */
async function removeAgent(agentAddress) {
  logger.info({ agentAddress }, '=== Removing Agent ===');
  
  const args = [
    new StellarSdk.Address(agentAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
    'remove_agent',
    args
  );
  
  logger.info({ response, agentAddress }, 'Agent removed');
  
  return response;
}

/**
 * Update the platform fee
 */
async function updateFee(feeBps) {
  logger.info({ feeBps }, '=== Updating Platform Fee ===');
  
  const args = [
    StellarSdk.xdr.ScVal.scvU32(feeBps),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
    'update_fee',
    args
  );
  
  logger.info({ response, feeBps }, 'Fee updated');
  
  return response;
}

/**
 * Create a new remittance
 * This is called by the sender who wants to send money
 */
async function createRemittance(senderKeypair, agentAddress, amount) {
  logger.info({ agentAddress, amount }, '=== Creating Remittance ===');
  
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
    CONFIG.contractId,
    'create_remittance',
    args
  );
  
  logger.info({ response }, 'Create remittance response');
  
  // Parse the returned remittance ID
  if (response.returnValue) {
    logger.info({ remittanceId, amount, agentAddress }, 'Remittance created');
    return remittanceId;
  }
  
  return null;
}

/**
 * Confirm payout - called by agent after they've paid the recipient
 */
async function confirmPayout(agentKeypair, remittanceId) {
  logger.info({ remittanceId }, '=== Confirming Payout ===');
  
  // The confirm_payout function takes:
  // - remittance_id: u64
  
  const args = [
    StellarSdk.xdr.ScVal.scvU64(remittanceId),
  ];
  
  const response = await invokeContract(
    agentKeypair,
    CONFIG.contractId,
    'confirm_payout',
    args
  );
  
  logger.info({ response, remittanceId }, 'Payout confirmed');
  
  return response;
}

/**
 * Cancel a pending remittance - called by sender
 */
async function cancelRemittance(senderKeypair, remittanceId) {
  logger.info({ remittanceId }, '=== Cancelling Remittance ===');
  
  const args = [
    StellarSdk.xdr.ScVal.scvU64(remittanceId),
  ];
  
  const response = await invokeContract(
    senderKeypair,
    CONFIG.contractId,
    'cancel_remittance',
    args
  );
  
  logger.info({ response, remittanceId }, 'Remittance cancelled');
  
  return response;
}

/**
 * Withdraw accumulated fees - called by admin
 */
async function withdrawFees(adminKeypair, recipientAddress) {
  logger.info({ recipientAddress }, '=== Withdrawing Fees ===');
  
  const args = [
    new StellarSdk.Address(recipientAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    adminKeypair,
    CONFIG.contractId,
    'withdraw_fees',
    args
  );
  
  logger.info({ response, recipientAddress }, 'Fees withdrawn');
  
  return response;
}

// === Query Functions (read-only) ===

/**
 * Get remittance details
 */
async function getRemittance(remittanceId) {
  logger.info({ remittanceId }, '=== Getting Remittance ===');
  
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new StellarSdk.Contract(CONFIG.contractId);
  
  const args = [StellarSdk.xdr.ScVal.scvU64(remittanceId)];
  
  // Build a simulated call (no signature needed for reads)
  const account = await server.getAccount(CONFIG.adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: CONFIG.networkPassphrase,
  })
    .addOperation(contract.call('get_remittance', ...args))
    .setTimeout(30)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
    logger.info({ remittanceId }, 'Remittance details retrieved');
    return returnValue;
  
  return null;
}

/**
 * Get accumulated fees
 */
async function getAccumulatedFees() {
  logger.info('=== Getting Accumulated Fees ===');
  
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new StellarSdk.Contract(CONFIG.contractId);
  
  const args = [];
  
  const account = await server.getAccount(CONFIG.adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: CONFIG.networkPassphrase,
  })
    .addOperation(contract.call('get_accumulated_fees'))
    .setTimeout(30)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const fees = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).i128();
    logger.info({ feesNum }, 'Accumulated fees retrieved');
    return feesNum;
  }
  
  return 0;
}

/**
 * Check if agent is registered
 */
async function isAgentRegistered(agentAddress) {
  logger.info({ agentAddress }, '=== Checking Agent Registration ===');
  
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new StellarSdk.Contract(CONFIG.contractId);
  
  const args = [new StellarSdk.Address(agentAddress).toScVal()];
  
  const account = await server.getAccount(CONFIG.adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: CONFIG.networkPassphrase,
  })
    .addOperation(contract.call('is_agent_registered', ...args))
    .setTimeout(30)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const registered = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).bool();
    logger.info({ agentAddress, registered }, 'Agent registration status');
    return registered;
  }
  
  return false;
}

/**
 * Get platform fee in basis points
 */
async function getPlatformFeeBps() {
  logger.info('=== Getting Platform Fee ===');
  
  const server = new StellarSdk.SorobanRpc.Server(CONFIG.rpcUrl);
  const contract = new StellarSdk.Contract(CONFIG.contractId);
  
  const account = await server.getAccount(CONFIG.adminKeypair.publicKey());
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: CONFIG.networkPassphrase,
  })
    .addOperation(contract.call('get_platform_fee_bps'))
    .setTimeout(30)
    .build();
  
  const preparedTx = await server.prepareTransaction(tx);
  const result = await server.simulateTransaction(preparedTx);
  
  if (result.results && result.results[0]) {
    const feeBps = StellarSdk.xdr.ScVal.fromScVal(result.results[0].returnValue).u32();
    logger.info({ feeBps }, 'Platform fee retrieved');
    return feeBps;
  }
  
  return 0;
}

// === Main Execution Flow ===

async function main() {
  logger.info({
    network: CONFIG.network,
    contractId: CONFIG.contractId,
    usdcToken: CONFIG.usdcTokenId,
    admin: CONFIG.adminKeypair.publicKey().slice(0, 8) + '...',
    sender: CONFIG.senderKeypair.publicKey().slice(0, 8) + '...',
    agent: CONFIG.agentKeypair.publicKey().slice(0, 8) + '...',
  }, 'Configuration');
  
  try {
    // === Step 1: Initialize Contract (run once) ===
    // await initializeContract();
    
    // === Step 2: Register Agent ===
    const agentAddress = CONFIG.agentKeypair.publicKey();
    await registerAgent(agentAddress);
    
    logger.info({ isRegistered }, 'Agent registration check');
    
    // === Step 4: Get Platform Fee ===
    const feeBps = await getPlatformFeeBps();
    logger.info({ feeBps }, 'Current fee check');
    
    // === Step 5: Create Remittance ===
    const amountToSend = 100; // 100 USDC
    const remittanceId = await createRemittance(
      CONFIG.senderKeypair,
      agentAddress,
      amountToSend
    );
    
    // === Step 6: Query Remittance ===
    await getRemittance(remittanceId);
    
    // === Step 7: Confirm Payout (Agent) ===
    await confirmPayout(CONFIG.agentKeypair, remittanceId);
    
    logger.info({ accumulatedFees }, 'Total accumulated fees');
    
    // === Step 9: Withdraw Fees (Admin) ===
    // Uncomment to withdraw fees:
    // await withdrawFees(CONFIG.adminKeypair, CONFIG.adminKeypair.publicKey());
    
    logger.info({ feesAfter }, 'Fees after withdrawal');
    
    logger.info('=== Example Completed Successfully! ===');
    
  } catch (error) {
    logger.error({ error: error.message, stack: error.stack }, 'Execution failed');
    process.exit(1);
  }
}

// Export functions for use in other modules
module.exports = {
  CONFIG,
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
