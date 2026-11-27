import { readFileSync } from 'node:fs'

import { getCollection } from 'astro:content'
import { JSDOM } from 'jsdom'
import { parse } from 'node-html-parser'
import { describe, expect, it } from 'vitest'
import { normalizeSdkLanguages } from '../src/config/languages'
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

  it('keeps a Rust-only route authoritative over a stored language preference', () => {
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

    expect(window.document.documentElement.dataset.sdkLanguage).toBe('rust')
    expect(window.document.documentElement.dataset.langPref).toBe('rust')
    expect(
      [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="Rust"]')].every(
        (button) => button.getAttribute('aria-checked') === 'true'
      )
    ).toBe(true)
    expect(
      [...window.document.querySelectorAll<HTMLButtonElement>('language-toggle [data-lang="TypeScript"]')].every(
        (button) => button.disabled === false && button.getAttribute('aria-checked') === 'false'
      )
    ).toBe(true)
    expect(window.localStorage.getItem(LANGUAGE_STORAGE_KEY)).toBe('TypeScript')
    expect(
      [...window.document.querySelectorAll<HTMLElement>('.site-footer [data-release-lang]')].every(
        (release) => !release.hidden
      )
    ).toBe(true)
    unbindFooter()
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
    const languagesById = new Map(
      docs.map((doc) => [doc.id, doc.data.languages ? normalizeSdkLanguages(doc.data.languages) : []])
    )

    for (const id of REQUIRED_RUST_PAGES) {
      expect(languagesById.has(id), `${id}: page is missing`).toBe(true)
      expect(languagesById.get(id), `${id}: Rust is missing from languages`).toContain('rust')
    }
  })

  it('keeps Rust-enabled pages internally consistent', async () => {
    const docs = await getCollection('docs')
    const offenders: string[] = []

    for (const doc of docs) {
      const languages = doc.data.languages ? normalizeSdkLanguages(doc.data.languages) : []
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

      const hasPythonOrTypeScriptTabs = /label=['"]Python['"]/.test(body) || /label=['"]TypeScript['"]/.test(body)
      if (hasPythonOrTypeScriptTabs && !/label=['"]Rust['"]/.test(body)) {
        offenders.push(`${doc.id}: language tabs are missing Rust`)
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
})
