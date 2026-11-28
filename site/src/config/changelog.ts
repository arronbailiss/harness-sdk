export type Sdk = 'sdk' | 'harness' | 'cli' | 'evals' | 'box' | 'shell'
export type Language = 'python' | 'typescript' | 'rust'
export type PackageRegistryLabel = 'PyPI' | 'npm' | 'crates.io' | 'Package'

export const LANGUAGE_META: Record<Language, { label: string; short: string }> = {
  python: { label: 'Python', short: 'Py' },
  typescript: { label: 'TypeScript', short: 'TS' },
  rust: { label: 'Rust', short: 'Rs' },
}

// Key order is the order of the product toggle; the first is the default.
// An empty `languages` marks a product with no language streams (one stream).
export const SDK_META: Record<Sdk, { label: string; languages: Language[] }> = {
  sdk: { label: 'SDK', languages: ['python', 'typescript', 'rust'] },
  harness: { label: 'Harness', languages: ['python', 'typescript', 'rust'] },
  cli: { label: 'Harness CLI', languages: [] },
  evals: { label: 'Evals', languages: ['python'] },
  box: { label: 'Box', languages: [] },
  shell: { label: 'Shell', languages: [] },
}

/**
 * Release stream for each navbar product, used by the docs footer to show the
 * release of the product being read. Every product label needs an entry (a
 * test enforces it); `null` marks a product with no release stream yet, which
 * shows no release. The `cli` stream has no docs product of its own, so it
 * surfaces only on the changelog.
 */
export const PRODUCT_RELEASE_STREAM: Record<string, Sdk | null> = {
  Harness: 'harness',
  'Harness SDK': 'sdk',
  Shell: 'shell',
  Box: 'box',
  Evals: 'evals',
  Decider: null,
}

/** Products released as separate per-language streams; only these carry `language`. */
export function hasLanguageStreams(sdk: Sdk): boolean {
  return SDK_META[sdk].languages.length > 1
}

/** Language encoded by a release anchor when that product has per-language streams. */
export function languageForReleaseAnchor(sdk: Sdk, language: string | undefined): Language | undefined {
  if (!hasLanguageStreams(sdk)) return undefined
  return SDK_META[sdk].languages.find((candidate) => candidate === language)
}

/** Registry label shown next to a release package link. */
export function packageRegistryLabel(language: Language | undefined, packageUrl: string): PackageRegistryLabel {
  if (language === 'python') return 'PyPI'
  if (language === 'typescript') return 'npm'
  if (language === 'rust') return 'crates.io'
  if (packageUrl.includes('pypi.org')) return 'PyPI'
  if (packageUrl.includes('npmjs.com')) return 'npm'
  if (packageUrl.includes('crates.io')) return 'crates.io'
  return 'Package'
}

// The site-wide language preference key + default live in
// util/language-preference.ts (shared with LanguageToggle and the landing page).

/**
 * Canonical human label for a release stream, e.g. "SDK Python", "Harness CLI".
 * Single source so the detail page, markdown endpoints, RSS, and llms.txt don't
 * each hand-roll it (they previously drifted between "Strands harness Python" and
 * "Strands harness (python)").
 */
export function streamLabel(sdk: Sdk, language?: Language): string {
  return [SDK_META[sdk].label, language ? LANGUAGE_META[language].label : ''].filter(Boolean).join(' ')
}

// Package-registry URL construction lives in the devtools changelog-release-pr
// action (the producer of `packageUrl` frontmatter) — not duplicated here.
