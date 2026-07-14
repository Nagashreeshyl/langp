# 22 — AI Framework

**Status: Specification — not in v0.1 interpreter**

---

> The AI framework defines assistants, tools, streaming, and agents. See [AI framework (spec)](../spec/18-ai-framework.md). `examples/agent.lp` is **aspirational**.

---

## Assistant creation (specification)

```lp
use ai.

assistant = Assistant().
response = assistant.chat("Explain Lang.P in one sentence").
print response.
```

---

## Tool calling (specification)

Assistants can register tools that call Lang.P functions — design in spec §18.

---

## Streaming (specification)

```lp
@ Stream tokens to output as they arrive.
for chunk in assistant.stream("Write a poem"),
    print inline chunk.
..
```

---

## Memory and agents (specification)

Long-term memory, RAG, and multi-step agents are specified for future releases.

---

## v0.1

Integrate external AI tools via shell scripts or host languages until `use ai.` ships.

---

## Next steps

- [23 — Best Practices](23-best-practices.md)
- [AI framework (spec)](../spec/18-ai-framework.md)
