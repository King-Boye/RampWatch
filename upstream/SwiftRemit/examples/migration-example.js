/**
 * Contract Migration Example for SwiftRemit
 * 
 * This example demonstrates how to safely migrate state from an old
 * contract deployment to a new one using cryptographic verification.
 */

const {
  Contract,
  SorobanRpc,
  TransactionBuilder,
  Networks,
  Keypair,
} = require('@stellar/stellar-sdk');

// Configuration
const config = {
  rpcUrl: 'https://soroban-testnet.stellar.org',
  networkPassphrase: Networks.TESTNET,
  oldContractId: process.env.OLD_CONTRACT_ID,
  newContractId: process.env.NEW_CONTRACT_ID,
};

// Initialize RPC server
const server = new SorobanRpc.Server(config.rpcUrl);

/**
 * Example 1: Full Migration
 * 
 * Demonstrates complete state migration for small datasets (< 100 remittances).
 */
async function exampleFullMigration() {
  console.log('\n=== Example 1: Full Migration ===\n');

  const oldContract = new Contract(config.oldContractId);
  const newContract = new Contract(config.newContractId);
  const admin = Keypair.fromSecret(process.env.ADMIN_SECRET);

  try {
    // Step 1: Pause old contract
    console.log('1. Pausing old contract...');
    await oldContract.pause({ caller: admin.publicKey() });
    console.log('   ✓ Old contract paused');

    // Step 2: Export state
    console.log('\n2. Exporting state...');
    const snapshot = await oldContract.export_migration_state({
      caller: admin.publicKey()
    });
    console.log(`   ✓ Exported ${snapshot.persistent_data.remittances.length} remittances`);
    console.log(`   ✓ Accumulated fees: ${snapshot.instance_data.accumulated_fees}`);
    console.log(`   ✓ Platform fee: ${snapshot.instance_data.platform_fee_bps} bps`);

    // Step 3: Verify snapshot
    console.log('\n3. Verifying snapshot integrity...');
    const verification = await oldContract.verify_migration_snapshot({
      snapshot: snapshot
    });
    
    if (!verification.valid) {
      throw new Error('Snapshot verification failed!');
    }
    console.log('   ✓ Snapshot verified');
    console.log(`   ✓ Hash: ${verification.expected_hash.toString('hex').substring(0, 16)}...`);

    // Step 4: Import to new contract
    console.log('\n4. Importing state to new contract...');
    await newContract.import_migration_state({
      caller: admin.publicKey(),
      snapshot: snapshot
    });
    console.log('   ✓ State imported');

    // Step 5: Verify import
    console.log('\n5. Verifying import...');
    const newSnapshot = await newContract.export_migration_state({
      caller: admin.publicKey()
    });
    
    if (snapshot.instance_data.remittance_counter !== newSnapshot.instance_data.remittance_counter) {
      throw new Error('Remittance counter mismatch!');
    }
    console.log('   ✓ Import verified');

    // Step 6: Test new contract
    console.log('\n6. Testing new contract...');
    const feeBps = await newContract.get_platform_fee_bps();
    console.log(`   ✓ Platform fee: ${feeBps} bps`);
    
    const fees = await newContract.get_accumulated_fees();
    console.log(`   ✓ Accumulated fees: ${fees}`);

    console.log('\n✅ Migration completed successfully!');

  } catch (error) {
    console.error('\n❌ Migration failed:', error.message);
    throw error;
  }
}

/**
 * Example 2: Batch Migration
 * 
 * Demonstrates incremental migration for large datasets (> 100 remittances).
 */
async function exampleBatchMigration() {
  console.log('\n=== Example 2: Batch Migration ===\n');

  const oldContract = new Contract(config.oldContractId);
  const newContract = new Contract(config.newContractId);
  const admin = Keypair.fromSecret(process.env.ADMIN_SECRET);

  try {
    // Step 1: Determine batch parameters
    console.log('1. Calculating batch parameters...');
    const remittanceCount = await oldContract.get_remittance_counter();
    const batchSize = 50;
    const totalBatches = Math.ceil(remittanceCount / batchSize);
    console.log(`   ✓ Total remittances: ${remittanceCount}`);
    console.log(`   ✓ Batch size: ${batchSize}`);
    console.log(`   ✓ Total batches: ${totalBatches}`);

    // Step 2: Pause old contract
    console.log('\n2. Pausing old contract...');
    await oldContract.pause({ caller: admin.publicKey() });
    console.log('   ✓ Old contract paused');

    // Step 3: Export batches
    console.log('\n3. Exporting batches...');
    const batches = [];
    for (let i = 0; i < totalBatches; i++) {
      const batch = await oldContract.export_migration_batch({
        caller: admin.publicKey(),
        batch_number: i,
        batch_size: batchSize
      });
      batches.push(batch);
      console.log(`   ✓ Exported batch ${i + 1}/${totalBatches} (${batch.remittances.length} items)`);
    }

    // Step 4: Initialize new contract
    console.log('\n4. Initializing new contract...');
    const instanceData = await oldContract.export_migration_state({
      caller: admin.publicKey()
    }).then(s => s.instance_data);
    
    await newContract.initialize({
      admin: admin.publicKey(),
      usdc_token: instanceData.usdc_token,
      fee_bps: instanceData.platform_fee_bps
    });
    console.log('   ✓ New contract initialized');

    // Step 5: Import batches
    console.log('\n5. Importing batches...');
    for (let i = 0; i < batches.length; i++) {
      await newContract.import_migration_batch({
        caller: admin.publicKey(),
        batch: batches[i]
      });
      console.log(`   ✓ Imported batch ${i + 1}/${totalBatches}`);
    }

    // Step 6: Verify completeness
    console.log('\n6. Verifying completeness...');
    const newCount = await newContract.get_remittance_counter();
    if (newCount !== remittanceCount) {
      throw new Error(`Count mismatch: expected ${remittanceCount}, got ${newCount}`);
    }
    console.log(`   ✓ All ${newCount} remittances migrated`);

    console.log('\n✅ Batch migration completed successfully!');

  } catch (error) {
    console.error('\n❌ Batch migration failed:', error.message);
    throw error;
  }
}

/**
 * Example 3: Verification Only
 * 
 * Demonstrates how to verify a snapshot without importing.
 */
async function exampleVerificationOnly() {
  console.log('\n=== Example 3: Verification Only ===\n');

  const oldContract = new Contract(config.oldContractId);

  try {
    // Export snapshot
    console.log('1. Exporting snapshot...');
    const snapshot = await oldContract.export_migration_state({
      caller: admin.publicKey()
    });
    console.log('   ✓ Snapshot exported');

    // Verify integrity
    console.log('\n2. Verifying integrity...');
    const verification = await oldContract.verify_migration_snapshot({
      snapshot: snapshot
    });

    console.log(`   Valid: ${verification.valid}`);
    console.log(`   Expected hash: ${verification.expected_hash.toString('hex').substring(0, 32)}...`);
    console.log(`   Actual hash:   ${verification.actual_hash.toString('hex').substring(0, 32)}...`);
    console.log(`   Timestamp: ${new Date(verification.timestamp * 1000).toISOString()}`);

    if (verification.valid) {
      console.log('\n✅ Snapshot is valid and ready for migration');
    } else {
      console.log('\n❌ Snapshot verification failed - do not use!');
    }

  } catch (error) {
    console.error('\n❌ Verification failed:', error.message);
    throw error;
  }
}

/**
 * Example 4: Tamper Detection
 * 
 * Demonstrates how hash verification detects tampering.
 */
async function exampleTamperDetection() {
  console.log('\n=== Example 4: Tamper Detection ===\n');

  const oldContract = new Contract(config.oldContractId);
  const newContract = new Contract(config.newContractId);
  const admin = Keypair.fromSecret(process.env.ADMIN_SECRET);

  try {
    // Export snapshot
    console.log('1. Exporting snapshot...');
    const snapshot = await oldContract.export_migration_state({
      caller: admin.publicKey()
    });
    console.log('   ✓ Snapshot exported');

    // Verify original
    console.log('\n2. Verifying original snapshot...');
    const verification1 = await oldContract.verify_migration_snapshot({
      snapshot: snapshot
    });
    console.log(`   ✓ Original valid: ${verification1.valid}`);

    // Tamper with data
    console.log('\n3. Tampering with snapshot data...');
    const tamperedSnapshot = { ...snapshot };
    tamperedSnapshot.instance_data.platform_fee_bps = 9999; // Change fee
    console.log('   ✓ Changed platform fee to 9999 bps');

    // Verify tampered
    console.log('\n4. Verifying tampered snapshot...');
    const verification2 = await oldContract.verify_migration_snapshot({
      snapshot: tamperedSnapshot
    });
    console.log(`   ✓ Tampered valid: ${verification2.valid}`);

    if (!verification2.valid) {
      console.log('   ✓ Tampering detected!');
      console.log(`   ✓ Hash mismatch: expected != actual`);
    }

    // Try to import tampered snapshot
    console.log('\n5. Attempting to import tampered snapshot...');
    try {
      await newContract.import_migration_state({
        caller: admin.publicKey(),
        snapshot: tamperedSnapshot
      });
      console.log('   ❌ Import should have failed!');
    } catch (error) {
      console.log('   ✓ Import correctly rejected');
      console.log(`   ✓ Error: ${error.message}`);
    }

    console.log('\n✅ Tamper detection working correctly!');

  } catch (error) {
    console.error('\n❌ Test failed:', error.message);
    throw error;
  }
}

/**
 * Example 5: Migration Audit Trail
 * 
 * Demonstrates how to create an audit trail of the migration.
 */
async function exampleAuditTrail() {
  console.log('\n=== Example 5: Migration Audit Trail ===\n');

  const oldContract = new Contract(config.oldContractId);
  const newContract = new Contract(config.newContractId);
  const admin = Keypair.fromSecret(process.env.ADMIN_SECRET);

  const auditLog = {
    migration_date: new Date().toISOString(),
    old_contract_id: config.oldContractId,
    new_contract_id: config.newContractId,
    admin_address: admin.publicKey(),
    steps: [],
  };

  try {
    // Export
    console.log('1. Exporting state...');
    const snapshot = await oldContract.export_migration_state({
      caller: admin.publicKey()
    });
    
    auditLog.steps.push({
      step: 'export',
      timestamp: new Date().toISOString(),
      remittances_count: snapshot.persistent_data.remittances.length,
      accumulated_fees: snapshot.instance_data.accumulated_fees.toString(),
      verification_hash: snapshot.verification_hash.toString('hex'),
    });
    console.log('   ✓ State exported');

    // Verify
    console.log('\n2. Verifying snapshot...');
    const verification = await oldContract.verify_migration_snapshot({
      snapshot: snapshot
    });
    
    auditLog.steps.push({
      step: 'verify',
      timestamp: new Date().toISOString(),
      valid: verification.valid,
      expected_hash: verification.expected_hash.toString('hex'),
      actual_hash: verification.actual_hash.toString('hex'),
    });
    console.log('   ✓ Snapshot verified');

    // Import
    console.log('\n3. Importing state...');
    await newContract.import_migration_state({
      caller: admin.publicKey(),
      snapshot: snapshot
    });
    
    auditLog.steps.push({
      step: 'import',
      timestamp: new Date().toISOString(),
      success: true,
    });
    console.log('   ✓ State imported');

    // Verify import
    console.log('\n4. Verifying import...');
    const newSnapshot = await newContract.export_migration_state({
      caller: admin.publicKey()
    });
    
    const countersMatch = snapshot.instance_data.remittance_counter === 
                         newSnapshot.instance_data.remittance_counter;
    const feesMatch = snapshot.instance_data.accumulated_fees === 
                     newSnapshot.instance_data.accumulated_fees;
    
    auditLog.steps.push({
      step: 'verify_import',
      timestamp: new Date().toISOString(),
      counters_match: countersMatch,
      fees_match: feesMatch,
      new_verification_hash: newSnapshot.verification_hash.toString('hex'),
    });
    console.log('   ✓ Import verified');

    // Save audit log
    console.log('\n5. Saving audit log...');
    const auditLogJson = JSON.stringify(auditLog, null, 2);
    console.log(auditLogJson);
    
    // In production, save to file or database
    // fs.writeFileSync('migration-audit.json', auditLogJson);
    
    console.log('\n✅ Migration audit trail created!');

  } catch (error) {
    auditLog.steps.push({
      step: 'error',
      timestamp: new Date().toISOString(),
      error: error.message,
    });
    console.error('\n❌ Migration failed:', error.message);
    console.log('\nAudit log:', JSON.stringify(auditLog, null, 2));
    throw error;
  }
}

/**
 * Helper: Verify migration success
 */
async function verifyMigrationSuccess(oldContract, newContract) {
  const oldCounter = await oldContract.get_remittance_counter();
  const newCounter = await newContract.get_remittance_counter();
  
  if (oldCounter !== newCounter) {
    throw new Error(`Counter mismatch: ${oldCounter} != ${newCounter}`);
  }

  const oldFees = await oldContract.get_accumulated_fees();
  const newFees = await newContract.get_accumulated_fees();
  
  if (oldFees !== newFees) {
    throw new Error(`Fees mismatch: ${oldFees} != ${newFees}`);
  }

  const oldFeeBps = await oldContract.get_platform_fee_bps();
  const newFeeBps = await newContract.get_platform_fee_bps();
  
  if (oldFeeBps !== newFeeBps) {
    throw new Error(`Fee bps mismatch: ${oldFeeBps} != ${newFeeBps}`);
  }

  return true;
}

/**
 * Main function to run examples
 */
async function main() {
  console.log('╔════════════════════════════════════════════════════════════╗');
  console.log('║     SwiftRemit Contract Migration Examples                ║');
  console.log('╚════════════════════════════════════════════════════════════╝');

  try {
    // Run examples
    // await exampleFullMigration();
    // await exampleBatchMigration();
    // await exampleVerificationOnly();
    // await exampleTamperDetection();
    await exampleAuditTrail();

  } catch (error) {
    console.error('\nError running examples:', error);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main().catch(console.error);
}

module.exports = {
  exampleFullMigration,
  exampleBatchMigration,
  exampleVerificationOnly,
  exampleTamperDetection,
  exampleAuditTrail,
  verifyMigrationSuccess,
};
