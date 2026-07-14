# lang

Package manager CLI for Lang.P. Handles dependencies, building, testing, and publishing.

## Usage

```bash
lang init my-project          # Create new project
lang init --template browser  # Create from template
lang build                    # Build the project
lang run                      # Build and run
lang test                     # Run tests
lang add requests             # Add dependency
lang publish                  # Publish to registry
```

## Responsibilities

- Project initialization from templates
- Dependency resolution and lock file management
- Build orchestration (invoke `langc`)
- Test runner integration
- Package publishing to registry
- Registry search and package info

## Status

Phase 10 (not yet implemented). See [Chapter 20 — Package System](../docs/spec/20-package-system.md).

## Configuration

Projects are configured via `lang.toml` at the project root.
