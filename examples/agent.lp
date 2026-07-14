@ agent.lp — Minimal AI agent example.

@ Import the AI framework for LLM integration.
use ai.

@ Create an AI assistant connected to Groq.
assistant = Assistant(),
    provider = Groq.
    api_key = env.GROQ_API_KEY.
    model = llama-3.3-70b.
    system_prompt = "You are a helpful assistant for the Lang.P programming language.".
..

@ Respond when the user sends a message.
on user.message,
    reply = assistant.chat(user.message).
    print reply.
..

@ Handle errors from the AI provider gracefully.
on assistant.error,
    print "AI Error: " with assistant.error.message.
..

@ Send an initial greeting to demonstrate the assistant.
reply = assistant.chat("What is Lang.P?").
print reply.
