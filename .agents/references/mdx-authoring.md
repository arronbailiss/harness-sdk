# MDX Authoring Patterns

Practical reference for writing documentation in this repo. This site uses Astro + Starlight with MDX.

## Auto-Imported Components

`<Tabs>` and `<Tab>` are globally available — no import statement needed. Tabs with matching labels auto-sync across the page.

```mdx
<Tabs>
  <Tab label="Python">
    ```python
    from strands import Agent
    agent = Agent(tools=[my_tool])
    ```
  </Tab>
  <Tab label="TypeScript">
    ```typescript
    --8<-- "user-guide/concepts/agents/example.ts:basic_agent"
    ```
  </Tab>
  <Tab label="Rust">
    ```rust
    --8<-- "user-guide/concepts/agents/example.rs:basic_agent"
    ```
  </Tab>
</Tabs>
```

`<Syntax>` is also globally available. Use it when shared prose needs to reference a language-specific identifier — any method name, parameter, class name, or short expression that differs between SDKs:

```mdx
Use <Syntax py=".as_tool()" ts=".asTool()" rs=".as_tool()" /> to customize the tool name.
The <Syntax py="retry_strategy" ts="retryStrategy" rs="retry_strategy" /> parameter controls retry behavior.
```

Renders as inline `<code>` by default. Pass `plain` for plain text. Reacts live to the global language toggle — no page reload.

Include `py`, `ts`, and `rs` on shared pages. A page with an explicit language
subset may omit the variants it does not cover. Do not rely on fallback text for
an SDK-specific identifier.

**When to use `<Syntax>` vs `<Tabs>`:**
- `<Syntax>`: single identifier or short expression that differs by language, embedded in a prose sentence.
- `<Tabs>`: code blocks, multi-line examples, or content that's structurally different between languages.

Never spell out both language variants manually in prose. Use `<Syntax>` instead.

For other Starlight components (`Aside`, `Card`, `CardGrid`, `LinkCard`, `Icon`, `Badge`), use explicit imports:

```mdx
import { Card, CardGrid } from '@astrojs/starlight/components';
```

## Snippet Inclusion

Code lives in runnable source files; the MDX page references named regions of those files. Snippet source files live alongside the page that includes them, with the same base name (`agent-loop.mdx` pulls from `agent-loop.ts`). **Imports go in a sibling `*_imports.ts` file**; the body lives in the main `.ts` file. The MDX block pulls both — imports first, then body — so a single rendered code block shows imports + usage.

Snippet names use `snake_case` (build-time identifiers, not source code) and follow `<feature>_<variant>` for bodies, plus `_imports` suffix for the matching import set.

**1. Imports file** — one snippet per import set, named `<name>_imports`. Add `// @ts-nocheck` at the top *only* when the same identifier is intentionally re-imported across multiple snippets in this file:

```typescript
// traces_imports.ts
// --8<-- [start:code_configuration_option1_imports]
import { Agent } from '@strands-agents/sdk'
// --8<-- [end:code_configuration_option1_imports]
```

**2. Body file** — runnable code, body snippets wrapped in functions for scoping (see "TypeScript Snippet Scoping" below):

```typescript
// traces.ts
async function codeConfigurationOption1() {
  // --8<-- [start:code_configuration_option1]
  const agent = new Agent({
    systemPrompt: 'You are a helpful AI assistant',
  })
  // --8<-- [end:code_configuration_option1]
}
```

**3. MDX page** — pull both into **one** code block. Both `--8<--` directives go inside a single ` ```typescript ` fence, separated by a blank line. The reader sees imports above the body in one continuous, copy-pasteable code box:

````markdown
```typescript
--8<-- "user-guide/observability-evaluation/traces_imports.ts:code_configuration_option1_imports"

--8<-- "user-guide/observability-evaluation/traces.ts:code_configuration_option1"
```
````

**Common mistake — do not** wrap each include in its own fence. Two fences render as two disconnected code blocks with a visible gap, which breaks the copy-paste-as-one-unit affordance:

````markdown
<!-- WRONG: renders as two disconnected blocks -->
```typescript
--8<-- "user-guide/observability-evaluation/traces_imports.ts:code_configuration_option1_imports"
```

```typescript
--8<-- "user-guide/observability-evaluation/traces.ts:code_configuration_option1"
```
````

Only the code between markers is rendered. Paths inside the `--8<--` directive are always resolved relative to `site/src/content/docs/`.

**When to skip the imports file:** the page renders no TypeScript snippets, *or* every snippet body genuinely has zero external imports. Otherwise, write the imports file — relying on the imports at the top of the body `.ts` is wrong, since those lines live above the `[start:...]` marker and never appear in rendered docs.

Rust snippets use `.rs` files and the same named-region markers. Keep each
page-specific snippet beside the page that includes it. The `site/rust-snippets`
crate discovers and compiles every `.rs` file under `site/src/content/docs/`, so
do not add documentation-only examples to a published SDK crate or register a
Cargo example per page. Include the `use` declarations required by the rendered
block inside its region. A sequence of examples may build on a helper or type
that the page already showed, but it must not depend on setup that is hidden
from the reader. Wrappers used only to give a focused fragment an async or
fallible compilation context may stay outside the region. The Cargo gate
compiles each complete sibling source file, not every extracted region as a
standalone crate.

Document every direct crate and optional Cargo feature used by a rendered Rust
region in that page's Rust tab. Do not rely on the `rust-snippets` crate's
dependency superset or on an SDK's transitive dependencies: those can make the
site check pass while copied code fails in a reader's project. Verify a
standalone example in a scratch crate containing only the dependencies and
features the page tells readers to add. Every source file must also compile
through `npm run typecheck:snippets`.

## Callout Syntax

Starlight-native admonitions. **Use sparingly** — callouts are visually loud and reset the reader's flow. In most cases inline prose is the better choice; reach for a callout only when the information genuinely needs to break frame. Overusing `caution`/`danger` in particular trains readers to ignore them.

When you do use one, pick the level that matches consequence:

- `:::note` — context the reader needs but might miss; non-actionable. *Use for:* clarifications, links to related concepts.
- `:::tip` — an optional improvement or shortcut. *Use for:* performance hints, idiomatic alternatives, "you can also…" suggestions.
- `:::caution` — something that will work but has a non-obvious gotcha or cost. Strongest when the failure is silent or hard to diagnose (wrong-but-not-broken output, a config that degrades behavior without erroring) and the reader could plausibly hit it on the happy path.
- `:::danger` — even rarer. Only for actions that destroy work, lose data, or have no recovery path.

```markdown
:::note[Optional Title]
Informational content.
:::

:::tip
Helpful suggestion.
:::

:::caution
Proceed carefully.
:::

:::danger[Breaking Change]
This will break existing code.
:::
```

## Frontmatter Schema

Required fields:

```yaml
---
title: "Page Title"
description: "Short description for SEO (140-160 chars)"
---
```

Optional fields (validated by Zod in `site/src/content.config.ts`):

| Field | Type | Purpose |
|-------|------|---------|
| `languages` | `string \| string[]` | Explicit subset with documentation on this page |
| `unsupportedLanguages` | `string \| string[]` | SDKs that do not implement the documented feature |
| `community` | `boolean` | Marks page as community-contributed |
| `experimental` | `boolean` | Marks feature as experimental |
| `externalDocs` | object | Generated provenance and publication status for imported docs; authors must not set it |
| `integrationType` | enum | `model-provider`, `decision-model`, `tool`, `session-manager`, `memory-store`, `storage`, `integration`, `plugin`, `observability`, `agent-extension`, `intervention` |
| `category` | `string` | For TypeScript API doc grouping |
| `redirectFrom` | `string[]` | Old slugs that should redirect here |
| `tags` | `Tag[]` | From `site/src/config/tags.yml`; drives the build-time "Related pages" block |
| `sourceLinks` | `{repo, path}[]` | Pointers to SDK implementation; rendered on headless surfaces (index.md, llms-full.txt) |

The relevant fields render contextual banners automatically (experimental → external preview → community → languages); community pages suppress the separate language banner. Anything not in this table is silently stripped by Zod at build time, so don't invent fields like `contentType` or `lastReviewed` — add them to the schema first if they'd be useful.

Omitting `languages` means the page covers the default Python, TypeScript, and
Rust SDKs. Do not add a redundant three-language list.

Use a single value or a smaller list when documentation is available only for
those SDKs.

Use `unsupportedLanguages` only when the feature itself is unavailable in an
SDK. A language omitted from both fields means the feature may exist, but that
page has not been documented for the language yet:

```yaml
languages: [Python, TypeScript]
```

When the feature is unavailable in an SDK, classify that separately:

```yaml
unsupportedLanguages: Rust
```

The site must preserve the reader's global language selection on every page.
For a language outside `languages`, show that documentation is not available
yet. For a language in `unsupportedLanguages`, explain that the feature is not
available. Render either message only when that language is selected. Never
redirect to another language or tell readers that a page was "verified." While
the unavailable state is active, hide the page body and table of contents so
the reader never sees another SDK's examples under their selected language.

Keep the same heading and paragraph structure across language tabs. Put
language-specific prose, setup, and caveats in the matching `<Tab>` rather than
adding a language-named section or comparing SDKs in shared prose.

## TypeScript Snippet Scoping

When a `.ts` file has multiple snippets using the same variable names, wrap each snippet body in a function. Place markers **inside** the function so only the snippet body — not the function declaration — appears in docs:

```typescript
// Correct: function is for scoping only
async function exampleScope() {
  // --8<-- [start:example]
  const result = await agent.invoke('Hello')
  console.log(result)
  // --8<-- [end:example]
}
```

TypeScript uses `isolatedModules: true` — multiple top-level snippets that redeclare the same identifiers will fail type-checking without scoping. Body files generally should *not* need `// @ts-nocheck`; if they do, the snippets aren't scoped correctly.

Markers must be on their own line as line comments — anything else on the line and the build won't detect them. A typo in the snippet name fails to render the block; verify each `--8<--` reference resolves to a real `[start:…]`/`[end:…]` pair.

## Relative Links

Use relative file paths. They resolve automatically via the PageLink component:

```markdown
[Custom tools](../tools/custom-tools.md)
[Tools overview](../tools/index.md)
```

## API Reference Shorthand

Link to generated API docs without fragile relative paths:

```markdown
[@api/python/strands.agent.agent](AgentResult)
[@api/typescript/Agent#constructor](Agent constructor)
```

Rust guide pages link public items directly to their docs.rs item URL. Do not
omit an API link merely because the Rust reference is not embedded in the site.

## Line Length

90 characters maximum for files under `site/src/content/docs/`. Template literal contents in snippet files must also stay under 90 characters. Prettier does not enforce this automatically.

## Validating Changes

Run from `site/`:

- `npm run typecheck:snippets` type-checks TypeScript and Rust snippet files under `src/content/docs/`
- `npm run typecheck` type-checks other TypeScript
- `npm run format` applies Prettier formatting to doc snippet files
- `npm test` runs the site's unit tests
- `npm run dev` previews at http://localhost:4321/ to confirm snippets render

## Gotchas

- **Snippet dedent**: Leading whitespace is automatically stripped from included snippets. If the snippet reference in markdown is indented, content indents to that level.
- **Tab indentation**: Content inside `<Tab>` must not have leading blank lines or it may not render correctly.
- **API docs differ by language**: `site/src/content/docs/api/python/_generated` and `typescript/_generated` are symlinks to `site/.build/api-docs/`. Never edit these directly — they are auto-generated. Rust uses a static `api/rust/index.mdx` landing page and direct docs.rs item links. Embedding rustdoc is a separate site-design decision.
- **No `TabItem`**: The auto-imported component is `Tab`, not `TabItem` (even though Starlight's native component is called TabItem).
- **Code theme**: Uses GitHub Light/Dark themes. Follows Starlight's `[data-theme]` attribute.
- **Formatting**: No semicolons, single quotes, 2-space indent, trailing commas ES5 style (enforced by Prettier).
