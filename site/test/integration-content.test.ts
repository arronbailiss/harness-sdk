import { describe, expect, it } from 'vitest'
import { getIntegrationEntries, getLanguageSupport } from '../src/util/integration-content'

describe('getLanguageSupport', () => {
  it('uses all SDK languages for unspecified and empty language fields', () => {
    expect(getLanguageSupport(undefined)).toEqual({ python: true, typescript: true, rust: true })
    expect(getLanguageSupport([])).toEqual({ python: true, typescript: true, rust: true })
  })

  it('removes unsupported languages from the defaults', () => {
    expect(getLanguageSupport(undefined, 'Rust')).toEqual({
      python: true,
      typescript: true,
      rust: false,
    })
    expect(getLanguageSupport(undefined, ['Python', 'TypeScript'])).toEqual({
      python: false,
      typescript: false,
      rust: true,
    })
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

  it('excludes unsupported languages from entries with omitted languages', () => {
    const docs = [
      {
        id: 'docs/integrations/tools/python-tool',
        data: {
          title: 'Python tool',
          integrationType: 'tool',
          unsupportedLanguages: ['TypeScript', 'Rust'],
        },
      },
    ] as Parameters<typeof getIntegrationEntries>[0]

    expect(getIntegrationEntries(docs, 'tool')[0]).toMatchObject({
      pythonSupport: true,
      typescriptSupport: false,
      rustSupport: false,
    })
  })
})
