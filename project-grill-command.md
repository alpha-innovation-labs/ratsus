# Repository Understanding Grill

## Purpose

Use this command to verify that the LLM understands a repository's purpose, architecture, ownership boundaries, and operating rules before it edits code.

## Command prompt

You are onboarding to this repository. Do **not** inspect source code, tests, configuration files, scripts, generated files, dependency manifests, lockfiles, or hidden files unless your system/developer instructions explicitly allow them.

Read only the orientation material explicitly allowed by your system/developer instructions. If the allowed material is ambiguous, stop and ask which files may be read instead of exploring the repository.

First, explain what you think this project is about from the allowed material only. Keep the explanation concise and name the evidence category you used, such as system prompt, repository guide, docs, or user-provided context. Do not infer unsupported details.

Then grill me with clarifying questions to test and improve your understanding. You must use the `ask_user_question` tool, not plain text, for the questions when the tool is available.

## Question requirements

- Ask 3 or 4 questions in a single `ask_user_question` call.
- Each question must check an architectural boundary, ownership rule, workflow rule, product concept, or quality expectation.
- Use a direct grilling style: specific, pointed, and hard to answer vaguely.
- Each question must have 2 to 4 mutually exclusive options.
- Put the recommended answer first and append `(Recommended)` to that option label.
- Every option must include a short description explaining why it is right, wrong, incomplete, or risky.
- Do not include custom `Other`, `Type something`, or `Chat about this` options; the tool provides those automatically when supported.
- If the tool is unavailable, list the same questions in plain text and clearly mark the recommended answer for each.

## Suggested topic categories

Cover a useful mix of these general topics without naming project-specific modules or technologies unless they appear in the allowed material:

1. What the product does and who it serves.
2. Which layer owns orchestration versus domain behavior versus reusable primitives.
3. Which components are product-specific and which are shared infrastructure.
4. Where external service, backend, adapter, or integration logic should live.
5. What should happen when behavior belongs in a different repository, package, service, or dependency.
6. What starts a bug fix, feature, refactor, or documentation change.
7. What belongs in shared/common utilities and what should stay domain-local.
8. When documentation, tests, contracts, or configuration must be updated.
9. Which consistency contracts apply across repeated UI, API, workflow, or data-handling patterns.
10. What data safety, migration, deletion, privacy, or user-owned state rules matter.

## Final behavior

After the user answers, grade each answer briefly. Correct misunderstandings, identify missing rules, and ask one follow-up only if a critical boundary remains unclear.
