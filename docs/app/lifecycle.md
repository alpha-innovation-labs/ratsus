# App lifecycle

`src/app/lifecycle/` owns startup of the Ratkit application runtime.

## Responsibilities

- Accept an injected `ChatHarness`.
- Build `AppState` through `AppState::new_with_harness`.
- Start Ratkit with `ratkit::run` and default runner configuration.

## Startup selection

`src/main.rs` chooses the harness before lifecycle startup. Nexus is the default. `RATSUS_BACKEND=stub` selects the deterministic `StubHarness`.

## Key files

- `src/app/lifecycle/run_app.rs`
- `src/main.rs`
