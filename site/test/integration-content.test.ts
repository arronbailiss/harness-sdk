import { describe, expect, it } from 'vitest'
import { getIntegrationEntries, getLanguageSupport } from '../src/util/integration-content'

describe('getLanguageSupport', () => {
  it('keeps unspecified and empty language fields on the Python/TypeScript defaults', () => {
    expect(getLanguageSupport(undefined)).toEqual({ python: true, typescript: true, rust: false })
    expect(getLanguageSupport([])).toEqual({ python: true, typescript: true, rust: false })
  })

  it('recognizes explicit Rust support', () => {
    expect(getLanguageSupport('Rust')).toEqual({ python: false, typescript: false, rust: true })
    expect(getLanguageSupport(['Python', 'Rust'])).toEqual({ python: true, typescript: false, rust: true })
  })
})

describe('getIntegrationEntries', () => {
  it('exposes Rust support on processed entries', () => {
    const docs = [
      {
        id: 'docs/integrations/tools/rust-tool',
        data: {
          title: 'Rust tool',
          integrationType: 'tool',
          languages: ['Rust'],
        },
      },
    ] as Parameters<typeof getIntegrationEntries>[0]

    expect(getIntegrationEntries(docs, 'tool')).toEqual([
      {
        id: 'docs/integrations/tools/rust-tool',
        title: 'Rust tool',
        displayTitle: 'Rust tool',
        href: '/docs/integrations/tools/rust-tool/',
        pythonSupport: false,
        typescriptSupport: false,
        rustSupport: true,
        community: false,
        description: undefined,
      },
    ])
  })
})
