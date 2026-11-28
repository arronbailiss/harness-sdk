import { getCollection } from 'astro:content'
import { describe, expect, it } from 'vitest'
import { normalizeSdkLanguages } from '../src/config/languages'

const OPENAI_COMPATIBLE_PROVIDERS = [
  'crusoe',
  'fireworksai',
  'nebius-token-factory',
  'openrouter',
  'orcarouter',
  'ovhcloud-ai-endpoints',
]

function normalized(value: string | string[] | undefined) {
  return normalizeSdkLanguages(value ?? [])
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
