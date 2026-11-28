import { readFileSync } from 'node:fs'

import { describe, expect, it } from 'vitest'

const docsDirectory = new URL('../src/content/docs/', import.meta.url)

const PAGES_WITH_NON_DEFAULT_RUST_PREREQUISITES = [
  {
    page: 'user-guide/sdk/agents/agent-loop',
    sourceMarkers: ['tokio::spawn', 'tokio::time::sleep'],
    setupCommands: ['cargo add tokio --features rt,time'],
  },
  {
    page: 'user-guide/sdk/agents/approve-before-acting',
    sourceMarkers: ['use strands::vended_interventions::cedar::'],
    setupCommands: ['cargo add strands --features cedar'],
  },
  {
    page: 'user-guide/sdk/agents/lifecycle-controls',
    sourceMarkers: ['tokio::spawn', 'tokio::time::sleep'],
    setupCommands: ['cargo add tokio --features rt,time'],
  },
  {
    page: 'user-guide/sdk/agents/middleware',
    sourceMarkers: ['use futures::StreamExt;', 'serde_json::to_string', 'tokio::time::sleep'],
    setupCommands: ['cargo add futures serde_json', 'cargo add tokio --features time'],
  },
  {
    page: 'user-guide/sdk/agents/prompts',
    sourceMarkers: ['serde_json::json!'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/agents/retry-strategies',
    sourceMarkers: ['tokio::time::sleep'],
    setupCommands: ['cargo add tokio --features time'],
  },
  {
    page: 'user-guide/sdk/agents/snapshots',
    sourceMarkers: ['serde_json::to_string_pretty', 'serde_json::from_str'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/agents/state',
    sourceMarkers: ['serde_json::json!', 'serde_json::Value::Null'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/agents/structured-output-behavior',
    sourceMarkers: ['use schemars::JsonSchema;', 'use serde::Deserialize;'],
    setupCommands: ['cargo add schemars', 'cargo add serde --features derive'],
  },
  {
    page: 'user-guide/sdk/deploy/index',
    sourceMarkers: ['use serde::{Deserialize, Serialize};'],
    setupCommands: ['cargo add serde --features derive'],
  },
  {
    page: 'user-guide/sdk/interrupts',
    sourceMarkers: ['use serde_json::json;'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/memory/managing-memory',
    sourceMarkers: ['use serde_json::json;'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/memory/test-memory-store',
    sourceMarkers: ['use serde_json::json;'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/model-providers/custom_model_provider',
    sourceMarkers: ['futures::stream::empty()', 'futures::stream::iter('],
    setupCommands: ['cargo add futures'],
  },
  {
    page: 'user-guide/sdk/model-providers/model-routing',
    sourceMarkers: ['use serde_json::json;', '#[tokio::main]'],
    setupCommands: ['cargo add serde_json', 'cargo add tokio --features macros,rt-multi-thread'],
  },
  {
    page: 'user-guide/sdk/model-providers/openai-responses',
    sourceMarkers: ['use serde_json::json;'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/model-providers/prompt-caching',
    sourceMarkers: ['use serde_json::json;'],
    setupCommands: ['cargo add serde_json'],
  },
  {
    page: 'user-guide/sdk/plugins/custom-plugins',
    sourceMarkers: ['tokio::fs::read_to_string'],
    setupCommands: ['cargo add tokio --features fs'],
  },
  {
    page: 'user-guide/sdk/sandbox/custom-sandbox',
    sourceMarkers: ['use futures::stream;'],
    setupCommands: ['cargo add futures'],
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
