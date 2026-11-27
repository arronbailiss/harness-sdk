export function showFooterLanguage(root: ParentNode, lang: string): void {
  const releases = root.querySelectorAll<HTMLElement>('.site-footer [data-release-lang]')
  const match = [...releases].some((release) => release.dataset.releaseLang === lang)

  releases.forEach((release) => {
    release.hidden = match && release.dataset.releaseLang !== lang
  })
}

export function bindFooterLanguage(root: ParentNode, target: EventTarget, initialLanguage: string): () => void {
  const update = (event: Event) => {
    const language = (event as CustomEvent<unknown>).detail
    if (typeof language === 'string') showFooterLanguage(root, language)
  }

  showFooterLanguage(root, initialLanguage)
  target.addEventListener('lang-change', update)
  target.addEventListener('lang-effective-change', update)

  return () => {
    target.removeEventListener('lang-change', update)
    target.removeEventListener('lang-effective-change', update)
  }
}
