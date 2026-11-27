import { describe, it, expect } from 'vitest'
import { docsLanguagesSchema } from '../src/content.config'

describe('docsLanguagesSchema', () => {
  it('accepts a single language string', () => {
    expect(docsLanguagesSchema.safeParse('python').success).toBe(true)
    expect(docsLanguagesSchema.safeParse('typescript').success).toBe(true)
    expect(docsLanguagesSchema.safeParse('rust').success).toBe(true)
  })

  it('accepts a single-element array', () => {
    expect(docsLanguagesSchema.safeParse(['python']).success).toBe(true)
    expect(docsLanguagesSchema.safeParse(['typescript']).success).toBe(true)
    expect(docsLanguagesSchema.safeParse(['rust']).success).toBe(true)
  })

  it('accepts undefined (field omitted)', () => {
    expect(docsLanguagesSchema.safeParse(undefined).success).toBe(true)
  })

  it('accepts explicit language subsets and all supported languages', () => {
    expect(docsLanguagesSchema.safeParse(['python', 'typescript']).success).toBe(true)
    expect(docsLanguagesSchema.safeParse(['python', 'typescript', 'rust']).success).toBe(true)
  })

  it('accepts language names regardless of case', () => {
    expect(docsLanguagesSchema.safeParse(['Python', 'TypeScript', 'Rust']).success).toBe(true)
  })

  it('rejects unknown languages', () => {
    expect(docsLanguagesSchema.safeParse('go').success).toBe(false)
  })

  it('rejects duplicate languages', () => {
    expect(docsLanguagesSchema.safeParse(['Rust', 'rust']).success).toBe(false)
  })
})
