import { readFileSync } from 'node:fs'

import yaml from 'js-yaml'
import { describe, expect, it } from 'vitest'

interface WorkflowStep {
  name?: string
  with?: {
    script?: string
  }
}

interface Workflow {
  jobs?: Record<
    string,
    {
      if?: string
      steps?: WorkflowStep[]
    }
  >
}

const workflowsDirectory = new URL('../../.github/workflows/', import.meta.url)

function loadWorkflow(fileName: string): Workflow {
  return yaml.load(readFileSync(new URL(fileName, workflowsDirectory), 'utf8')) as Workflow
}

describe('Rust workflow integration', () => {
  it('checks crates.io package provenance for Rust catalog entries', () => {
    const workflow = loadWorkflow('catalog-editorial-review.yml')
    const script = workflow.jobs?.['flag-editorial-fields']?.steps?.find(
      (step) => step.name === 'Flag featured/badges and unverified package provenance'
    )?.with?.script

    expect(script).toContain("['python', 'typescript', 'rust']")
    expect(script).toContain("if (lang === 'rust')")
    expect(script).toContain('https://crates.io/api/v1/crates/')
    expect(script).toContain("'User-Agent'")
    expect(script).toContain('data.crate?.repository')
  })

  it('runs changelog sync immediately for Rust releases', () => {
    const workflow = loadWorkflow('changelog-sync.yml')
    const condition = workflow.jobs?.sync?.if

    expect(condition).toContain("startsWith(github.event.release.tag_name, 'rust/v')")
    expect(condition).toContain("startsWith(github.event.release.tag_name, 'harness-rust/v')")
  })
})
