# Shared utilities

`src/shared/` is reserved for dependency-light primitives that are genuinely useful across multiple domains. It may stay empty when no real shared primitive exists.

## Responsibilities

- Hold small utilities that do not belong to a single app, UI, core, or extension module.
- Avoid feature ownership, backend behavior, rendering, and stateful services.
- Stay dependency-light so shared code does not become a hidden coupling point.

## Current state

The module currently contains only `src/shared/mod.rs`. Add helpers here only when at least two domains need the same primitive and no clearer domain owner exists.

## Rules

- Do not place product features in `shared/`.
- Do not place app orchestration in `shared/`.
- Do not place backend adapters in `shared/`.
- Prefer a domain-local helper until reuse is real.
- Do not add code here just to make `shared/` non-empty.

## Key files

- `src/shared/mod.rs`
