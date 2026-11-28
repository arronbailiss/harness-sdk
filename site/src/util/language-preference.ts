/**
 * Shared constants and helpers for the site-wide SDK language
 * preference and clipboard interactions used by the landing page and header.
 */
import { SDK_LANGUAGE_LABELS } from '../config/languages'

/**
 * Stable sync key for every SDK language tab group. Keep this value when
 * adding languages so existing preferences remain valid.
 */
export const LANGUAGE_SYNC_KEY = 'jarkqt'

/**
 * localStorage key for the selected language. The key retains the original
 * Python/TypeScript sync key so preferences survive the addition of Rust.
 */
export const LANGUAGE_STORAGE_KEY = `starlight-synced-tabs__${LANGUAGE_SYNC_KEY}`

/**
 * Language assumed when no preference is stored yet. Uses the display label
 * because that is what Starlight's synced tabs write to storage.
 */
export const DEFAULT_LANGUAGE_LABEL = SDK_LANGUAGE_LABELS.typescript

/**
 * Copy text to the clipboard, resolving to whether it succeeded. Returns false
 * instead of throwing when the Clipboard API is unavailable (non-secure
 * contexts) or permission is denied, so callers can avoid showing a success
 * state for a copy that did not happen.
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text)
    return true
  } catch {
    return false
  }
}
