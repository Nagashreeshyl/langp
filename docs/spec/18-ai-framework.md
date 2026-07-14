# Chapter 18 — AI Framework

## 18.1 Overview

The Lang.P AI framework provides first-class support for building AI-powered applications, agents, and LLM integrations. AI is a core part of the Lang.P ecosystem, not an afterthought.

```lp
use ai.
```

## 18.2 Design Goals

1. **Provider-agnostic** — switch between OpenAI, Anthropic, Groq, Google, Ollama without code changes.
2. **Beginner-friendly** — a working chatbot in under 10 lines.
3. **Production-ready** — streaming, tool calling, memory, RAG, agents.
4. **MCP-native** — Model Context Protocol integration built in.

## 18.3 Creating an Assistant

### 18.3.1 Basic Assistant

```lp
use ai.

assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
.

on user.message,
    reply = assistant.chat(user.message).
    print reply.
.
```

### 18.3.2 Full Configuration

```lp
assistant = Assistant(),
    provider = OpenAI.
    api_key = env.OPENAI_API_KEY.
    model = gpt-4o.
    temperature = 0.7.
    max_tokens = 4096.
    system_prompt = "You are a helpful coding assistant.".
    streaming = enabled.
    memory = enabled.
    tools = [search_web, run_code, read_file].
.
```

## 18.4 Supported Providers

| Provider | Configuration | Models |
|----------|--------------|--------|
| `OpenAI` | `api_key = env.OPENAI_API_KEY` | gpt-4o, gpt-4o-mini, o1 |
| `Anthropic` | `api_key = env.ANTHROPIC_API_KEY` | claude-sonnet-4, claude-opus-4 |
| `Google` | `api_key = env.GOOGLE_API_KEY` | gemini-2.0-flash, gemini-2.0-pro |
| `Groq` | `api_key = env.GROQ_API_KEY` | llama-3.3-70b, mixtral-8x7b |
| `OpenRouter` | `api_key = env.OPENROUTER_API_KEY` | All supported models |
| `Ollama` | `host = "http://localhost:11434"` | Local models |

Switching providers requires changing only the provider and model:

```lp
@ Development — local Ollama
assistant = Assistant(),
    provider = Ollama.
    model = llama3.
.

@ Production — Groq
assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
.
```

## 18.5 Chat

### 18.5.1 Simple Chat

```lp
reply = assistant.chat("What is Lang.P?").
print reply.
```

### 18.5.2 Conversation

```lp
conversation = assistant.conversation().

reply1 = conversation.send("Hello").
reply2 = conversation.send("What can you help me with?").
history = conversation.history().
conversation.clear().
```

### 18.5.3 Streaming

```lp
on user.message,
    stream = assistant.stream(user.message).
    async for chunk in stream,
        print inline chunk.
    .
    print "".
.
```

## 18.6 Tool Calling

Define tools that the AI can invoke:

```lp
@ Tool definition
function search_web(query: String) -> String,
    results = wait for get ("https://search.example.com?q=" with query).
    return results.body.
.

function run_code(code: String) -> String,
    result = execute_sandbox(code).
    return result.output.
.

assistant = Assistant(),
    provider = OpenAI.
    api_key = env.OPENAI_API_KEY.
    model = gpt-4o.
    tools = [search_web, run_code].
.

@ The assistant automatically calls tools when needed
reply = assistant.chat("Search for the latest Lang.P news and summarize").
```

Tool definition with metadata:

```lp
tool get_weather,
    description = "Get current weather for a city".
    parameter city: String, description = "City name".
    return fetch_weather(city).
.
```

## 18.7 Agents

Agents are autonomous AI entities that can plan, execute, and iterate:

```lp
agent = Agent(),
    assistant = assistant.
    goal = "Research and write a summary about quantum computing".
    max_steps = 10.
    tools = [search_web, read_page, write_file].
.

result = wait for agent.run().
print result.summary.
```

Agent workflow:

```
Goal → Plan → Execute Tool → Evaluate → Replan → ... → Result
```

## 18.8 Embeddings

```lp
use ai.

embeddings = Embeddings(),
    provider = OpenAI.
    model = text-embedding-3-small.
.

vector = embeddings.embed("Hello, world").
vectors = embeddings.embed_batch(["text1", "text2"]).
similarity = embeddings.cosine_similarity(vector1, vector2).
```

## 18.9 RAG (Retrieval-Augmented Generation)

```lp
use ai.

@ Build a knowledge base
kb = KnowledgeBase(),
    embeddings = embeddings.
.

kb.add_document("manual.pdf").
kb.add_text("Lang.P is a readable programming language.").
kb.add_directory("docs/").

@ Query with context
assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
    knowledge = kb.
.

reply = assistant.chat("How do I create a browser in Lang.P?").
@ Response is grounded in the knowledge base
```

## 18.10 Memory

Persistent conversation memory:

```lp
assistant = Assistant(),
    provider = OpenAI.
    model = gpt-4o.
    memory = Memory(),
        type = persistent.
        storage = "memory.db".
        max_entries = 1000.
.
.

@ The assistant remembers past conversations
reply = assistant.chat("What did we discuss yesterday?").
```

Memory types:

| Type | Description |
|------|-------------|
| `session` | Current session only |
| `persistent` | Stored across sessions |
| `semantic` | Vector-based semantic recall |

## 18.11 MCP Integration

Model Context Protocol support:

```lp
use ai.mcp.

@ Connect to MCP servers
mcp = MCPClient(),
    servers = [
        "filesystem": "npx @modelcontextprotocol/server-filesystem /path".
        "github": "npx @modelcontextprotocol/server-github".
    ].
.

assistant = Assistant(),
    provider = Anthropic.
    model = claude-sonnet-4.
    mcp = mcp.
.

@ Assistant can use MCP tools (file access, GitHub, etc.)
reply = assistant.chat("List the files in my project and create a README").
```

## 18.12 AI Events

```lp
on user.message,
    @ Fires when user sends a message
.

on assistant.response,
    @ Fires when assistant completes a response
    print assistant.response.text.
.

on assistant.stream_chunk,
    @ Fires for each streaming chunk
    print inline assistant.stream_chunk.text.
.

on assistant.tool_call,
    @ Fires when assistant invokes a tool
    print "Calling: " with assistant.tool_call.name.
.

on assistant.error,
    @ Fires on API or processing errors
    print "Error: " with assistant.error.message.
.
```

## 18.13 Structured Output

```lp
type Analysis = ,
    summary: String.
    sentiment: String.
    keywords: List<String>.
.

result = assistant.structured("Analyze this text: " with text, type = Analysis).
print result.summary.
print result.sentiment.
```

## 18.14 Multi-Modal

```lp
@ Image understanding
reply = assistant.chat("Describe this image", image = "photo.jpg").

@ Image generation (provider-dependent)
image = assistant.generate_image("A sunset over mountains").
write_bytes image to "sunset.png".
```

## 18.15 AI in Lang Studio

Lang Studio integrates AI throughout the IDE:

- **AI Assistant panel** — chat with AI about your code.
- **Inline suggestions** — AI-powered code completion.
- **Explain code** — select code, ask AI to explain.
- **Generate from comment** — write a `@` comment describing what you want, AI generates code.
- **Fix errors** — AI suggests fixes for compiler errors.
- **Generate tests** — AI writes test cases for functions.

## 18.16 Cost Tracking

```lp
usage = assistant.usage().
print "Tokens used: " with usage.total_tokens.
print "Estimated cost: $" with usage.estimated_cost.
```

## 18.17 Error Handling

```lp
try,
    reply = assistant.chat("Hello").
catch error: ai.RateLimitError,
    print "Rate limited. Retry after " with error.retry_after with " seconds".
    wait for sleep(seconds = error.retry_after).
    reply = assistant.chat("Hello").
catch error: ai.AuthenticationError,
    print "Invalid API key".
catch error: ai.ModelNotFoundError,
    print "Model not available: " with error.model.
..
```

## 18.18 Best Practices

1. Store API keys in environment variables, never in source code.
2. Use streaming for long responses to improve perceived latency.
3. Set `max_tokens` to prevent runaway costs.
4. Use RAG for domain-specific knowledge instead of long system prompts.
5. Define focused tools — one tool per capability.
6. Use `structured` output when you need parseable responses.
