import { readFileSync } from 'node:fs'

import { getCollection } from 'astro:content'
import { JSDOM } from 'jsdom'
import { parse } from 'node-html-parser'
import { describe, expect, it } from 'vitest'
import { DEFAULT_DOC_LANGUAGES, normalizeSdkLanguages, type SdkLanguage } from '../src/config/languages'
import { bindFooterLanguage } from '../src/util/footer'
import { LANGUAGE_STORAGE_KEY } from '../src/util/language-preference'

const REQUIRED_RUST_PAGES = [
  'docs/api/rust',
  'docs/contribute/contributing/core-sdk',
  'docs/contribute/contributing/documentation',
  'docs/integrations/get-featured',
  'docs/user-guide/harness',
  'docs/user-guide/harness/quickstart',
  'docs/user-guide/sdk',
  'docs/user-guide/sdk/agents',
  'docs/user-guide/sdk/observability-evaluation',
  'docs/user-guide/sdk/quickstart/overview',
  'docs/user-guide/sdk/quickstart/rust',
  'docs/user-guide/sdk/streaming',
  'docs/user-guide/sdk/tools',
]

function documentedLanguages(data: {
  languages?: string | string[]
  unsupportedLanguages?: string | string[]
}): SdkLanguage[] {
  const unsupported = normalizeSdkLanguages(data.unsupportedLanguages ?? [])
  return data.languages
    ? normalizeSdkLanguages(data.languages)
    : DEFAULT_DOC_LANGUAGES.filter((language) => !unsupported.includes(language))
}

function directTabLabels(source: string): string[][] {
  const tabs: string[][] = []
  const stack: { labels: string[]; nestedDepth: number }[] = []
  const tagPattern = /<\/?Tabs\b[^>]*>|<Tab\b[^>]*\blabel=['"]([^'"]+)['"][^>]*>/g

  for (const match of source.matchAll(tagPattern)) {
    const tag = match[0]
    if (tag.startsWith('<Tabs')) {
      for (const group of stack) group.nestedDepth += 1
      stack.push({ labels: [], nestedDepth: 0 })
      continue
    }
    if (tag.startsWith('</Tabs')) {
      const group = stack.pop()
      if (group) tabs.push(group.labels)
      for (const parent of stack) parent.nestedDepth -= 1
      continue
    }

    const current = stack.at(-1)
    if (current?.nestedDepth === 0 && match[1]) current.labels.push(match[1])
  }

  return tabs
}

describe('Rust documentation language coverage', () => {
  it('keeps nested provider tabs visible inside language tabs', () => {
    const overview = parse(
      readFileSync(new URL('../dist/docs/user-guide/sdk/quickstart/overview/index.html', import.meta.url), 'utf8')
    )
    const nestedProviderTabs = overview.querySelectorAll(
      '[data-language-tabs] starlight-tabs starlight-tabs > .tablist-wrapper'
    )
    const componentSource = readFileSync(new URL('../src/components/AutoSyncTabs.astro', import.meta.url), 'utf8')

    expect(nestedProviderTabs.length).toBeGreaterThan(0)
    expect(nestedProviderTabs.some((tabs) => tabs.textContent.includes('Amazon Bedrock'))).toBe(true)
    expect(componentSource).toContain('[data-language-tabs] > starlight-tabs > .tablist-wrapper')
  })

  it('keeps counterpart routes enabled in the rendered language selector', () => {
    const pythonQuickstart = parse(
      readFileSync(new URL('../dist/docs/user-guide/sdk/quickstart/python/index.html', import.meta.url), 'utf8')
    )
    const rustQuickstart = parse(
      readFileSync(new URL('../dist/docs/user-guide/sdk/quickstart/rust/index.html', import.meta.url), 'utf8')
    )

    const rustTarget = pythonQuickstart.querySelector<HTMLButtonElement>('language-toggle [data-lang="Rust"]')
    expect(rustTarget?.hasAttribute('disabled')).toBe(false)
    expect(rustTarget?.getAttribute('data-switch-href')).toContain('/docs/user-guide/sdk/quickstart/rust/')

    for (const language of ['Python', 'TypeScript']) {
      const target = rustQuickstart.querySelector<HTMLButtonElement>(`language-toggle [data-lang="${language}"]`)
      expect(target?.hasAttribute('disabled'), `${language} counterpart is disabled`).toBe(false)
      expect(target?.getAttribute('data-switch-href'), `${language} counterpart is missing`).toBeTruthy()
    }
  })

  it('distinguishes unsupported features from undocumented Rust pages', () => {
    const unsupported = parse(
      readFileSync(new URL('../dist/docs/integrations/model-providers/vllm/index.html', import.meta.url), 'utf8')
    )
    const undocumented = parse(
      readFileSync(
        new URL('../dist/docs/user-guide/sdk/streaming/streaming-to-the-browser-guide/index.html', import.meta.url),
        'utf8'
      )
    )

    const unsupportedRust = unsupported.querySelector<HTMLButtonElement>('language-toggle [data-lang="Rust"]')
    const undocumentedRust = undocumented.querySelector<HTMLButtonElement>('language-toggle [data-lang="Rust"]')

    expect(unsupportedRust?.getAttribute('data-language-availability')).toBe('unsupported')
    expect(unsupportedRust?.hasAttribute('disabled')).toBe(false)
    expect(unsupportedRust?.getAttribute('title')).toBe('Rust: This SDK does not implement this feature')
    expect(undocumentedRust?.getAttribute('data-language-availability')).toBe('undocumented')
    expect(undocumentedRust?.hasAttribute('disabled')).toBe(false)
    expect(undocumentedRust?.getAttribute('title')).toBe(
      'Rust: This page does not have documentation in this language yet'
    )

    const unsupportedRustAside = unsupported.querySelector(
      '[data-language-support-for="rust"] aside[aria-label="Rust availability"]'
    )
    expect(unsupportedRustAside?.textContent).toContain('The Rust SDK does not implement this feature.')
    expect(unsupportedRustAside?.textContent).toContain('Choose Python in the language selector')

    const undocumentedSupportAsides = undocumented.querySelectorAll('[data-language-support-for] aside')
    expect(undocumentedSupportAsides).toHaveLength(2)
    expect(undocumentedSupportAsides.every((aside) => !aside.textContent.includes('verified'))).toBe(true)
    expect(undocumented.querySelector('[data-language-support-for="python"]')?.textContent).toContain(
      'This page does not have Python documentation yet.'
    )
    expect(undocumented.querySelector('[data-language-support-for="rust"]')?.textContent).toContain(
      'This page does not have Rust documentation yet.'
    )
  })

  it('preserves an unavailable stored language on ordinary docs pages', () => {
    const fixtures = [
      {
        pathname: '/docs/integrations/model-providers/vllm/',
        availability: 'unsupported',
      },
      {
        pathname: '/docs/user-guide/sdk/streaming/streaming-to-the-browser-guide/',
        availability: 'undocumented',
      },
    ]

    for (const { pathname, availability } of fixtures) {
      const html = readFileSync(new URL(`../dist${pathname}index.html`, import.meta.url), 'utf8')
      const dom = new JSDOM(html, {
        runScripts: 'outside-only',
        url: `https://example.test${pathname}`,
      })
      const { window } = dom
      const script = [...window.document.querySelectorAll<HTMLScriptElement>('script:not([src])')].find((candidate) =>
        candidate.textContent.includes('__strandsLanguageToggleInit')
      )

      expect(script, `${pathname}: language toggle script is missing`).toBeDefined()
      window.localStorage.setItem(LANGUAGE_STORAGE_KEY, 'Rust')
      window.eval(script!.textContent)

      expect(window.document.documentElement.dataset.sdkLanguage).toBe('rust')
      expect(window.document.documentElement.dataset.sdkLanguageAvailability).toBe(availability)

      window.document.dispatchEvent(new window.Event('DOMContentLoaded'))

      expect(window.localStorage.getItem(LANGUAGE_STORAGE_KEY)).toBe('Rust')
      expect(window.document.documentElement.dataset.sdkLanguage).toBe('rust')
      expect(window.document.documentElement.dataset.sdkLanguageAvailability).toBe(availability)
      expect(
        [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="Rust"]')].every(
          (button) => button.disabled === false && button.getAttribute('aria-checked') === 'true'
        )
      ).toBe(true)
      expect(window.document.querySelector('[data-language-page-content]')).not.toBeNull()
      expect(
        window.document.querySelector('[data-language-support-for="rust"]')?.closest('[data-language-page-content]')
      ).toBeNull()
    }

    const markdownContentSource = readFileSync(
      new URL('../src/components/overrides/MarkdownContent.astro', import.meta.url),
      'utf8'
    )
    expect(markdownContentSource).toContain("html[data-sdk-language-availability='unsupported']")
    expect(markdownContentSource).toContain("html[data-sdk-language-availability='undocumented']")
    expect(markdownContentSource).toContain('[data-language-page-content]')
  })

  it('preserves the stored language when opening a language-specific route', () => {
    const html = readFileSync(
      new URL('../dist/docs/user-guide/sdk/quickstart/rust/index.html', import.meta.url),
      'utf8'
    )
    const dom = new JSDOM(html, {
      runScripts: 'outside-only',
      url: 'https://example.test/docs/user-guide/sdk/quickstart/rust/',
    })
    const { window } = dom
    const script = [...window.document.querySelectorAll<HTMLScriptElement>('script:not([src])')].find((candidate) =>
      candidate.textContent.includes('__strandsLanguageToggleInit')
    )

    expect(script, 'language toggle script is missing').toBeDefined()
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, 'TypeScript')
    const unbindFooter = bindFooterLanguage(window.document, window, 'TypeScript')
    window.eval(script!.textContent)
    window.document.dispatchEvent(new window.Event('DOMContentLoaded'))

    expect(window.document.documentElement.dataset.sdkLanguage).toBe('typescript')
    expect(window.document.documentElement.dataset.langPref).toBe('typescript')
    expect(
      [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="Rust"]')].every(
        (button) => button.getAttribute('aria-checked') === 'false'
      )
    ).toBe(true)
    expect(
      [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="TypeScript"]')].every(
        (button) => button.disabled === false && button.getAttribute('aria-checked') === 'true'
      )
    ).toBe(true)
    expect(window.localStorage.getItem(LANGUAGE_STORAGE_KEY)).toBe('TypeScript')
    const releases = [...window.document.querySelectorAll<HTMLElement>('.site-footer [data-release-lang]')]
    expect(releases.filter((release) => !release.hidden).map((release) => release.dataset.releaseLang)).toEqual([
      'TypeScript',
    ])
    unbindFooter()
  })

  it('preserves the stored language on a language-specific route without a counterpart', () => {
    const pathname = '/docs/user-guide/sdk/deploy/deploy_to_bedrock_agentcore/python/'
    const html = readFileSync(new URL(`../dist${pathname}index.html`, import.meta.url), 'utf8')
    const dom = new JSDOM(html, {
      runScripts: 'outside-only',
      url: `https://example.test${pathname}`,
    })
    const { window } = dom
    const script = [...window.document.querySelectorAll<HTMLScriptElement>('script:not([src])')].find((candidate) =>
      candidate.textContent.includes('__strandsLanguageToggleInit')
    )

    expect(script, 'language toggle script is missing').toBeDefined()
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, 'Rust')
    window.eval(script!.textContent)
    window.document.dispatchEvent(new window.Event('DOMContentLoaded'))

    const rustOptions = [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="Rust"]')]
    expect(window.localStorage.getItem(LANGUAGE_STORAGE_KEY)).toBe('Rust')
    expect(window.document.documentElement.dataset.sdkLanguage).toBe('rust')
    expect(rustOptions.every((button) => button.getAttribute('aria-checked') === 'true')).toBe(true)
    expect(rustOptions.every((button) => button.dataset.switchHref === undefined)).toBe(true)
  })

  it('switches inline syntax to the effective Rust language', async () => {
    const html = readFileSync(new URL('../dist/docs/user-guide/harness/quickstart/index.html', import.meta.url), 'utf8')
    const dom = new JSDOM(html, {
      runScripts: 'outside-only',
      url: 'https://example.test/docs/user-guide/harness/quickstart/',
    })
    const { window } = dom
    const inlineScript = (marker: string) =>
      [...window.document.querySelectorAll<HTMLScriptElement>('script:not([src])')].find((candidate) =>
        candidate.textContent.includes(marker)
      )?.textContent
    const languageScript = inlineScript('__strandsLanguageToggleInit')
    const syntaxScript = inlineScript('__strandsSyntaxSwitchInit')

    expect(languageScript, 'language toggle script is missing').toBeDefined()
    expect(syntaxScript, 'syntax switch script is missing').toBeDefined()
    if (window.document.readyState === 'loading') {
      await new Promise<void>((resolve) =>
        window.document.addEventListener('DOMContentLoaded', () => resolve(), { once: true })
      )
    }
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, 'TypeScript')
    window.eval(languageScript!)
    window.eval(syntaxScript!)

    const syntaxSwitch = [...window.document.querySelectorAll<HTMLElement>('syntax-switch')].find((candidate) =>
      candidate.textContent.includes('provider/name')
    )
    const visibleVariant = () =>
      [...(syntaxSwitch?.querySelectorAll<HTMLElement>('[data-lang]') ?? [])].find(
        (variant) => variant.style.display === 'inline'
      )

    expect(visibleVariant()?.dataset.lang).toBe('TypeScript')

    window.dispatchEvent(new window.CustomEvent('lang-change', { detail: 'Rust' }))
    await Promise.resolve()

    expect(window.document.documentElement.dataset.sdkLanguage).toBe('rust')
    expect(visibleVariant()?.dataset.lang).toBe('Rust')
    expect(visibleVariant()?.textContent).toBe('.model("provider/name")')
  })

  it('keeps the Rust launch path enabled', async () => {
    const docs = await getCollection('docs')
    const languagesById = new Map(docs.map((doc) => [doc.id, documentedLanguages(doc.data)]))

    for (const id of REQUIRED_RUST_PAGES) {
      expect(languagesById.has(id), `${id}: page is missing`).toBe(true)
      expect(languagesById.get(id), `${id}: Rust is missing from languages`).toContain('rust')
    }
  })

  it('keeps Rust-enabled pages internally consistent', async () => {
    const docs = await getCollection('docs')
    const offenders: string[] = []

    for (const doc of docs) {
      const languages = documentedLanguages(doc.data)
      const rustEnabled = languages.includes('rust')
      const rustRoute = doc.id.split('/').includes('rust')

      if (rustRoute && !rustEnabled) {
        offenders.push(`${doc.id}: Rust route must declare Rust in languages`)
      }

      const body = doc.body ?? ''
      const hasRustVariant =
        /label=['"]Rust['"]/.test(body) ||
        /<Syntax\b[\s\S]*?\brs\s*=/.test(body) ||
        /<CopyPromptButton\b[^>]*\blanguage=['"]rust['"]/.test(body)

      if (hasRustVariant && !rustEnabled) {
        offenders.push(`${doc.id}: Rust content requires Rust in languages`)
      }
      if (!rustEnabled) continue

      for (const labels of directTabLabels(body)) {
        const languageLabels = labels.filter((label) => ['Python', 'TypeScript', 'Rust'].includes(label))
        if (
          languageLabels.length === labels.length &&
          languageLabels.some((label) => label === 'Python' || label === 'TypeScript') &&
          !languageLabels.includes('Rust')
        ) {
          offenders.push(`${doc.id}: language tab group is missing Rust`)
        }
      }

      const syntaxTags = body.match(/<Syntax\b[\s\S]*?>/g) ?? []
      for (const syntaxTag of syntaxTags) {
        if (!/\brs\s*=/.test(syntaxTag)) {
          offenders.push(`${doc.id}: <Syntax> is missing rs`)
        }
      }
    }

    expect(offenders, offenders.join('\n')).toEqual([])
  })

  it('classifies every remaining Rust language-tab gap', async () => {
    const docs = await getCollection('docs')
    const offenders: string[] = []

    for (const doc of docs) {
      const inRustRollout =
        doc.id === 'docs/user-guide/sdk' ||
        doc.id.startsWith('docs/user-guide/sdk/') ||
        doc.id === 'docs/user-guide/harness' ||
        doc.id.startsWith('docs/user-guide/harness/')
      if (!inRustRollout) continue

      const body = doc.body ?? ''
      const hasMissingRustGroup = directTabLabels(body).some(
        (labels) => (labels.includes('Python') || labels.includes('TypeScript')) && !labels.includes('Rust')
      )
      if (!hasMissingRustGroup) continue

      const languages = documentedLanguages(doc.data)
      const unsupported = doc.data.unsupportedLanguages ? normalizeSdkLanguages(doc.data.unsupportedLanguages) : []
      const explicitlyDocumented = doc.data.languages ? normalizeSdkLanguages(doc.data.languages) : undefined

      if (languages.includes('rust') || unsupported.includes('rust')) continue
      if (explicitlyDocumented && !explicitlyDocumented.includes('rust')) continue

      offenders.push(`${doc.id}: classify Rust as documented, undocumented, or unsupported`)
    }

    expect(offenders, offenders.join('\n')).toEqual([])
  })
})
