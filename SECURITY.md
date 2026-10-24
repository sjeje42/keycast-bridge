# Security model / Confidentialité

Keycast Bridge intentionally reads global keyboard and optionally mouse events. It does not detect password fields or stop automatically on session lock/switch. Stop before entering secrets or locking. Shortcuts-only filtering reduces exposure; it is not a boundary against sensitive shortcuts.

## Linux

The GUI/server refuses root. `pkexec` launches an administrator-installed helper at a fixed path; the policy requests authorization on each start. The helper forks before starting threads:

- A root broker monitors udev, enumerates input event nodes, checks device numbers and evdev capabilities, and opens only the selected keyboards and optionally mice. It stays privileged for hotplug but never reads key events or connects to the network.
- An event reader drops supplementary groups, GID and UID, enables `NO_NEW_PRIVS`, and receives descriptors via `SCM_RIGHTS` on an inherited, private `AF_UNIX` socketpair. No public Unix socket is created. The reader uses epoll and keeps per-device modifier state; removal closes the descriptor and discards that state.

Only normalized labels, held modifier states, mouse buttons (no Linux pointer coordinates), readiness and device notifications reach the GUI via stdout. Stdin carries a heartbeat. Stop or application exit invalidates the generation and closes stdin; EOF or three seconds without heartbeat stops the reader, then the broker exits. Event node numbers are not stored as persistent identity. Selected devices use serial/name, falling back to physical USB path/name; this is convenience matching, not cryptographic device authentication. Input remains visible to the reader after privilege drop; filtering is application policy, not hardware isolation.

## Windows

The normal-user process uses `WH_KEYBOARD_LL` and optional `WH_MOUSE_LL` hooks on a dedicated thread. Callbacks queue bounded events and always pass input on to other applications. Overflow ends capture. No driver, service or elevation is installed. Mouse click coordinates are sent only to the local overlay to position the optional selected-monitor halo. Keyboard/OBS capture does not retain a movement stream. The separately enabled presentation tools retain only a bounded, transient trail. Secure desktops and elevated applications are outside the supported target.

## Desktop presentation tools

These tools require a separate, explicit enable action and never resume automatically after application launch. Windows uses a topmost, click-through layered window for passive effects; annotation, whiteboard and frozen zoom temporarily receive mouse/keyboard input. Escape releases the mode, and Ctrl+Alt+F12 stops the tools and keyboard capture. Windows releases modal tools on focus loss and removes all effects on session lock/disconnect or display changes.

On GNOME 48, an explicitly enabled Shell extension renders effects inside the compositor. The GUI uses a dedicated session-bus connection, validated appearance messages and a unique-client lease. It never sends key events, drawings or screenshots over D-Bus. Client disappearance or a two-second heartbeat timeout clears effects, releases modal grabs and destroys the snapshot. Lock, Overview and monitor changes also stop the tools. A stale heartbeat cannot reopen a dismissed mode. The read-only `State` method exposes mode, dimensions and counts, never input coordinates or drawing/image contents. Same-user processes remain outside the security boundary; the lease prevents accidental competing controllers, not a compromised session.

Cursor trails and shake samples expire in under a second and are capped. Drawing is capped at 128 strokes and 2048 points per stroke (long strokes are simplified). Frozen zoom temporarily holds a desktop image in process/compositor memory. These are never written to files or sent to the OBS server. Application memory, swap, core dumps and screen-recording software remain outside this guarantee. Styles are saved; active modes, strokes and screenshots are not. The keyboard-capture lock limitation described above still applies when desktop tools are disabled.

## Shared boundaries and residual exposure

Only IPv4 localhost is bound. A random per-launch capability token protects the read-only overlay and WebSocket; Host/Origin checks reject foreign origins. No history, telemetry, external assets or keystroke logs. Late events from stopped generations are discarded; stop/disconnect clears the overlay. Slow WebSocket consumers are disconnected instead of accumulating unlimited events.

Other same-user software, administrators, a compromised GUI or malicious OBS browser extensions are outside the threat model. The token may appear in OBS configuration or browser history: do not share it. Restarting rotates it. The keyboard-capture emergency shortcut consumes no keys. Desktop presentation shortcuts are registered while tools are enabled and can consume those combinations. On Linux, press it on a captured keyboard, with all modifiers on that keyboard. No independent security audit is claimed.

Appearance preferences (position, colors, pointer effects, drawing style, zoom factor and canvas dimensions only) are written to the user configuration directory. They contain no captured keys, pointer history or capability token. Loaded coordinates and colors are validated before rendering. The overlay/WebSocket remain read-only; appearance changes originate in the native application.

The embedded English/French help uses the same localhost token and Host/Origin checks. It is static HTML with no scripts or external assets. Canvas configuration does not control OBS remotely.

## Reporting a vulnerability

Do not disclose vulnerabilities in public issues or include actual captured secrets in a report. Include the affected version, OS and minimal reproduction steps with synthetic input.

Use [GitHub private vulnerability reporting](https://github.com/sjeje42/keycast-bridge/security/advisories/new), or open **Security → Advisories → Report a vulnerability**. Private vulnerability reporting is enabled for this repository; reports are shared privately with the maintainers rather than posted as public issues.

A GitHub account is required. The GitHub `noreply` address in package metadata cannot receive security reports.
