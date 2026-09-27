# Validation — 0.2.0-alpha.4

The previous 0.2.0-alpha.2 passed all three build pipelines and the user confirmed it worked. This update adds Windows monitor selection and refreshes coordinates from the selected monitor.

## New verification

- Geometry tests cover secondary monitors to the left or above the primary, portrait layouts, shared boundaries, different resolutions, reordered/disconnected monitors and invalid rectangles.
- A state test verifies that changing the selected monitor clears the old overlay without stopping capture, and stopping preserves the selection for this application session.
- The Windows test suite queries the CI runner's real primary monitor using the native enumeration API.
- Cross-compilation checks cover the Windows backend and GTK interface; native Windows build and execution remain required by CI.

## Publication gates

The [Actions pipelines](https://github.com/sjeje42/keycast-bridge/actions) must all succeed for the published commit: Linux Clippy/tests/udev hotplug/HTTP smoke, Debian package install/launch/remove/reinstall, and Windows tests/build/portable launch/HTTP smoke. The portable Windows launch test temporarily hides the build runtime directory to catch external DLL dependencies.

## Physical validation

A CI runner with one virtual monitor does not validate a real multi-monitor OBS setup. Follow [TESTING.md](TESTING.md), especially mixed DPI, monitor rotation, unplug/replug and matching OBS source geometry. Linux/Wayland still has no pointer-position halo.

