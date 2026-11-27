import { describe, it, expect } from 'vitest'
import { docsLanguagesSchema, docsLanguageSupportSchema, docsUnsupportedLanguagesSchema } from '../src/content.config'
import { sdkLanguageAvailability } from '../src/config/languages'

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

describe('docsUnsupportedLanguagesSchema', () => {
  it('accepts known language names', () => {
    expect(docsUnsupportedLanguagesSchema.safeParse('Rust').success).toBe(true)
    expect(docsUnsupportedLanguagesSchema.safeParse(['Python', 'Rust']).success).toBe(true)
  })

  it('rejects unknown and duplicate language names', () => {
    expect(docsUnsupportedLanguagesSchema.safeParse('go').success).toBe(false)
    expect(docsUnsupportedLanguagesSchema.safeParse(['Rust', 'rust']).success).toBe(false)
  })

  it('cannot overlap documented languages', () => {
    expect(
      docsLanguageSupportSchema.safeParse({
        languages: ['Python', 'TypeScript', 'Rust'],
        unsupportedLanguages: 'Rust',
      }).success
    ).toBe(false)
  })

  it('treats omitted languages as the Python and TypeScript default', () => {
    expect(
      docsLanguageSupportSchema.safeParse({
        unsupportedLanguages: 'Rust',
      }).success
    ).toBe(true)
    expect(
      docsLanguageSupportSchema.safeParse({
        unsupportedLanguages: 'Python',
      }).success
    ).toBe(false)
  })
})

describe('sdkLanguageAvailability', () => {
  it('distinguishes documented, unsupported, and undocumented languages', () => {
    const documented = ['python', 'typescript'] as const

    expect(sdkLanguageAvailability('python', documented, [])).toBe('available')
    expect(sdkLanguageAvailability('rust', documented, ['rust'])).toBe('unsupported')
    expect(sdkLanguageAvailability('rust', documented, [])).toBe('undocumented')
  })

  it('treats a counterpart route as available', () => {
    expect(sdkLanguageAvailability('rust', ['python'], [], true)).toBe('available')
  })
})
