# Security model / Confidentialité

Keycast Bridge intentionally reads global keyboard and optionally mouse events. It does not detect password fields or stop automatically on session lock/switch. Stop before entering secrets or locking. Shortcuts-only filtering reduces exposure; it is not a boundary against sensitive shortcuts.

## Linux

The GUI/server refuses root. `pkexec` launches an administrator-installed helper at a fixed path; the policy requests authorization on each start. The helper forks before starting threads:

- A root broker monitors udev, enumerates input event nodes, checks device numbers and evdev capabilities, and opens only the selected keyboards and optionally mice. It stays privileged for hotplug but never reads key events or connects to the network.
- An event reader drops supplementary groups, GID and UID, enables `NO_NEW_PRIVS`, and receives descriptors via `SCM_RIGHTS` on an inherited, private `AF_UNIX` socketpair. No public Unix socket is created. The reader uses epoll and keeps per-device modifier state; removal closes the descriptor and discards that state.

Only normalized labels, held modifier states, mouse buttons (no Linux pointer coordinates), readiness and device notifications reach the GUI via stdout. Stdin carries a heartbeat. Stop or application exit invalidates the generation and closes stdin; EOF or three seconds without heartbeat stops the reader, then the broker exits. Event node numbers are not stored as persistent identity. Selected devices use serial/name, falling back to physical USB path/name; this is convenience matching, not cryptographic device authentication. Input remains visible to the reader after privilege drop; filtering is application policy, not hardware isolation.

## Windows

The normal-user process uses `WH_KEYBOARD_LL` and optional `WH_MOUSE_LL` hooks on a dedicated thread. Callbacks queue bounded events and always pass input on to other applications. Overflow ends capture. No driver, service or elevation is installed. Mouse click coordinates are sent only to the local overlay to position the optional selected-monitor halo. No movement stream is retained. Secure desktops and elevated applications are outside the supported target.

## Shared boundaries and residual exposure

Only IPv4 localhost is bound. A random per-launch capability token protects the read-only overlay and WebSocket; Host/Origin checks reject foreign origins. No history, telemetry, external assets or keystroke logs. Late events from stopped generations are discarded; stop/disconnect clears the overlay. Slow WebSocket consumers are disconnected instead of accumulating unlimited events.

Other same-user software, administrators, a compromised GUI or malicious OBS browser extensions are outside the threat model. The token may appear in OBS configuration or browser history: do not share it. Restarting rotates it. The emergency shortcut consumes no keys; other applications receive it too. On Linux, press it on a captured keyboard, with all modifiers on that keyboard. No independent security audit is claimed.

Appearance preferences (position, colors and canvas dimensions only) are written to the user configuration directory. They contain no captured keys, pointer history or capability token. Loaded coordinates and colors are validated before rendering. The overlay/WebSocket remain read-only; appearance changes originate in the native application.

The embedded English/French help uses the same localhost token and Host/Origin checks. It is static HTML with no scripts or external assets. Canvas configuration does not control OBS remotely.

## Reporting a vulnerability

Do not disclose vulnerabilities in public issues or include actual captured secrets in a report. Include the affected version, OS and minimal reproduction steps with synthetic input.

Use [GitHub private vulnerability reporting](https://github.com/sjeje42/keycast-bridge/security/advisories/new), or open **Security → Advisories → Report a vulnerability**. Private vulnerability reporting is enabled for this repository; reports are shared privately with the maintainers rather than posted as public issues.

A GitHub account is required. The GitHub `noreply` address in package metadata cannot receive security reports.
