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
  
  console.log('Transaction response:', response);
  
  // Wait for status
  if (response.status === 'pending') {
    let txResponse = await server.getTransaction(response.hash);
    while (txResponse.status === 'not_found') {
      await new Promise(resolve => setTimeout(resolve, 1000));
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
  console.log('\n=== Initializing Contract ===');
  
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
  
  console.log('Initialize response:', response);
  console.log('✅ Contract initialized with fee_bps:', feeBps);
  
  return response;
}

/**
 * Register an agent who can receive payouts
 */
async function registerAgent(agentAddress) {
  console.log('\n=== Registering Agent ===');
  
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
  
  console.log('Register agent response:', response);
  console.log('✅ Agent registered:', agentAddress);
  
  return response;
}

/**
 * Remove an agent from the approved list
 */
async function removeAgent(agentAddress) {
  console.log('\n=== Removing Agent ===');
  
  const args = [
    new StellarSdk.Address(agentAddress).toScVal(),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
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
  
  const args = [
    StellarSdk.xdr.ScVal.scvU32(feeBps),
  ];
  
  const response = await invokeContract(
    CONFIG.adminKeypair,
    CONFIG.contractId,
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
    CONFIG.contractId,
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
    CONFIG.contractId,
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
    CONFIG.contractId,
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
    CONFIG.contractId,
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
    const feesNum = Number(fees.low) / USDC_MULTIPLIER;
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
  console.log('   Network:', CONFIG.network);
  console.log('   Contract ID:', CONFIG.contractId);
  console.log('   USDC Token:', CONFIG.usdcTokenId);
  console.log('   Admin:', CONFIG.adminKeypair.publicKey().slice(0, 8) + '...');
  console.log('   Sender:', CONFIG.senderKeypair.publicKey().slice(0, 8) + '...');
  console.log('   Agent:', CONFIG.agentKeypair.publicKey().slice(0, 8) + '...');
  
  try {
    // === Step 1: Initialize Contract (run once) ===
    // await initializeContract();
    
    // === Step 2: Register Agent ===
    const agentAddress = CONFIG.agentKeypair.publicKey();
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
      CONFIG.senderKeypair,
      agentAddress,
      amountToSend
    );
    
    // === Step 6: Query Remittance ===
    await getRemittance(remittanceId);
    
    // === Step 7: Confirm Payout (Agent) ===
    await confirmPayout(CONFIG.agentKeypair, remittanceId);
    
    // === Step 8: Check Accumulated Fees ===
    const accumulatedFees = await getAccumulatedFees();
    console.log('   Total accumulated fees:', accumulatedFees, 'USDC');
    
    // === Step 9: Withdraw Fees (Admin) ===
    // Uncomment to withdraw fees:
    // await withdrawFees(CONFIG.adminKeypair, CONFIG.adminKeypair.publicKey());
    
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
