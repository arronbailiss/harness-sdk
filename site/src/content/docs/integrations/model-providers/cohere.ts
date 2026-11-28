import { Agent } from '@strands-agents/sdk'
import { OpenAIModel } from '@strands-agents/sdk/models/openai'

async function usage() {
  // --8<-- [start:usage]
  const model = new OpenAIModel({
    api: 'chat',
    apiKey: process.env.COHERE_API_KEY,
    clientConfig: {
      baseURL: 'https://api.cohere.ai/compatibility/v1',
    },
    modelId: 'command-a-03-2025',
    params: {
      stream_options: null,
    },
  })

  const agent = new Agent({ model })
  const response = await agent.invoke('Explain tool calling in one sentence.')
  console.log(response)
  // --8<-- [end:usage]
}
