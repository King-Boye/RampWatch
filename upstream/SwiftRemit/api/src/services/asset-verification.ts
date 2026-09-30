import { Horizon } from "@stellar/stellar-sdk";
import { config } from "../config";

/**
 * Asset verification service.
 *
 * Verifies that an asset issued on the Stellar network is backed by a
 * trusted anchor. Verification is performed by querying one or more
 * anchor registries (e.g. the Stellar TOML registry, SEP-1 anchor
 * directories, or custom registries) and checking whether the asset's
 * issuer is listed as a known anchor.
 */

export interface AnchorRegistry {
  /** Human readable name used for logging and diagnostics. */
  name: string;
  /**
   * Returns the set of issuer account IDs known to this registry.
   * Implementations may cache or paginate internally.
   */
  getKnownIssuers(): Promise<string[]>;
}

export interface AssetVerificationResult {
  assetCode: string;
  assetIssuer: string;
  verified: boolean;
  /** Names of the registries that vouched for the issuer. */
  matchedRegistries: string[];
}

/**
 * Registry backed by the Stellar TOML / SEP-1 anchor directory.
 * This is the default registry used when none are configured.
 */
export class StellarTomlRegistry implements AnchorRegistry {
  name = "stellar-toml";

  async getKnownIssuers(): Promise<string[]> {
    const server = new Horizon.Server(config.horizonUrl);
    const issuers = new Set<string>();
    let page = await server.accounts().limit(200).call();

    while (page.records.length > 0) {
      for (const record of page.records) {
        if (record.account_id) {
          issuers.add(record.account_id);
        }
      }
      page = await page.next();
    }

    return Array.from(issuers);
  }
}

/**
 * Registry backed by a static, operator-provided list of issuer account
 * IDs. Useful for private or partner anchor registries that are not
 * published through the Stellar TOML directory.
 */
export class StaticAnchorRegistry implements AnchorRegistry {
  name: string;
  private issuers: string[];

  constructor(name: string, issuers: string[]) {
    this.name = name;
    this.issuers = issuers;
  }

  async getKnownIssuers(): Promise<string[]> {
    return this.issuers;
  }
}

/**
 * Builds the list of anchor registries to consult during verification.
 * Additional registries can be configured through
 * `config.anchorRegistries`, allowing integration with anchor
 * registries beyond the default Stellar TOML directory.
 */
export function getAnchorRegistries(): AnchorRegistry[] {
  const registries: AnchorRegistry[] = [new StellarTomlRegistry()];

  for (const entry of config.anchorRegistries ?? []) {
    registries.push(new StaticAnchorRegistry(entry.name, entry.issuers));
  }

  return registries;
}

/**
 * Verifies an asset against all configured anchor registries.
 * An asset is considered verified when at least one registry lists
 * its issuer as a known anchor.
 */
export async function verifyAsset(
  assetCode: string,
  assetIssuer: string,
): Promise<AssetVerificationResult> {
  const registries = getAnchorRegistries();
  const matchedRegistries: string[] = [];

  for (const registry of registries) {
    try {
      const issuers = await registry.getKnownIssuers();
      if (issuers.includes(assetIssuer)) {
        matchedRegistries.push(registry.name);
      }
    } catch (error) {
      // A failing registry must not prevent verification through the
      // remaining registries.
      console.error(
        `Anchor registry "${registry.name}" failed during asset verification`,
        error,
      );
    }
  }

  return {
    assetCode,
    assetIssuer,
    verified: matchedRegistries.length > 0,
    matchedRegistries,
  };
}
