import { describe, expect, it } from 'vitest'

import { languageForReleaseAnchor, packageRegistryLabel } from '../src/config/changelog'

describe('changelog configuration', () => {
  it('labels package registries for every SDK language', () => {
    expect(packageRegistryLabel('python', 'https://pypi.org/project/strands-agents/1.0.0/')).toBe('PyPI')
    expect(packageRegistryLabel('typescript', 'https://www.npmjs.com/package/@strands-agents/sdk/v/1.0.0')).toBe('npm')
    expect(packageRegistryLabel('rust', 'https://crates.io/crates/strands/1.0.0')).toBe('crates.io')
  })

  it('selects Rust for a Harness release deep link', () => {
    expect(languageForReleaseAnchor('harness', 'rust')).toBe('rust')
    expect(languageForReleaseAnchor('sdk', 'rust')).toBe('rust')
    expect(languageForReleaseAnchor('evals', 'rust')).toBeUndefined()
  })
})
