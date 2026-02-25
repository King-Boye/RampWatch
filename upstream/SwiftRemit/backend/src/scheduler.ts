import cron from 'node-cron';
import { AssetVerifier } from './verifier';
import { getStaleAssets, saveAssetVerification } from './database';
import { storeVerificationOnChain } from './stellar';

const verifier = new AssetVerifier();

export function startBackgroundJobs() {
  // Run every 6 hours
  cron.schedule('0 */6 * * *', async () => {
    console.log('Starting periodic asset revalidation...');
    await revalidateStaleAssets();
  });

  console.log('Background jobs scheduled');
}

async function revalidateStaleAssets() {
  try {
    const hoursOld = parseInt(process.env.VERIFICATION_INTERVAL_HOURS || '24');
    const staleAssets = await getStaleAssets(hoursOld);

    console.log(`Found ${staleAssets.length} assets to revalidate`);

    for (const asset of staleAssets) {
      try {
        console.log(`Revalidating ${asset.asset_code}-${asset.issuer}`);

        const result = await verifier.verifyAsset(asset.asset_code, asset.issuer);

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
          community_reports: asset.community_reports || 0,
        };

        await saveAssetVerification(verification);

        // Store on-chain
        try {
          await storeVerificationOnChain(verification);
        } catch (error) {
          console.error(`Failed to store on-chain for ${asset.asset_code}:`, error);
        }

        // Rate limiting - wait 1 second between verifications
        await new Promise(resolve => setTimeout(resolve, 1000));
      } catch (error) {
        console.error(`Failed to revalidate ${asset.asset_code}:`, error);
      }
    }

    console.log('Periodic revalidation completed');
  } catch (error) {
    console.error('Error in revalidation job:', error);
  }
}
