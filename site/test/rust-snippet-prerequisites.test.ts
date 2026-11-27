import { readFileSync } from 'node:fs'

import { describe, expect, it } from 'vitest'

const docsDirectory = new URL('../src/content/docs/', import.meta.url)

const PAGES_WITH_NON_DEFAULT_RUST_PREREQUISITES = [
  {
    page: 'user-guide/sdk/agents/structured-output-behavior',
    sourceMarkers: ['use schemars::JsonSchema;', 'use serde::Deserialize;'],
    setupCommands: ['cargo add schemars', 'cargo add serde --features derive'],
  },
  {
    page: 'user-guide/sdk/tools/vended-tools/mcp-router',
    sourceMarkers: ['use strands::mcp::McpServerConfig;', 'use strands::vended_tools::mcp_router::'],
    setupCommands: ['cargo add strands --features mcp'],
  },
]

describe('Rust snippet prerequisites', () => {
  it.each(PAGES_WITH_NON_DEFAULT_RUST_PREREQUISITES)(
    'documents the non-default setup for $page',
    ({ page, sourceMarkers, setupCommands }) => {
      const rust = readFileSync(new URL(`${page}.rs`, docsDirectory), 'utf8')
      const mdx = readFileSync(new URL(`${page}.mdx`, docsDirectory), 'utf8')

      for (const marker of sourceMarkers) expect(rust).toContain(marker)
      for (const command of setupCommands) expect(mdx).toContain(command)
    }
  )
})
