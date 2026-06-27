import type { Meta, StoryObj } from "@storybook/react";
import {
  AnchorCapabilityCard,
  type AnchorCapabilityCardProps,
  type SupportedAsset,
} from "./AnchorCapabilityCard";

// ─── Shared fixture data ───────────────────────────────────────────────────────

const USDC_ASSET: SupportedAsset = {
  code: "USDC",
  name: "USD Coin",
  icon: "💵",
  operationTypes: ["both"],
  depositEnabled: true,
  withdrawalEnabled: true,
  networks: ["ACH", "Wire", "Cash"],
  countries: ["US", "MX", "GB", "DE", "IN", "PH", "NG"],
  fees: {
    deposit: {
      type: "tiered",
      currency: "USD",
      tiers: [
        { upTo: 200, fee: "$1.99" },
        { upTo: 1000, fee: "$3.99" },
        { upTo: null, fee: "0.5%" },
      ],
    },
    withdrawal: {
      type: "tiered",
      currency: "USD",
      tiers: [
        { upTo: 200, fee: "$2.49" },
        { upTo: 1000, fee: "$4.99" },
        { upTo: null, fee: "0.6%" },
      ],
    },
  },
  limits: {
    minDeposit: 10,
    maxDeposit: 10_000,
    minWithdrawal: 10,
    maxWithdrawal: 10_000,
    dailyLimit: 10_000,
    monthlyLimit: 50_000,
    currency: "USD",
  },
  kyc: {
    level: "basic",
    estimatedTime: "< 2 minutes",
    fields: [
      { name: "first_name", label: "First Name", required: true },
      { name: "last_name", label: "Last Name", required: true },
      { name: "email_address", label: "Email Address", required: true },
      { name: "phone_number", label: "Phone Number", required: true },
      { name: "date_of_birth", label: "Date of Birth", required: false },
    ],
  },
};

const EURC_ASSET: SupportedAsset = {
  code: "EURC",
  name: "Euro Coin",
  icon: "💶",
  operationTypes: ["both"],
  depositEnabled: true,
  withdrawalEnabled: true,
  networks: ["SEPA", "SWIFT"],
  countries: ["DE", "FR", "ES", "IT", "NL"],
  fees: {
    deposit: { type: "percent", percent: 0.1, currency: "EUR" },
    withdrawal: { type: "flat", flatAmount: 0.5, currency: "EUR" },
  },
  limits: {
    minDeposit: 10,
    maxDeposit: 50_000,
    minWithdrawal: 10,
    maxWithdrawal: 50_000,
    dailyLimit: 50_000,
    monthlyLimit: 200_000,
    currency: "EUR",
  },
  kyc: {
    level: "full",
    estimatedTime: "1–3 business days",
    description: "EU AML regulations require government-issued ID and address verification.",
    fields: [
      { name: "first_name", label: "First Name", required: true },
      { name: "last_name", label: "Last Name", required: true },
      { name: "id_type", label: "ID Document Type", required: true },
      { name: "id_number", label: "ID Number", required: true },
      { name: "address", label: "Home Address", required: true },
      { name: "date_of_birth", label: "Date of Birth", required: true },
    ],
  },
};

/** Base props shared across all stories. */
const BASE_PROPS: AnchorCapabilityCardProps = {
  anchorName: "MoneyGram",
  domain: "moneygram.stellar.org",
  logoInitials: "MG",
  accentColor: "#2563eb",
  description: "Global cash-in / cash-out network. Available at 350,000+ locations.",
  assets: [USDC_ASSET, EURC_ASSET],
};

// ─── Meta ─────────────────────────────────────────────────────────────────────

const meta: Meta<typeof AnchorCapabilityCard> = {
  title: "Components/AnchorCapabilityCard",
  component: AnchorCapabilityCard,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
    docs: {
      description: {
        component:
          "Displays the full capability profile of a Stellar anchor: supported assets, fees, limits, KYC requirements, and service health.",
      },
    },
  },
  argTypes: {
    // ── Appearance ──────────────────────────────────────────────────────────
    anchorName: {
      control: "text",
      description: "Display name of the anchor",
      table: { category: "Appearance" },
    },
    domain: {
      control: "text",
      description: "SEP-1 TOML domain of the anchor",
      table: { category: "Appearance" },
    },
    logoInitials: {
      control: "text",
      description: "Two-letter initials shown in the logo avatar",
      table: { category: "Appearance" },
    },
    accentColor: {
      control: "color",
      description: "Brand accent color used for active tabs, badges, and highlights",
      table: { category: "Appearance" },
    },
    description: {
      control: "text",
      description: "Short tagline shown in the card header",
      table: { category: "Appearance" },
    },

    // ── State ───────────────────────────────────────────────────────────────
    isLoading: {
      control: "boolean",
      description: "Replaces tab content with an animated skeleton while data is fetching",
      table: { category: "State" },
    },
    error: {
      control: "text",
      description: "Replaces tab content with an error message; leave empty for no error",
      table: { category: "State" },
    },

    // ── Services ────────────────────────────────────────────────────────────
    "services.deposits": {
      control: "boolean",
      description: "Whether the anchor supports deposit operations",
      table: { category: "Services" },
    },
    "services.withdrawals": {
      control: "boolean",
      description: "Whether the anchor supports withdrawal operations",
      table: { category: "Services" },
    },
    "services.quotes": {
      control: "boolean",
      description: "Whether the anchor supports SEP-38 quotes",
      table: { category: "Services" },
    },
    "services.kyc": {
      control: "boolean",
      description: "Whether the anchor has KYC service active",
      table: { category: "Services" },
    },

    // ── Health ──────────────────────────────────────────────────────────────
    "healthStatus.deposits": {
      control: "select",
      options: ["healthy", "degraded", "unavailable", undefined],
      description: "Health of the deposits service",
      table: { category: "Health" },
    },
    "healthStatus.withdrawals": {
      control: "select",
      options: ["healthy", "degraded", "unavailable", undefined],
      description: "Health of the withdrawals service",
      table: { category: "Health" },
    },
    "healthStatus.quotes": {
      control: "select",
      options: ["healthy", "degraded", "unavailable", undefined],
      description: "Health of the quotes service",
      table: { category: "Health" },
    },
    "healthStatus.kyc": {
      control: "select",
      options: ["healthy", "degraded", "unavailable", undefined],
      description: "Health of the KYC service",
      table: { category: "Health" },
    },
  },
  args: BASE_PROPS,
};

export default meta;
type Story = StoryObj<typeof AnchorCapabilityCard>;

// ─── Stories ──────────────────────────────────────────────────────────────────

/**
 * The default fully-loaded state showing all services healthy with two assets.
 * Use this as the baseline for comparing other states.
 */
export const AllServices: Story = {
  name: "All Services (Default)",
  args: {
    ...BASE_PROPS,
    services: {
      deposits: true,
      withdrawals: true,
      quotes: true,
      kyc: true,
    },
    healthStatus: {
      deposits: "healthy",
      withdrawals: "healthy",
      quotes: "healthy",
      kyc: "healthy",
    },
  },
  parameters: {
    docs: {
      description: {
        story:
          "All four services are enabled and reporting healthy. This is the happy-path baseline.",
      },
    },
  },
};

/**
 * Skeleton loader shown while anchor capability data is being fetched.
 * `isLoading=true` replaces the tab body with an animated shimmer.
 */
export const Loading: Story = {
  name: "Loading",
  args: {
    ...BASE_PROPS,
    isLoading: true,
    // Assets are still required by the type; they won't be rendered while loading
    assets: [USDC_ASSET],
  },
  parameters: {
    docs: {
      description: {
        story:
          "Pass `isLoading={true}` while awaiting a SEP-1 or SEP-24 response. The header and tabs render immediately; only the tab body shows a shimmer skeleton.",
      },
    },
  },
};

/**
 * Error state when the capability fetch fails or the anchor is unreachable.
 * The `error` prop drives the message shown inside the tab body.
 */
export const ErrorState: Story = {
  name: "Error",
  args: {
    ...BASE_PROPS,
    error: "Unable to reach moneygram.stellar.org — connection timed out after 10 s.",
    assets: [USDC_ASSET],
  },
  parameters: {
    docs: {
      description: {
        story:
          "Pass a non-empty `error` string to surface a fetch or network failure. The header stays visible so users can still identify the anchor.",
      },
    },
  },
};

/**
 * All services disabled — the anchor has no active on-chain operations.
 * Triggers the "No services available" empty state in the tab body.
 */
export const NoServices: Story = {
  name: "No Services",
  args: {
    ...BASE_PROPS,
    services: {
      deposits: false,
      withdrawals: false,
      quotes: false,
      kyc: false,
    },
    assets: [USDC_ASSET],
  },
  parameters: {
    docs: {
      description: {
        story:
          "Set all `services.*` flags to `false` (or pass `services={}`) to render the no-services empty state. Useful for anchors that are onboarded but not yet live.",
      },
    },
  },
};

/**
 * One or more services are experiencing degraded performance or are completely
 * unavailable. Health badges are surfaced above the tab content so users
 * understand which operations may be slow or blocked.
 */
export const DegradedHealth: Story = {
  name: "Degraded Health",
  args: {
    ...BASE_PROPS,
    services: {
      deposits: true,
      withdrawals: true,
      quotes: true,
      kyc: true,
    },
    healthStatus: {
      deposits: "healthy",
      withdrawals: "degraded",
      quotes: "unavailable",
      kyc: "healthy",
    },
  },
  parameters: {
    docs: {
      description: {
        story:
          "Withdrawals are degraded and Quotes are unavailable. Degraded/unavailable services render colored warning badges above the tab panel so users can quickly spot which operations are impacted.",
      },
    },
  },
};

/**
 * Interactive playground with every prop wired to a control.
 * Use this story to explore all combinations of service flags, health states,
 * accent colors, and copy without jumping between fixed stories.
 */
export const Playground: Story = {
  name: "Playground",
  args: {
    ...BASE_PROPS,
    isLoading: false,
    error: "",
    services: {
      deposits: true,
      withdrawals: true,
      quotes: true,
      kyc: true,
    },
    healthStatus: {
      deposits: "healthy",
      withdrawals: "healthy",
      quotes: "healthy",
      kyc: "healthy",
    },
  },
  parameters: {
    docs: {
      description: {
        story:
          "All controls are active. Toggle `isLoading`, set `error`, flip service flags, or change health values to preview any combination.",
      },
    },
  },
};
