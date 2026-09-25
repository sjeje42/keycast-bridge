# Security model / Confidentialité

This application intentionally reads global keyboard events. It cannot infer password fields, focused applications, lock-screen state or active sessions from evdev. Stop before entering secrets or locking your session. Shortcuts-only filtering reduces exposure; it is not a security boundary against sensitive shortcuts.

## Boundaries

- The GTK process and HTTP/WebSocket server refuse to run as root.
- `pkexec` runs an administrator-installed, root-owned helper at a fixed path. Polkit requires `auth_admin` for each start; inactive/remote authorization is denied by the supplied policy.
- The helper accepts one canonical `/dev/input/eventN` character device, checks keyboard capabilities, opens it without grabbing it, and drops supplementary groups, GID and UID before processing events. It never opens a network socket or launches other processes.
- The anonymous stdout pipe carries normalized shortcut labels. The stdin pipe carries a heartbeat only. Closing the app or stopping invalidates the current session and closes that pipe. A three-second heartbeat timeout bounds orphan capture; the normal EOF path stops within the polling interval.
- Only localhost IPv4 is bound. A per-launch random 128-bit capability token protects the read-only overlay and WebSocket. Host and Origin validation reject foreign browser origins / hostnames. No permissive CORS, network logs, telemetry, key history or external assets are used.
- On stop, output is cleared and late messages from an old capture generation are discarded. A slow WebSocket consumer is disconnected instead of receiving an unbounded backlog. Disconnect clears the browser display.

## Residual exposure

The opened evdev descriptor necessarily remains readable after privilege drop. Filtering is application policy, not hardware isolation. Other software running as the same user, an administrator, a compromised GTK process or malicious OBS browser extensions are outside the threat model. The token appears in OBS configuration/browser history if pasted there. Do not share it. Reopening the application rotates it.

This alpha does not automatically stop at session lock or switch. Pause before locking. The emergency shortcut is recognized only on the selected device and consumes no keys: applications also receive that shortcut. No claim of independent security audit is made.

Report vulnerabilities privately to the project maintainer through a private GitHub security report once that facility has been enabled; do not include actual captured secrets in reports.
