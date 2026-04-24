import { describe, it, expect } from "vitest";
import { toStroops, fromStroops, USDC_MULTIPLIER } from "../src/index.js";

describe("toStroops / fromStroops", () => {
  it("converts 1 USDC to 10_000_000 stroops", () => {
    expect(toStroops(1)).toBe(USDC_MULTIPLIER);
  });

  it("round-trips correctly", () => {
    expect(fromStroops(toStroops(42.5))).toBeCloseTo(42.5);
  });

  it("handles zero", () => {
    expect(toStroops(0)).toBe(0n);
    expect(fromStroops(0n)).toBe(0);
  });
});
