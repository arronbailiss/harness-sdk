/**
 * Utilities for querying and filtering integration content from the docs collection.
 *
 * This module provides typed helper functions for working with integration pages
 * (model providers, tools, session managers, memory stores, etc.) that have `integrationType` frontmatter.
 */

import type { CollectionEntry } from 'astro:content'
import { DEFAULT_DOC_LANGUAGES, normalizeSdkLanguages } from '../config/languages'

/**
 * Integration types that can be used to filter content.
 */
export type IntegrationType =
  | 'model-provider'
  | 'decision-model'
  | 'tool'
  | 'session-manager'
  | 'memory-store'
  | 'storage'
  | 'integration'
  | 'plugin'
  | 'observability'
  | 'agent-extension'
  | 'intervention'

/**
 * Represents language support for an integration.
 */
export interface LanguageSupport {
  python: boolean
  typescript: boolean
  rust: boolean
}

/**
 * Represents a processed integration entry with computed properties.
 */
export interface IntegrationEntry {
  /** The document ID (path) */
  id: string
  /** The page title */
  title: string
  /** The display title (sidebar label if set, otherwise page title) */
  displayTitle: string
  /** The href to the page */
  href: string
  /** Python support status */
  pythonSupport: boolean
  /** TypeScript support status */
  typescriptSupport: boolean
  /** Rust support status */
  rustSupport: boolean
  /** Whether this is a community-contributed integration */
  community: boolean
  /** Optional description for catalog listings */
  description?: string | undefined
}

/**
 * Determines language support from the docs language metadata.
 *
 * Convention:
 * - No `languages` field = all SDK languages except `unsupportedLanguages`
 * - `languages: 'Python'` = Python only
 * - `languages: 'TypeScript'` = TypeScript only
 * - `languages: 'Rust'` = Rust only
 * - `languages: ['Python', 'TypeScript', 'Rust']` = all three supported
 */
export function getLanguageSupport(
  languages: string | string[] | undefined,
  unsupportedLanguages?: string | string[]
): LanguageSupport {
  const unsupported = normalizeSdkLanguages(unsupportedLanguages ?? [])
  const normalized =
    !languages || (Array.isArray(languages) && languages.length === 0)
      ? DEFAULT_DOC_LANGUAGES.filter((language) => !unsupported.includes(language))
      : normalizeSdkLanguages(languages)
  return {
    python: normalized.includes('python'),
    typescript: normalized.includes('typescript'),
    rust: normalized.includes('rust'),
  }
}

/**
 * Filters and processes docs collection entries by integration type and community flag.
 *
 * @param docs - The full docs collection from `getCollection('docs')`
 * @param integrationType - The integration type to filter by
 * @param community - When true, returns only community entries; when false, returns only official entries
 * @returns Sorted array of integration entries (alphabetically by display name)
 */
export function getIntegrationEntries(
  docs: CollectionEntry<'docs'>[],
  integrationType: IntegrationType,
  community: boolean = false
): IntegrationEntry[] {
  return docs
    .filter((doc) => {
      if (doc.data.integrationType !== integrationType) return false
      if ((doc.data.community === true) !== community) return false
      return true
    })
    .map((doc) => {
      const { python, typescript, rust } = getLanguageSupport(doc.data.languages, doc.data.unsupportedLanguages)

      // Get sidebar label if available
      const sidebarLabel = (doc.data.sidebar as { label?: string } | undefined)?.label

      // Get description if available
      const description = (doc.data as { description?: string }).description

      return {
        id: doc.id,
        title: doc.data.title as string,
        displayTitle: sidebarLabel ?? (doc.data.title as string),
        href: `/docs/${doc.id.replace(/^docs\//, '')}/`,
        pythonSupport: python,
        typescriptSupport: typescript,
        rustSupport: rust,
        community: doc.data.community === true,
        description,
      }
    })
    .sort((a, b) => a.displayTitle.localeCompare(b.displayTitle))
}
