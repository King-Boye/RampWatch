import type { Meta, StoryObj } from "@storybook/react";
import { action } from "@storybook/addon-actions";
import { TransactionTimeline, type TransactionTimelineProps, type TxEvent } from "./TransactionTimeline";

// ─── Shared fixture helpers ────────────────────────────────────────────────────

const minsAgo = (n: number) => new Date(Date.now() - n * 60_000).toISOString();

// ─── Per-story event fixtures ──────────────────────────────────────────────────

const PENDING_EVENTS: TxEvent[] = [
  { status: "initiated",     timestamp: minsAgo(14), detail: "via ACH transfer", rawApiResponse: { transaction: { id: "dep_8f3c1a9e2b", status: "completed", kind: "deposit", amount_in: "250.00", asset_code: "USDC" } } },
  { status: "awaiting_user", timestamp: minsAgo(12), detail: "Reference: ANC-20240115-0042", rawApiResponse: { transaction: { id: "dep_8f3c1a9e2b", status: "pending_user_transfer_start", more_info_url: "https://anchor.example.com/sep24/transaction/dep_8f3c1a9e2b" } } },
  { status: "pending",       timestamp: minsAgo(3),  description: "Funds detected on the ACH rail — awaiting settlement.", rawApiResponse: { transaction: { id: "dep_8f3c1a9e2b", status: "pending_external", amount_in: "250.00", amount_fee: "1.50" } } },
];

const AWAITING_USER_EVENTS: TxEvent[] = [
  { status: "initiated",     timestamp: minsAgo(4), detail: "via Wire transfer" },
  { status: "awaiting_user", timestamp: minsAgo(2), description: "Send your wire to the account details provided in the interactive flow." },
];

const COMPLETED_EVENTS: TxEvent[] = [
  { status: "initiated",     timestamp: minsAgo(78) },
  { status: "awaiting_user", timestamp: minsAgo(75), detail: "via ACH transfer" },
  { status: "pending",       timestamp: minsAgo(62) },
  { status: "processing",    timestamp: minsAgo(38) },
  {
    status: "completed",
    timestamp: minsAgo(6),
    txHash: "4a7f8c3d2e1b9a6f5c0d3e8b1a4f7c2d5e8b1a4f7c2d9e6b3a0f5c8d1e4b7a",
    detail: "0.001 XLM fee",
  },
];

const FAILED_EVENTS: TxEvent[] = [
  { status: "initiated",     timestamp: minsAgo(50), detail: "to SEPA •••• 4821", rawApiResponse: { transaction: { id: "wdl_9b3e7c1a4f", status: "completed", kind: "withdrawal" } } },
  { status: "awaiting_user", timestamp: minsAgo(49), rawApiResponse: { transaction: { id: "wdl_9b3e7c1a4f", status: "pending_user_transfer_start" } } },
  { status: "pending",       timestamp: minsAgo(44), rawApiResponse: { transaction: { id: "wdl_9b3e7c1a4f", status: "pending_external" } } },
  {
    status: "failed",
    timestamp: minsAgo(22),
    label: "Bank Rejected",
    description: "Destination bank rejected the transfer. Please verify your IBAN and try again.",
    error: "SEPA transfer declined: invalid IBAN checksum (error code: RJCT-AC01)",
    rawApiResponse: { transaction: { id: "wdl_9b3e7c1a4f", status: "error", message: "Bank rejected: invalid IBAN", external_transaction_id: "SEPA-20240115-9821" } },
  },
];

const REFUNDED_EVENTS: TxEvent[] = [
  { status: "initiated",     timestamp: minsAgo(95) },
  { status: "awaiting_user", timestamp: minsAgo(92), detail: "via ACH transfer" },
  { status: "pending",       timestamp: minsAgo(72) },
  { status: "processing",    timestamp: minsAgo(50) },
  {
    status: "refunded",
    timestamp: minsAgo(12),
    description: "Deposit could not be credited to your account — the full amount will be returned within 3–5 business days.",
  },
];

// ─── Shared base for Playground ───────────────────────────────────────────────

const BASE_PROPS: TransactionTimelineProps = {
  type: "deposit",
  amount: "250.00",
  asset: "USDC",
  id: "dep_8f3c1a9e2b",
  currentStatus: "pending",
  events: PENDING_EVENTS,
  onRetry: action("onRetry"),
  onClose: action("onClose"),
};

// ─── Meta ─────────────────────────────────────────────────────────────────────

const meta: Meta<typeof TransactionTimeline> = {
  title: "Components/TransactionTimeline",
  component: TransactionTimeline,
  tags: ["autodocs"],
  parameters: {
    layout: "centered",
    docs: {
      description: {
        component:
          "Step-by-step status timeline for SEP-24 deposit and withdrawal transactions. Visualises the anchor processing lifecycle — from initiation through user action, external settlement, on-chain minting, and terminal states (completed, failed, refunded) — with animated state transitions.",
      },
    },
  },
  argTypes: {
    type: {
      control: "select",
      options: ["deposit", "withdrawal"],
      description: "Direction of the transaction",
      table: { category: "Transaction" },
    },
    currentStatus: {
      control: "select",
      options: ["initiated", "awaiting_user", "pending", "processing", "completed", "failed", "refunded"],
      description: "Active status that drives the timeline highlight position and header colour",
      table: { category: "Transaction" },
    },
    amount: {
      control: "text",
      description: "Formatted transaction amount (display only — no parsing)",
      table: { category: "Transaction" },
    },
    asset: {
      control: "text",
      description: "Asset code shown next to the amount, e.g. USDC or EURC",
      table: { category: "Transaction" },
    },
    id: {
      control: "text",
      description: "Anchor transaction ID — only the last 8 characters are shown in the header",
      table: { category: "Transaction" },
    },
    events: {
      control: false,
      description: "Ordered list of TxEvent objects; each entry populates a matching timeline step",
      table: { category: "Data" },
    },
    onRetry: {
      control: false,
      description: "Called when the user clicks Retry — only rendered for failed transactions",
      table: { category: "Actions" },
    },
    onClose: {
      control: false,
      description: "Called when the user dismisses the timeline card",
      table: { category: "Actions" },
    },
  },
  args: BASE_PROPS,
};

export default meta;
type Story = StoryObj<typeof TransactionTimeline>;

// ─── Stories ──────────────────────────────────────────────────────────────────

/**
 * Funds have been detected on the ACH rail but have not yet settled.
 * The anchor is polling for confirmation — this is the most common
 * in-flight state a user will see during a deposit.
 */
export const Pending: Story = {
  name: "Pending",
  args: {
    type: "deposit",
    amount: "250.00",
    asset: "USDC",
    id: "dep_8f3c1a9e2b",
    currentStatus: "pending",
    events: PENDING_EVENTS,
  },
  parameters: {
    docs: {
      description: {
        story:
          "The anchor has received the external funds on the rail but is still awaiting final settlement confirmation. The Pending node pulses to indicate live monitoring.",
      },
    },
  },
};

/**
 * The anchor's interactive flow is complete but the user must now
 * initiate their own bank transfer before processing can continue.
 * Maps to SEP-24 `pending_user_transfer_start`.
 */
export const AwaitingUser: Story = {
  name: "Awaiting User",
  args: {
    type: "deposit",
    amount: "500.00",
    asset: "USDC",
    id: "dep_3c1a5e2f9b",
    currentStatus: "awaiting_user",
    events: AWAITING_USER_EVENTS,
  },
  parameters: {
    docs: {
      description: {
        story:
          "KYC / auth flow is complete. The anchor is ready and waiting for the user to push funds via wire or ACH. The \"Action Required\" badge and orange accent make the required action unmissable.",
      },
    },
  },
};

/**
 * All five steps resolved successfully. The Stellar transaction hash
 * is surfaced on the completed step for on-chain verification.
 */
export const Completed: Story = {
  name: "Completed",
  args: {
    type: "deposit",
    amount: "1,000.00",
    asset: "USDC",
    id: "dep_5a7d3f1b8c",
    currentStatus: "completed",
    events: COMPLETED_EVENTS,
  },
  parameters: {
    docs: {
      description: {
        story:
          "Happy path — USDC has landed in the user's Stellar account. The header turns green, the Done button is shown, and the Stellar tx hash links to stellar.expert for independent verification.",
      },
    },
  },
};

/**
 * Terminal error state: the withdrawal was rejected by the destination bank.
 * The red failure node is appended below the standard steps and the
 * Retry button is rendered because `onRetry` is provided.
 */
export const Failed: Story = {
  name: "Failed",
  args: {
    type: "withdrawal",
    amount: "500.00",
    asset: "USDC",
    id: "wdl_9b3e7c1a4f",
    currentStatus: "failed",
    events: FAILED_EVENTS,
  },
  parameters: {
    docs: {
      description: {
        story:
          "The destination bank rejected the withdrawal. The standard steps are dimmed and the red failure node surfaces the rejection reason and timestamp. Pass `onRetry` to show the Retry button.",
      },
    },
  },
};

/**
 * The deposit could not be credited and the funds are being returned.
 * Distinct from Failed — the money is on its way back, not simply lost.
 * Maps to SEP-24 `refunded`.
 */
export const Refunded: Story = {
  name: "Refunded",
  args: {
    type: "deposit",
    amount: "350.00",
    asset: "USDC",
    id: "dep_7e4b2c9f1a",
    currentStatus: "refunded",
    events: REFUNDED_EVENTS,
  },
  parameters: {
    docs: {
      description: {
        story:
          "The anchor could not complete the deposit and is returning the full amount. The violet accent distinguishes this from a hard failure — the user's funds are not lost, just returning.",
      },
    },
  },
};

/**
 * Interactive sandbox. Use the controls panel to toggle type, status,
 * amount, and asset to explore any combination without switching stories.
 */
export const Playground: Story = {
  name: "Playground",
  args: {
    ...BASE_PROPS,
    type: "deposit",
    amount: "250.00",
    asset: "USDC",
    id: "dep_8f3c1a9e2b",
    currentStatus: "processing",
    events: [
      { status: "initiated",     timestamp: minsAgo(22), detail: "via ACH transfer" },
      { status: "awaiting_user", timestamp: minsAgo(20), detail: "Reference: ANC-20240115-0099" },
      { status: "pending",       timestamp: minsAgo(14) },
      { status: "processing",    timestamp: minsAgo(4) },
    ],
  },
  parameters: {
    docs: {
      description: {
        story:
          "All controls are active. Change `currentStatus` to jump between any state, or flip `type` between deposit and withdrawal to see how descriptions adapt.",
      },
    },
  },
};
