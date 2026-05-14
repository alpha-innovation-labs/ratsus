# App test support

`src/app/test_support/` owns fixtures for focused tests that need app state without launching real external terminal processes.

## Responsibilities

- Build minimal `AppState` fixtures.
- Provide deterministic state setup for rendering and input tests.
- Avoid real Nexus process, PTY, or shell dependencies in focused unit tests.

## Boundary

Production startup belongs in `src/app/state/new_app_state.rs` and `src/app/lifecycle/run_app.rs`. Test helpers should stay explicit and should not hide production behavior behind mocks unless the test specifically needs an isolated fixture.

## Key files

- `src/app/test_support/app_fixture.rs`
