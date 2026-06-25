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
export {
  useRateLimitStatus,
  clearRateLimitCache,
  type RateLimitStatus,
  type UseRateLimitStatusResult,
  type UseRateLimitStatusOptions,
} from './useRateLimitStatus';
export { type RateLimitStatusRaw, ContractError } from './contractClient';