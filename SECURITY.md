# Security model / Confidentialité

Keycast Bridge intentionally reads global keyboard and optionally mouse events. It does not detect password fields or stop automatically on session lock/switch. Stop before entering secrets or locking. Shortcuts-only filtering reduces exposure; it is not a boundary against sensitive shortcuts.

## Linux

The GUI/server refuses root. `pkexec` launches an administrator-installed helper at a fixed path; the policy requests authorization on each start. The helper forks before starting threads:

- A root broker monitors udev, enumerates input event nodes, checks device numbers and evdev capabilities, and opens only the selected keyboards and optionally mice. It stays privileged for hotplug but never reads key events or connects to the network.
- An event reader drops supplementary groups, GID and UID, enables `NO_NEW_PRIVS`, and receives descriptors via `SCM_RIGHTS` on an inherited, private `AF_UNIX` socketpair. No public Unix socket is created. The reader uses epoll and keeps per-device modifier state; removal closes the descriptor and discards that state.

Only normalized labels, mouse buttons (no Linux pointer coordinates), readiness and device notifications reach the GUI via stdout. Stdin carries a heartbeat. Stop or application exit invalidates the generation and closes stdin; EOF or three seconds without heartbeat stops the reader, then the broker exits. Event node numbers are not stored as persistent identity. Selected devices use serial/name, falling back to physical USB path/name; this is convenience matching, not cryptographic device authentication. Input remains visible to the reader after privilege drop; filtering is application policy, not hardware isolation.

## Windows

The normal-user process uses `WH_KEYBOARD_LL` and optional `WH_MOUSE_LL` hooks on a dedicated thread. Callbacks queue bounded events and always pass input on to other applications. Overflow ends capture. No driver, service or elevation is installed. Mouse click coordinates are sent only to the local overlay to position the optional primary-monitor halo. No movement stream is retained. Secure desktops and elevated applications are outside the supported target.

## Shared boundaries and residual exposure

Only IPv4 localhost is bound. A random per-launch capability token protects the read-only overlay and WebSocket; Host/Origin checks reject foreign origins. No history, telemetry, external assets or keystroke logs. Late events from stopped generations are discarded; stop/disconnect clears the overlay. Slow WebSocket consumers are disconnected instead of accumulating unlimited events.

Other same-user software, administrators, a compromised GUI or malicious OBS browser extensions are outside the threat model. The token may appear in OBS configuration or browser history: do not share it. Restarting rotates it. The emergency shortcut consumes no keys; other applications receive it too. On Linux, press it on a captured keyboard, with all modifiers on that keyboard. No independent security audit is claimed.

Report vulnerabilities privately to the maintainer; do not include actual captured secrets.
