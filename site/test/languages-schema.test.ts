import { describe, it, expect } from 'vitest'
import { docsLanguagesSchema, docsLanguageSupportSchema, docsUnsupportedLanguagesSchema } from '../src/content.config'
import { DEFAULT_DOC_LANGUAGES, sdkLanguageAvailability } from '../src/config/languages'

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

  it('accepts explicit language subsets', () => {
    expect(docsLanguagesSchema.safeParse(['python', 'typescript']).success).toBe(true)
  })

  it('rejects a redundant list of every default language', () => {
    const result = docsLanguagesSchema.safeParse(['python', 'typescript', 'rust'])
    expect(result.success).toBe(false)
    if (!result.success) {
      expect(result.error.issues[0].message).toContain('omit languages')
    }
  })

  it('accepts language names regardless of case', () => {
    expect(docsLanguagesSchema.safeParse(['Python', 'Rust']).success).toBe(true)
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
        languages: ['Python', 'TypeScript'],
        unsupportedLanguages: 'TypeScript',
      }).success
    ).toBe(false)
  })

  it('uses all SDK languages by default and removes explicit unsupported languages', () => {
    expect(DEFAULT_DOC_LANGUAGES).toEqual(['python', 'typescript', 'rust'])
    expect(
      docsLanguageSupportSchema.safeParse({
        unsupportedLanguages: 'Rust',
      }).success
    ).toBe(true)
    expect(
      docsLanguageSupportSchema.safeParse({
        unsupportedLanguages: 'Python',
      }).success
    ).toBe(true)
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

  it('keeps unsupported status even when a counterpart route exists', () => {
    expect(sdkLanguageAvailability('rust', ['python'], ['rust'], true)).toBe('unsupported')
  })
})
