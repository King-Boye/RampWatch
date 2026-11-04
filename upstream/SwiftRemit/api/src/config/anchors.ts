import { Networks } from "@stellar/stellar-sdk";

export interface Anchor {
  name: string;
  homeDomain: string;
  network: Networks;
  /** Registry the anchor was sourced from (e.g. "sep1", "stellar.toml"). */
  registry?: string;
}

/**
 * Anchor registries used by the asset verification system.
 *
 * Each registry exposes a list of anchors that can be queried for
 * asset verification. Additional registries can be appended here to
 * extend verification coverage without changing consumers.
 */
export interface AnchorRegistry {
  /** Stable identifier for the registry. */
  id: string;
  /** Human readable name. */
  name: string;
  /** Resolve the anchors published by this registry. */
  getAnchors: () => Anchor[];
}

const SEP1_REGISTRY: AnchorRegistry = {
  id: "sep1",
  name: "SEP-1 stellar.toml",
  getAnchors: () => [
    {
      name: "Circle",
      homeDomain: "circle.com",
      network: Networks.PUBLIC,
      registry: "sep1",
    },
    {
      name: "MoneyGram",
      homeDomain: "moneygram.com",
      network: Networks.PUBLIC,
      registry: "sep1",
    },
  ],
};

/**
 * Additional anchor registries integrated with the asset verification
 * system. Registries are additive: consumers iterate over this list so
 * new registries can be plugged in without touching verification logic.
 */
export const ANCHOR_REGISTRIES: AnchorRegistry[] = [SEP1_REGISTRY];

/**
 * Aggregate anchors from every registered anchor registry.
 *
 * Deduplicates by home domain so the same anchor published by multiple
 * registries is only returned once, preserving existing behavior for
 * the SEP-1 registry while enabling additional registries.
 */
export function getAnchorsFromRegistries(
  registries: AnchorRegistry[] = ANCHOR_REGISTRIES,
): Anchor[] {
  const seen = new Set<string>();
  const anchors: Anchor[] = [];

  for (const registry of registries) {
    for (const anchor of registry.getAnchors()) {
      const key = anchor.homeDomain.toLowerCase();
      if (seen.has(key)) continue;
      seen.add(key);
      anchors.push(anchor);
    }
  }

  return anchors;
}

export const anchors: Anchor[] = getAnchorsFromRegistries();
