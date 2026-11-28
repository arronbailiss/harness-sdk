import { getCollection } from 'astro:content'
import { describe, expect, it } from 'vitest'
import {
  DEFAULT_DOC_LANGUAGES,
  SDK_LANGUAGES,
  normalizeSdkLanguages,
  sdkLanguageAvailability,
  type SdkLanguage,
  type SdkLanguageAvailability,
} from '../src/config/languages'

const OPENAI_COMPATIBLE_PROVIDERS = [
  'cohere',
  'crusoe',
  'fireworksai',
  'nebius-token-factory',
  'openrouter',
  'orcarouter',
  'ovhcloud-ai-endpoints',
]

const PYTHON_ONLY_PRODUCT_PAGE_IDS = [
  'docs/integrations/agent-extensions/strands-code-agent',
  'docs/integrations/integrations/ag-ui',
  'docs/integrations/interventions/strands-agt',
  'docs/integrations/memory-stores/strands-dakera',
  'docs/integrations/model-providers/clova-studio',
  'docs/integrations/model-providers/mlx',
  'docs/integrations/model-providers/nvidia-nim',
  'docs/integrations/model-providers/sglang',
  'docs/integrations/model-providers/vllm',
  'docs/integrations/model-providers/xai',
  'docs/integrations/plugins/agent-control',
  'docs/integrations/plugins/agentcore-payments',
  'docs/integrations/plugins/agentcore-tool-search',
  'docs/integrations/plugins/datadog-ai-guard',
  'docs/integrations/plugins/s3-vectors-memory',
  'docs/integrations/session-managers/agentcore-memory',
  'docs/integrations/session-managers/strands-valkey-session-manager',
  'docs/integrations/tools/strands-apify',
  'docs/integrations/tools/strands-deepgram',
  'docs/integrations/tools/strands-google',
  'docs/integrations/tools/strands-hubspot',
  'docs/integrations/tools/strands-perplexity',
  'docs/integrations/tools/strands-spraay',
  'docs/integrations/tools/strands-sql',
  'docs/integrations/tools/strands-teams',
  'docs/integrations/tools/strands-telegram-listener',
  'docs/integrations/tools/strands-telegram',
  'docs/integrations/tools/utcp',
  'docs/user-guide/evals-sdk/evaluators/typed_rubric_evaluator',
  'docs/user-guide/migrate/strands-agents-tools',
  'docs/user-guide/sdk/bidi/agent',
  'docs/user-guide/sdk/bidi/content',
  'docs/user-guide/sdk/bidi/events',
  'docs/user-guide/sdk/bidi/hooks',
  'docs/user-guide/sdk/bidi',
  'docs/user-guide/sdk/bidi/interrupts',
  'docs/user-guide/sdk/bidi/io',
  'docs/user-guide/sdk/bidi/models/bedrock',
  'docs/user-guide/sdk/bidi/models/google',
  'docs/user-guide/sdk/bidi/models',
  'docs/user-guide/sdk/bidi/models/openai/live',
  'docs/user-guide/sdk/bidi/models/openai/realtime',
  'docs/user-guide/sdk/bidi/observability',
  'docs/user-guide/sdk/bidi/quickstart',
  'docs/user-guide/sdk/bidi/session-management',
  'docs/user-guide/sdk/bidi/tools',
  'docs/user-guide/sdk/model-providers/amazon-nova',
  'docs/user-guide/sdk/model-providers/litellm',
  'docs/user-guide/sdk/model-providers/llamaapi',
  'docs/user-guide/sdk/model-providers/llamacpp',
  'docs/user-guide/sdk/model-providers/mistral',
  'docs/user-guide/sdk/model-providers/ollama',
  'docs/user-guide/sdk/model-providers/sagemaker',
  'docs/user-guide/sdk/model-providers/writer',
  'docs/user-guide/sdk/plugins/genai-prices-estimator',
] as const

function normalized(value: string | string[] | undefined) {
  return normalizeSdkLanguages(value ?? [])
}

function languageStates(data: {
  languages?: string | string[]
  unsupportedLanguages?: string | string[]
}): Record<SdkLanguage, SdkLanguageAvailability> {
  const unsupported = normalized(data.unsupportedLanguages)
  const documented = data.languages
    ? normalized(data.languages)
    : DEFAULT_DOC_LANGUAGES.filter((language) => !unsupported.includes(language))

  return Object.fromEntries(
    SDK_LANGUAGES.map((language) => [language, sdkLanguageAvailability(language, documented, unsupported)])
  ) as Record<SdkLanguage, SdkLanguageAvailability>
}

function inSection(id: string, section: string) {
  return id === section || id.startsWith(`${section}/`)
}

describe('product documentation language scope', () => {
  it('marks every Evals SDK page as Python-only product support', async () => {
    const docs = (await getCollection('docs')).filter((doc) => inSection(doc.id, 'docs/user-guide/evals-sdk'))

    expect(docs.length).toBeGreaterThan(0)
    for (const doc of docs) {
      expect(normalized(doc.data.unsupportedLanguages), doc.id).toEqual(expect.arrayContaining(['typescript', 'rust']))
    }
  })

  it('keeps the learning course scoped to its Python examples', async () => {
    const docs = (await getCollection('docs')).filter((doc) => doc.id.startsWith('docs/learning/'))

    expect(docs.length).toBeGreaterThan(0)
    for (const doc of docs) {
      expect(normalized(doc.data.languages), doc.id).toEqual(['python'])
    }
  })

  // Regression: https://github.com/strands-agents/harness-sdk/pull/6447
  it('preserves explicitly unsupported product states', async () => {
    const docs = await getCollection('docs')
    const byId = new Map(docs.map((doc) => [doc.id, doc]))

    for (const id of PYTHON_ONLY_PRODUCT_PAGE_IDS) {
      const doc = byId.get(id)
      expect(doc, `${id}: page is missing`).toBeDefined()
      expect(languageStates(doc!.data), id).toEqual({
        python: 'available',
        typescript: 'unsupported',
        rust: 'unsupported',
      })
    }

    expect(languageStates(byId.get('docs/user-guide/sdk/model-providers/vercel')!.data)).toEqual({
      python: 'unsupported',
      typescript: 'available',
      rust: 'unsupported',
    })
    expect(languageStates(byId.get('docs/user-guide/sdk/agents/interventions/dogwood-authorization')!.data)).toEqual({
      python: 'unsupported',
      typescript: 'available',
      rust: 'available',
    })
    expect(languageStates(byId.get('docs/user-guide/sdk/streaming/streaming-to-the-browser-guide')!.data)).toEqual({
      python: 'undocumented',
      typescript: 'available',
      rust: 'undocumented',
    })
  })

  it('marks every Strands Shell page as unsupported in Rust', async () => {
    const docs = (await getCollection('docs')).filter((doc) => inSection(doc.id, 'docs/user-guide/shell'))

    expect(docs.length).toBeGreaterThan(0)
    for (const doc of docs) {
      expect(normalized(doc.data.unsupportedLanguages), doc.id).toContain('rust')
    }
  })

  it('distinguishes unavailable features from documentation gaps', async () => {
    const docs = await getCollection('docs')
    const byId = new Map(docs.map((doc) => [doc.id, doc]))

    for (const id of [
      'docs/user-guide/sdk/experimental/agent-config',
      'docs/integrations/memory-stores/agentcore-memory-store',
      'docs/integrations/storage/github',
    ]) {
      expect(normalized(byId.get(id)?.data.unsupportedLanguages), id).toContain('rust')
    }

    for (const id of [
      'docs/user-guide/sdk/safety-security/guardrails',
      'docs/user-guide/sdk/safety-security/pii-redaction',
      'docs/user-guide/sdk/safety-security/responsible-ai',
    ]) {
      expect(normalized(byId.get(id)?.data.languages), id).toEqual(['python'])
      expect(normalized(byId.get(id)?.data.unsupportedLanguages), id).not.toContain('rust')
    }

    const langGraph = byId.get('docs/user-guide/migrate/langgraph')
    expect(normalized(langGraph?.data.languages)).toEqual(['python', 'typescript'])
    expect(normalized(langGraph?.data.unsupportedLanguages)).not.toContain('rust')
  })

  it('keeps Rust examples on OpenAI-compatible provider pages', async () => {
    const docs = await getCollection('docs')
    const byId = new Map(docs.map((doc) => [doc.id, doc]))

    for (const provider of OPENAI_COMPATIBLE_PROVIDERS) {
      const id = `docs/integrations/model-providers/${provider}`
      const doc = byId.get(id)

      expect(doc, `${id}: page is missing`).toBeDefined()
      expect(doc?.body, `${id}: Rust tab is missing`).toContain('<Tab label="Rust">')
      expect(doc?.body, `${id}: Rust snippet is missing`).toContain(`integrations/model-providers/${provider}.rs:usage`)
      expect(normalized(doc?.data.unsupportedLanguages), id).not.toContain('rust')
    }
  })
})
