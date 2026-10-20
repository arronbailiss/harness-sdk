// Map a repo + release tag to changelog metadata, and build package-registry URLs.
// Pure, dependency-free. Mirrors site/src/config/changelog.ts (Plan 1 contract).

import type { TagMeta, Sdk, Language } from './types'

function cleanVersion(raw: string): string {
  // Strip a leading 'v' and any stray dot after it (handles 'v.1.2.0').
  return raw.replace(/^v\.?/, '')
}

const MONOREPO_STREAMS: Record<string, { sdk: Sdk; language?: Language }> = {
  'python/': { sdk: 'sdk', language: 'python' },
  'typescript/': { sdk: 'sdk', language: 'typescript' },
  'harness-python/': { sdk: 'harness', language: 'python' },
  'harness-typescript/': { sdk: 'harness', language: 'typescript' },
  'harness-cli/': { sdk: 'cli' },
  'mcp/': { sdk: 'mcp' },
}

// Repos that release one stream under bare `v` tags. `mcp-server` is the
// archived pre-monorepo MCP server repo (used only by the one-time backfill).
const BARE_TAG_REPOS = new Map<string, Sdk>([
  ['box', 'box'],
  ['shell', 'shell'],
  ['agent-sop', 'agent-sop'],
  ['mcp-server', 'mcp'],
])

function monorepoPrefix(tag: string): string | undefined {
  return Object.keys(MONOREPO_STREAMS).find((p) => tag.startsWith(p))
}

/** True for a harness-sdk tag in a per-package stream (not a pre-monorepo bare `v` tag). */
export function isMonorepoTag(tag: string): boolean {
  return monorepoPrefix(tag) !== undefined
}

/** Identifies a release stream, e.g. 'sdk:python', 'cli', 'evals'. */
export function streamKey(sdk: Sdk, language?: Language): string {
  return language ? `${sdk}:${language}` : sdk
}

/**
 * @param repo  e.g. 'strands-agents/harness-sdk' | 'strands-agents/evals' | 'strands-agents/shell'
 * @param tag   the release tag
 */
export function tagToMeta(repo: string, tag: string): TagMeta | null {
  const isEvals = repo.endsWith('/evals')
  if (isEvals) {
    // Evals is python-only; accept bare vX or python/vX.
    const m = tag.match(/(?:^|\/)v\.?(\d.*)$/)
    if (!m) return null
    return { sdk: 'evals', language: undefined, version: cleanVersion('v' + m[1]) }
  }
  // The archived pre-monorepo TypeScript repo: all its releases are
  // sdk/typescript history (used only by the one-time backfill).
  if (repo.endsWith('/sdk-typescript')) {
    if (!/^v\.?\d/.test(tag)) return null
    return { sdk: 'sdk', language: 'typescript', version: cleanVersion(tag) }
  }
  const bareTagSdk = BARE_TAG_REPOS.get(repo.split('/')[1])
  if (bareTagSdk) {
    if (!/^v\.?\d/.test(tag)) return null
    return { sdk: bareTagSdk, language: undefined, version: cleanVersion(tag) }
  }
  // harness-sdk
  const prefix = monorepoPrefix(tag)
  if (prefix) return { ...MONOREPO_STREAMS[prefix], version: cleanVersion(tag.slice(prefix.length)) }
  if (/^v\.?\d/.test(tag)) {
    return { sdk: 'sdk', language: 'python', version: cleanVersion(tag) }
  }
  return null
}

const pypi = (name: string, v: string) => `https://pypi.org/project/${name}/${v}/`
const npm = (name: string, v: string) => `https://www.npmjs.com/package/${name}/v/${v}`

export function getPackageUrl(sdk: Sdk, language: Language | undefined, version: string): string {
  if (sdk === 'evals') return pypi('strands-agents-evals', version)
  if (sdk === 'box') return `https://github.com/strands-agents/box/releases/tag/v${version}`
  // Shell publishes to both PyPI and npm from one tag, so link the release itself.
  if (sdk === 'shell') return `https://github.com/strands-agents/shell/releases/tag/v${version}`
  if (sdk === 'mcp') return pypi('strands-agents-mcp-server', version)
  if (sdk === 'agent-sop') return pypi('strands-agents-sops', version)
  if (sdk === 'cli') return npm('@strands-agents/cli', version)
  if (sdk === 'harness') {
    return language === 'typescript' ? npm('@strands-agents/harness', version) : pypi('strands-harness', version)
  }
  if (language === 'typescript') return npm('@strands-agents/sdk', version)
  return pypi('strands-agents', version)
}
