export const SDK_LANGUAGES = ['python', 'typescript', 'rust'] as const
export type SdkLanguage = (typeof SDK_LANGUAGES)[number]

export const DEFAULT_DOC_LANGUAGES = SDK_LANGUAGES

export const SDK_LANGUAGE_LABELS = {
  python: 'Python',
  typescript: 'TypeScript',
  rust: 'Rust',
} as const satisfies Record<SdkLanguage, string>

export type SdkLanguageLabel = (typeof SDK_LANGUAGE_LABELS)[SdkLanguage]

export type SdkLanguageAvailability = 'available' | 'undocumented' | 'unsupported'

export const SDK_LANGUAGE_LABEL_LIST = SDK_LANGUAGES.map(
  (language) => SDK_LANGUAGE_LABELS[language]
) as SdkLanguageLabel[]

export function isSdkLanguage(value: string): value is SdkLanguage {
  return (SDK_LANGUAGES as readonly string[]).includes(value)
}

export function normalizeSdkLanguage(value: string): SdkLanguage | undefined {
  const normalized = value.trim().toLowerCase()
  return isSdkLanguage(normalized) ? normalized : undefined
}

export function normalizeSdkLanguages(values: string | readonly string[]): SdkLanguage[] {
  const normalized = (Array.isArray(values) ? values : [values])
    .map(normalizeSdkLanguage)
    .filter((language): language is SdkLanguage => language !== undefined)
  return [...new Set(normalized)]
}

export function sdkLanguageLabel(language: SdkLanguage): SdkLanguageLabel {
  return SDK_LANGUAGE_LABELS[language]
}

export function sdkLanguageFromLabel(label: string): SdkLanguage | undefined {
  const normalized = label.trim().toLowerCase()
  return SDK_LANGUAGES.find((language) => SDK_LANGUAGE_LABELS[language].toLowerCase() === normalized)
}

export function sdkLanguageAvailability(
  language: SdkLanguage,
  documentedLanguages: readonly SdkLanguage[],
  unsupportedLanguages: readonly SdkLanguage[],
  hasSwitchTarget = false
): SdkLanguageAvailability {
  if (unsupportedLanguages.includes(language)) return 'unsupported'
  if (documentedLanguages.includes(language) || hasSwitchTarget) return 'available'
  return 'undocumented'
}
