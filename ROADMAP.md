# Where JackRabbit is heading

We want an R1 that's easy to get running, dependable every day and enjoyable to
make your own. GitHub issues and PRs carry the details. Release notes identify
what's available to install.

## Active delivery tracks

| Track | Scope | Status |
| --- | --- | --- |
| Windows installer | Verification [#13][13], preloader detection [#14][14], driver guidance [#15][15] and clearer confirmations | Planned |
| App maintenance | Calendar time zones [#11][11], a focused Display control fix and dependency updates | Planned |

Preparation can overlap. The Windows installer is first in release order, with
each track tested and released on its own evidence. Brightness controls already
exist. The Display work focuses on wheel adjustment and brightness when the app
cannot write the system setting.

## Workstreams

The order reflects the project's current emphasis. The active tracks above
show the work planned for delivery first.

1. **Security and provenance.** Verify the dependencies in each release and
   make it easier to see what shipped and which limitations remain.
2. **Daily-use reliability.** Make Calendar, Display, Wi-Fi and audio dependable.
   Review the captive portal contribution [#8][8], and improve on-device
   Bluetooth pairing and routing.
3. **Installation, firmware and recovery.** Improve Windows installation first.
   Verify boot, supported starting states and recovery on an identified
   candidate before releasing a new full image.
4. **Voice and conversations.** Improve everyday controls and audio transitions.
   Review direct Voice [#5][5] and push-to-talk [#9][9], and explore keeping
   conversation context when audio is interrupted.
5. **Project maintenance and contributor experience.** Keep the download route,
   getting-started instructions, contribution guide and release notes current.
6. **Experiments and extensibility.** Explore ideas with a clear use on the R1
   and a way to test them, including extensions and new controls.

See [Contributing](CONTRIBUTING.md) to report a problem, discuss an idea or
submit a change.

[5]: https://github.com/r1warren/JackRabbit-OS/pull/5
[8]: https://github.com/r1warren/JackRabbit-OS/pull/8
[9]: https://github.com/r1warren/JackRabbit-OS/pull/9
[11]: https://github.com/r1warren/JackRabbit-OS/pull/11
[13]: https://github.com/r1warren/JackRabbit-OS/issues/13
[14]: https://github.com/r1warren/JackRabbit-OS/issues/14
[15]: https://github.com/r1warren/JackRabbit-OS/issues/15
