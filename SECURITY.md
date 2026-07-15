# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.2.x   | Yes       |
| 0.1.x   | Best effort |
| < 0.1   | No        |

## Reporting a vulnerability

**Please do not open public GitHub issues for security vulnerabilities.**

Report security issues privately by:

1. Opening a **GitHub Security Advisory** on [Nagashreeshyl/langp](https://github.com/Nagashreeshyl/langp/security/advisories/new), or
2. Contacting the repository maintainers through GitHub direct message if you have an existing channel

Include:

- Description of the vulnerability
- Steps to reproduce
- Affected version(s)
- Impact assessment (if known)

We aim to acknowledge reports within **72 hours** and provide a fix or mitigation plan within **14 days** for confirmed issues.

## Scope

In scope:

- `lang`, `langc`, `lang-lsp` binaries and install scripts
- Interpreter filesystem access and package manager cache paths
- VS Code extension command execution (`lang check`, `lang run`)

Out of scope:

- Third-party editor extensions not published by this repository
- User `.lp` programs (Lang.P is a general-purpose language; sandboxing is not provided in v0.2)

## Safe usage

- Do not run untrusted `.lp` programs with elevated privileges
- Review install scripts before piping to `sh` or `iex`
- Package manager cache lives in `~/.cache/langp/` — treat installed packages as code you trust

## Recognition

We appreciate responsible disclosure and will credit reporters in release notes when fixes ship (unless you prefer anonymity).
