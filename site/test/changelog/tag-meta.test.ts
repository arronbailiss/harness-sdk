import { describe, it, expect } from 'vitest'
import { tagToMeta, getPackageUrl } from '../../scripts/changelog/tag-meta'

describe('tag-meta', () => {
  it('harness python prefixed tag', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'python/v1.42.0')).toEqual({
      sdk: 'sdk',
      language: 'python',
      version: '1.42.0',
    })
  })

  it('harness typescript prefixed tag', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'typescript/v1.4.0')).toEqual({
      sdk: 'sdk',
      language: 'typescript',
      version: '1.4.0',
    })
  })

  it('harness bare v tag is pre-monorepo python', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'v1.9.1')).toEqual({
      sdk: 'sdk',
      language: 'python',
      version: '1.9.1',
    })
  })

  it('evals bare v tag is python-only (no language)', () => {
    expect(tagToMeta('strands-agents/evals', 'v0.2.1')).toEqual({
      sdk: 'evals',
      language: undefined,
      version: '0.2.1',
    })
  })

  it('evals python-prefixed tag also maps to evals python', () => {
    expect(tagToMeta('strands-agents/evals', 'python/v0.1.3')).toEqual({
      sdk: 'evals',
      language: undefined,
      version: '0.1.3',
    })
  })

  it('malformed typescript tag still parses', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'typescript/v.1.2.0')).toEqual({
      sdk: 'sdk',
      language: 'typescript',
      version: '1.2.0',
    })
  })

  it('python-wasm is skipped (null)', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'python-wasm/v0.0.1')).toBe(null)
  })

  it('archived sdk-typescript repo bare v tags map to sdk/typescript', () => {
    expect(tagToMeta('strands-agents/sdk-typescript', 'v1.3.0')).toEqual({
      sdk: 'sdk',
      language: 'typescript',
      version: '1.3.0',
    })
    // rc tags parse too
    expect(tagToMeta('strands-agents/sdk-typescript', 'v1.0.0-rc.5')).toEqual({
      sdk: 'sdk',
      language: 'typescript',
      version: '1.0.0-rc.5',
    })
  })

  it('harness package tags map to the harness and cli streams', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'harness-python/v0.3.1')).toEqual({
      sdk: 'harness',
      language: 'python',
      version: '0.3.1',
    })
    expect(tagToMeta('strands-agents/harness-sdk', 'harness-typescript/v0.3.0')).toEqual({
      sdk: 'harness',
      language: 'typescript',
      version: '0.3.0',
    })
    expect(tagToMeta('strands-agents/harness-sdk', 'harness-cli/v0.2.1')).toEqual({ sdk: 'cli', version: '0.2.1' })
  })

  it('mcp package tags map to the mcp stream', () => {
    expect(tagToMeta('strands-agents/harness-sdk', 'mcp/v0.3.0')).toEqual({ sdk: 'mcp', version: '0.3.0' })
  })

  it('archived mcp-server repo bare v tags map to mcp', () => {
    expect(tagToMeta('strands-agents/mcp-server', 'v0.2.7')).toEqual({
      sdk: 'mcp',
      language: undefined,
      version: '0.2.7',
    })
  })

  it('shell and agent-sop bare v tags have no language', () => {
    expect(tagToMeta('strands-agents/shell', 'v0.5.0')).toEqual({ sdk: 'shell', language: undefined, version: '0.5.0' })
    expect(tagToMeta('strands-agents/agent-sop', 'v1.1.4')).toEqual({
      sdk: 'agent-sop',
      language: undefined,
      version: '1.1.4',
    })
    expect(tagToMeta('strands-agents/agent-sop', '0.0.2')).toBe(null)
  })

  it('box bare v tag has no language', () => {
    expect(tagToMeta('strands-agents/box', 'v0.1.0')).toEqual({ sdk: 'box', language: undefined, version: '0.1.0' })
    expect(tagToMeta('strands-agents/box', 'nightly')).toBe(null)
  })

  it('package urls', () => {
    expect(getPackageUrl('sdk', 'python', '1.42.0')).toBe('https://pypi.org/project/strands-agents/1.42.0/')
    expect(getPackageUrl('sdk', 'typescript', '1.4.0')).toBe(
      'https://www.npmjs.com/package/@strands-agents/sdk/v/1.4.0'
    )
    expect(getPackageUrl('evals', undefined, '0.2.1')).toBe('https://pypi.org/project/strands-agents-evals/0.2.1/')
    expect(getPackageUrl('harness', 'python', '0.3.1')).toBe('https://pypi.org/project/strands-harness/0.3.1/')
    expect(getPackageUrl('harness', 'typescript', '0.3.0')).toBe(
      'https://www.npmjs.com/package/@strands-agents/harness/v/0.3.0'
    )
    expect(getPackageUrl('cli', undefined, '0.2.1')).toBe('https://www.npmjs.com/package/@strands-agents/cli/v/0.2.1')
    expect(getPackageUrl('box', undefined, '0.1.0')).toBe('https://github.com/strands-agents/box/releases/tag/v0.1.0')
    expect(getPackageUrl('shell', undefined, '0.5.0')).toBe(
      'https://github.com/strands-agents/shell/releases/tag/v0.5.0'
    )
    expect(getPackageUrl('mcp', undefined, '0.3.0')).toBe('https://pypi.org/project/strands-agents-mcp-server/0.3.0/')
    expect(getPackageUrl('agent-sop', undefined, '1.1.4')).toBe('https://pypi.org/project/strands-agents-sops/1.1.4/')
  })
})
