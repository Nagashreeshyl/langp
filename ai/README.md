# AI Framework

Lang.P's built-in framework for LLM integration, agents, RAG, and MCP support.

## Capabilities

- Provider-agnostic assistant (OpenAI, Anthropic, Google, Groq, Ollama, OpenRouter)
- Streaming responses
- Tool calling and function execution
- Autonomous agents with planning
- Embeddings and RAG (Retrieval-Augmented Generation)
- MCP (Model Context Protocol) integration
- Persistent conversation memory

## Quick Example

```lp
use ai.

assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
.

reply = assistant.chat("What is Lang.P?").
print reply.
```

## Status

Phase 12 (not yet implemented). See [Chapter 18 — AI Framework](../docs/spec/18-ai-framework.md).

## Example

See [`examples/agent.lp`](../examples/agent.lp).
