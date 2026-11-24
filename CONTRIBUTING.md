# Contributing to JackRabbit

JackRabbit welcomes focused, non-commercial community contributions. Keep
`main` stable and make changes through pull requests.

Before writing code, read:

1. [README.md](README.md) for the product and architecture boundaries.
2. [BUILDING.md](BUILDING.md) for the supported build path.
3. [Extension Development Guide](EXTENSION-DEVELOPMENT.md) when adding an
   integration, Skill, Plugin, MCP server, Tool, Creation, or Card.
4. [llm.md](llm.md) when using a coding assistant.

## Branches

- `main` is the stable branch. Every change on it that affects the device has
  been tested on a physical Rabbit R1.
- `hutch` is where reviewed changes wait before release. Open pull requests
  against `hutch`. Any change that can alter what runs on the device, including
  changes to the build toolchain or builder image, is kept there until it has
  been installed and checked on real hardware. Changes with no effect on the
  device, such as documentation, need no hardware test. Changes are then
  released to `main` together. Releases that change what users install are
  tagged.

  Why "hutch": a hutch is where rabbits are kept until they are ready to be let
  out. The name was also chosen because it is not an everyday word. Much of
  this project is built by talking to coding agents, and common branch names
  such as `next`, `dev`, `staging` or `preview` are words that come up in
  conversation with other meanings. "Hutch" can only mean this branch.
- If you want the newest changes and accept that they are less proven, build
  from `hutch`. Otherwise use `main`.

**Merging.** PRs into `hutch` are squashed. Releases from `hutch` to `main` use
merge commits. Both branches require resolved review threads. No minimum count
of approving reviews is configured.

## Issues or pull requests

Check for related work first. Open an issue when diagnosis, design, release
scope or overlapping work needs discussion. A focused fix or docs change can
go straight to a PR. Explain the problem and expected result in the PR if
there's no issue. Use a draft PR to show work that still has open questions.

Use `Fixes #123` when a PR fully resolves an issue. GitHub can close the issue
when the PR merges into `hutch`, the default branch. If the fix still needs a
published build, say it is awaiting release. Use a plain issue reference for
partial fixes. Link duplicates to their tracking issue and explain declined work.

## Working together

- Create one branch per focused change. Do not push feature work directly to
  `hutch` or `main`.
- Keep each responsibility in its existing module. Avoid catch-all utility,
  helper, common, manager, or service files.
- If another pull request changes the same area, update your branch from the
  latest `hutch`, resolve the conflict there, and retest before review.
- Prefer small pull requests that preserve existing behavior. Separate local
  inference, STT/TTS, UI, and transport changes when they can be reviewed
  independently.

Share overlapping work early in an issue or draft PR. Two changes can
implement the same responsibility even when Git reports no line conflicts.

If a PR depends on another PR, name the dependency and intended final base.
After the dependency merges, update the remaining PR against `hutch`, review the
smaller diff, reconcile the APK version code if affected and rerun affected checks.

## Choose the correct boundary

Do not overhaul the application when an existing extension point owns the
work:

- Use an instruction `SKILLS.MD` when the owner only needs to change how Voice
  or Background Agent behaves.
- Use an Agent Skill inside a Plugin for portable, reusable instructions.
- Use MCP when a model needs tools from an external or independently running
  service.
- Use a Plugin when Skills and MCP configuration should install and move
  through one lifecycle, or when the package also owns one JackRabbit Card.
- Use a standalone Creation for a bounded static Card that does not need a
  Plugin lifecycle.
- Add a built-in Tool only when the capability must be implemented and shipped
  inside the trusted on-device Python runtime.
- Change the Android or provider/runtime code only for behavior that cannot be
  truthfully implemented through those extension boundaries. Native audio,
  WebRTC/WebSocket transport, camera, hardware, and provider selection are not
  Skills or Creations.

The [Extension Development Guide](EXTENSION-DEVELOPMENT.md) contains the exact
supported layouts and current limitations.

## Implementation rules

- Connect every user-facing surface to real behavior. Do not add mock data,
  placeholder screens, simulated states, or facade endpoints.
- Preserve one canonical owner for configuration, state, and domain data.
- Skills provide instructions; they do not grant tools or permissions.
- MCP is the model-facing tool boundary. Do not route high-rate microphone or
  speaker audio through MCP.
- Keep ChatGPT/Codex device authorization separate from external MCP
  credentials.
- Treat imported packages and remote MCP servers as untrusted until their
  validation, audience, and permission gates pass.
- Record copied donor code before importing it: source revision, exact source
  and destination paths, retained and omitted behavior, license decision, and
  tests. Donor repositories are read-only.

## Tests and evidence

Run the smallest relevant tests while developing, then the applicable project
checks from [BUILDING.md](BUILDING.md). Extension work should normally include
the matching runtime tests under `tests/runtime/`.

Do not describe a device behavior as working merely because it compiles or
passes a host test. Identify what was host-tested, what was exercised against a
real service, and what was physically accepted on an R1.

For changed Markdown, run the repository's 100-column configuration with
`markdownlint-cli` from the repository root:

```sh
markdownlint --config .markdownlint.jsonc <changed-markdown-files>
```

Code blocks and tables are exempt from the line-length rule.

## Pull requests

Use the [PR template](.github/PULL_REQUEST_TEMPLATE.md) to record scope,
ownership, validation and applicable physical R1 evidence. Request AI review
when it helps. AI reviewer availability and verdicts are optional. Check
findings and record the fix or reason for declining before resolving a thread.

Reviewers may ask for a large change to be split when it crosses independent
owners. A Plugin that includes a Skill, an MCP connection, and a Card may stay
together when those pieces form one coherent integration and share one
lifecycle.

## From merge to release

| State | What to expect |
| --- | --- |
| Proposed | An issue or draft PR is open for discussion. |
| Ready for review | The PR describes the change and provides the checks available so far. |
| Integrated | The PR is merged into `hutch` and can be included in an identified test build. |
| Promoted | The maintainer has merged the accepted batch into `main` through a release PR. |
| Released | Release notes identify the downloadable build and how to install it. |

We integrate focused changes into `hutch`, test identified builds and settle
the final candidate when the batch is ready. The maintainer, currently
[@jd-d](https://github.com/jd-d), opens a release PR when the batch, acceptance
evidence and release material are ready. Releases have no fixed schedule.

1. Record the exact candidate revision, included changes and known limitations.
   For installable builds, record versions, hashes, signing identity where
   applicable, and relevant host, live-service and physical R1 evidence.
2. If `hutch` advances, test the changed candidate or use a release branch
   pinned to the accepted revision. Confirm the release PR matches the accepted
   content before merging. A release-candidate tag and GitHub pre-release can
   identify the exact revision and build shared for testing.
3. For installable releases, tag the promoted `main` revision and publish the
   accepted artifacts, checksums, supported starting states and install or
   update steps. Check the published downloads against the accepted hashes.
   Include known limitations and any data erasure.
4. Update the download guidance and list the included fixes in the release
   notes. Add a comment on each fixed issue with a link to the release that
   actually includes its fix.

Docs-only batches can be promoted without a binary release.

## License

Contributions are accepted under the repository's
[PolyForm Noncommercial License 1.0.0](LICENSE). JackRabbit is a
source-available noncommercial project and its license is not OSI-approved.
Agent Skills, Agent Plugins, and MCP are open standards; using those formats
does not change JackRabbit's project license.

Commercial use, monetized distribution, and commercial sublicensing are not
accepted. Verify that new dependencies and bundled assets can legally be
distributed under this noncommercial project model.

Questions and early design discussion are welcome in the
[JackRabbit Discord community](https://discord.gg/HeKGmh5mC).
