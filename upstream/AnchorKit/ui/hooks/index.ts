/**
 * AnchorKit UI Hooks
 * 
 * Reusable React hooks for common UI patterns
 */

export {
  useCopyToClipboard,
  formatJsonForCopy,
  generateCurlCommand,
  generateInstallCommand,
  type CopyToClipboardOptions,
  type CopyToClipboardResult,
} from './useCopyToClipboard';
export { useTheme } from './useTheme';
export { useTransactionStatus } from './useTransactionStatus';
export type {
  UseTransactionStatusOptions,
  UseTransactionStatusResult,
  FetchStatusFn,
  TransactionTransition,
  TransactionSnapshot,
} from './useTransactionStatus';
export { TransactionStateTracker, isTerminalStatus } from './TransactionStateTracker';
export { useAnchorCapabilities, clearCapabilitiesCache } from './useAnchorCapabilities';
export type {
  AnchorServicesResult,
  FetchCapabilitiesFn,
  UseAnchorCapabilitiesOptions,
  UseAnchorCapabilitiesResult,
} from './useAnchorCapabilities';
export {
  SERVICE_DEPOSITS,
  SERVICE_WITHDRAWALS,
  SERVICE_QUOTES,
  SERVICE_KYC,
} from './useAnchorCapabilities';