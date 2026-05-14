# UI notifications

`src/ui/notifications/` owns user-facing toast notification helpers.

## Responsibilities

- Build notification messages for completed user actions.
- Keep notification presentation separate from the work that caused the notification.
- Use Ratkit toast rendering through app-level rendering.

## Child modules

- `toast/` contains toast helper functions.

## Boundary

Notification helpers should not perform backend operations, mutate Nexus data, spawn processes, or own long-running work. They present results produced by other domains.

## Key files

- `src/ui/notifications/toast/`
