import { describe, expect, it } from 'vitest'
import { RUST_QUICKSTART_PROMPT } from '../src/data/quickstart-prompts'

describe('Rust quickstart prompt', () => {
  it('asks for a provider and keeps that model when tools are added', () => {
    expect(RUST_QUICKSTART_PROMPT).toContain('Ask me which model provider I want to use:')
    expect(RUST_QUICKSTART_PROMPT).toContain('cargo add strands --no-default-features --features anthropic')
    expect(RUST_QUICKSTART_PROMPT).toContain('cargo add strands --no-default-features --features openai')
    expect(RUST_QUICKSTART_PROMPT).toContain('cargo add strands --no-default-features --features mantle')
    expect(RUST_QUICKSTART_PROMPT).toContain('Keep .model(model) when the selected provider')
    expect(RUST_QUICKSTART_PROMPT).not.toContain('Use Amazon Bedrock, the default model provider.')
  })
})
