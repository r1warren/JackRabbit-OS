# JackRabbit

**Give your Rabbit R1 a new burrow.** JackRabbit is a noncommercial community
voice system for the R1. Talk with it, check your Calendar and Tasks, and add
Cards of your own. The app, local data and management console run on the R1.
Voice and connected services still need their respective providers.

## Get started

- **Stock Rabbit R1:** Start with the
  [installation guide and download status](installer/INSTALL.md).
  The guide links to the complete `jackrabbit-current-v0.2.zip` bundle and gives
  its checksum. Read the whole guide before starting. **Installation erases R1 data.**
- **Already running JackRabbit:** Follow [Using JackRabbit](USER-GUIDE.md) for
  setup and daily use. For the [v0.4.18 device candidate][apk-candidate], check
  the release notes and confirm it fits your device and setup before trying it.
  This is a pre-release APK for an existing compatible setup.
- **Building or contributing:** Start with [Building JackRabbit](BUILDING.md)
  and [Contributing](CONTRIBUTING.md). Use `main` for stable source and `hutch`
  for changes being prepared for release.

On macOS, the [v0.2.1 installer patch][mac-patch] updates the matching host
installer inside the complete v0.2 bundle. It contains no Android images.
On Windows, read the open reports on [verification][windows-verify],
[preloader detection][windows-preloader] and [driver setup][windows-driver]
before starting. Windows installer maintenance is on the [roadmap](ROADMAP.md).

| Voice | Cards | Calendar |
| --- | --- | --- |
| ![JackRabbit Voice on the R1](images/r1-voice-page.png) | ![JackRabbit Cards on the R1](images/r1-cards-page.png) | ![Calendar Card showing an upcoming event](images/r1-calendar-card.png) |

## Start your first Voice conversation

1. On the R1, open **Settings → Wi-Fi** and connect it to the same network as
   your browser.
2. Open **Settings → Management** and visit the displayed HTTPS address in the
   browser. Follow the
   [management setup instructions](USER-GUIDE.md#connect-wi-fi-and-open-management)
   to trust the device certificate, then enter the pairing code.
3. In the browser, open **Overview → Your Profile**, enter your name and select
   **Save name**. Check for **Name saved.** so Voice can greet you by name.
4. In **AI & Voice**, connect ChatGPT/Codex or an OpenAI Platform API key.
   JackRabbit uses one access method at a time. To switch from ChatGPT/Codex
   to Platform access, disconnect ChatGPT/Codex first. Choose your models,
   then return to Voice and press the microphone control. See
   [AI setup and model selection](USER-GUIDE.md#connect-openai-access).

## What you can do

| Area | What it does |
| --- | --- |
| Voice | Talk over native OpenAI Realtime audio and use supported tools during a conversation. |
| Cards and local data | See Calendar and Tasks on the R1. Connect Mail or Calendar and use their supported Voice tools. |
| Background Agent | Delegate a goal, track progress and collect results in the on-device workspace. |
| Extensions | Add Skills, Plugins, MCP connections or static Creations. |
| Management | Use a paired browser on the same network to manage AI, connections and extensions. |
| R1 controls | Configure Wi-Fi, Bluetooth, volume, brightness and keep-screen-awake. |

The [user guide](USER-GUIDE.md) covers device controls, Cards, management,
connections and troubleshooting.

### Voice, camera and Cards

Voice and Cards are the first two pages of the R1's home screen. The native app
is designed for its 480×640 display, touch screen, scroll wheel and side button.
The camera can open outward for a live preview, then return to its privacy
position. Use it to scan a Creation QR code or hand an image directly to an
active Voice conversation.

| Settings | Mail and Calendar connections |
| --- | --- |
| ![JackRabbit Settings page](images/r1-settings-page.png) | ![Management page showing Mail and Calendar connections](images/web_management/connections.png) |

Calendar and Tasks are built-in Cards. Enabled Creations appear alongside them.
For account limits, sync timing and what each tool can do, see
[Calendar](USER-GUIDE.md#connect-calendar), [Mail](USER-GUIDE.md#connect-mail)
and [Tasks](USER-GUIDE.md#use-tasks).
An installed Creation may fail to open on an image with no active Android
WebView provider. The [WebView recovery steps][webview-recovery] explain how to
recover without reflashing or reinstalling the Creation.

### Management console

The R1 serves its management console over HTTPS to a browser on the same local
network. Pair with a six-digit, one-time code that expires after five minutes.
A paired browser session lasts 30 minutes.

| Overview | AI & Voice |
| --- | --- |
| ![JackRabbit management Overview](images/web_management/overview.png) | ![JackRabbit AI and Voice settings](images/web_management/ai-voice.png) |

From Management you can check device and runtime status, edit your Voice profile,
download the R1's TLS certificate, and restart the runtime. You can also manage
AI access and models, configure Mail and Calendar accounts, import and manage
extensions, and inspect Background Agent runs.

The Library distinguishes owner instructions (`SKILLS.MD`) from portable
Agent Skills packaged as Plugins. It also manages MCP connections, tool access
and Creations. See the [Library guide](USER-GUIDE.md#manage-the-library) for
day-to-day use and the [Extension Development Guide](EXTENSION-DEVELOPMENT.md)
for formats and boundaries.

![JackRabbit Library with Skills, Plugins, MCP, Tools and Creations](images/web_management/library.png)

### Background Agent

Voice can delegate a goal to the on-device Background Agent worker. You can
watch progress, cancel a run and collect its artifacts. The queue can hold up
to eight runs and allows one active run per origin. The workspace confines
paths, rejects symbolic links and writes files atomically. Artifacts selected
for delivery are moved to durable storage.

The R1 has completed multi-turn research runs and delivered Markdown artifacts
through the native runner view. The
[Background Agent guide](USER-GUIDE.md#use-background-agent) covers setup,
default limits and which details appear in Run Logs and Reasoning Logs.

![JackRabbit Background Agent settings and run logs](images/web_management/background-agent.png)

## Security and privacy

- Platform, subscription and connection credentials pass through an Android
  bridge and are sealed with Android Keystore-backed AES-256-GCM. The Python
  database stores no plaintext credentials.
- The local API token and management TLS key use device-protected storage.
  The certificate identity is bound to the active Wi-Fi or Ethernet address
  when available. Management writes require a paired session, a matching HTTPS
  origin and a CSRF token.
- Mail sending requires a single-use confirmation tied to the exact draft.
  Voice has no Mail deletion tool. The management API exposes account
  configuration and status without returning message content.
- Imported archives are checked before activation. Outbound MCP tools are
  limited by both the configured permissions and the agent's audience.
- Transcripts, summaries, memories, domain records, run records and workspace
  files are stored locally. Network-backed AI, Mail and Calendar sync, web
  search and outbound MCP tools send relevant data to their respective services.

The project makes no claim of a formal security audit. See the
[privacy reminders](USER-GUIDE.md#privacy-reminders) for what to check before
connecting an account or importing an extension.

## How it fits together

Android owns the device UI, input, camera, hardware controls and live WebRTC
audio. The embedded Python runtime owns providers, agents, tools and storage.
The web module serves the management interface. The app targets Android API 36,
requires API 31 or newer, and is built for ARM64.

See the [architecture diagram and repository map](llm.md#architecture) and the
[build guide](BUILDING.md) for module boundaries and toolchain requirements.
The [SQLite migration list][migrations] records the schema version for this checkout.

## Contributing

Open an issue to discuss a problem or idea, or send a focused PR directly to
`hutch`. Changes that affect the device need physical R1 evidence before
promotion to `main`. For installable changes, the release notes tell you which
download contains the fix. See the [contribution guide](CONTRIBUTING.md) for the
full process and the [roadmap](ROADMAP.md) for current priorities.

See the [architecture notes](llm.md) and [extension guide](EXTENSION-DEVELOPMENT.md)
to find the right place for a change.

Join the [JackRabbit Discord](https://discord.gg/HeKGmh5mC) for discussion and
installation help.

## Project history

JackRabbit OS was created by christianmcgivern at ReSono-Labs. In October 2026
the repository moved to the R1 Warren organisation and maintenance passed to
jd-d. The tag `org-move-resonolabs-to-r1warren` marks the project as it stood
at that point.

## License

JackRabbit is source-available under the
[PolyForm Noncommercial License 1.0.0](LICENSE). Community members may use,
study, modify, fork, and share JackRabbit and modified versions for
noncommercial purposes, subject to the license and its required notices.

Commercial use is not permitted. This includes selling JackRabbit, including it
in a paid product or service, monetizing its distribution, or using it for an
anticipated commercial application. JackRabbit is and will remain a
noncommercial community project: ReSono Labs will not sell JackRabbit, offer a
commercial license for it, or authorize commercial use or monetized
distribution.

This is a source-available noncommercial license, not an OSI-approved license.
Third-party components remain subject to their own licenses. JackRabbit is an
independent community project and is not affiliated with, endorsed by, or
sponsored by rabbit inc.

[apk-candidate]: https://github.com/r1warren/JackRabbit-OS/releases/tag/v0.4.18-device-candidate
[mac-patch]: https://github.com/r1warren/JackRabbit-OS/releases/tag/installer-macos-v0.2.1
[windows-verify]: https://github.com/r1warren/JackRabbit-OS/issues/13
[windows-preloader]: https://github.com/r1warren/JackRabbit-OS/issues/14
[windows-driver]: https://github.com/r1warren/JackRabbit-OS/issues/15
[webview-recovery]: USER-GUIDE.md#an-installed-creation-does-not-open
[migrations]: runtime/resono_runtime/storage/migrations/__init__.py
