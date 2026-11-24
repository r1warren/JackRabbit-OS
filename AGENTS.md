# JackRabbit project instructions

Read [Contributing](CONTRIBUTING.md) for workflow, [Building](BUILDING.md) for
supported checks and [llm.md](llm.md) for architecture. Read any applicable
module instructions before making changes.

## Scope

- Preserve the noncommercial license and existing module boundaries.
- Android owns native UI, input, hardware and live audio. The runtime owns
  provider configuration, agents, tools, storage and APIs. Web owns management
  assets. Use the existing extension point when it fits.
- Keep one owner for configuration, state and credential resolution. Keep
  high-rate microphone and speaker audio out of Python and MCP.
- Connect user-facing changes to real behavior and keep patches focused.

## Before handing off

- Use `hutch` as the base for contribution PRs. Follow the release process in
  [Contributing](CONTRIBUTING.md), and keep contributor credit intact.
- Report host checks, live-service checks and physical R1 acceptance separately.
  Device-affecting changes include changes to the toolchain and builder image.
- Confirm device availability before operating an R1. Get separate approval
  for firmware writes, data erasure and recovery operations.
- For installer changes, run formatting, locked tests and a release build
  against `installer/cli/Cargo.toml`. Exercise the built Windows binary when
  Windows behavior changes. Preserve device identity, hash and confirmation gates.
- Keep signing keys, credentials, device logs and generated binaries out of Git.
- Update affected user and developer docs. Check changed Markdown with
  `markdownlint --config .markdownlint.jsonc <changed-markdown-files>`.
