# Keycast Bridge — User guide

Version **0.2.0-alpha.8** · [Français](USER_GUIDE.fr.md)

Keycast Bridge shows keyboard shortcuts, held modifier keys and optional mouse clicks in a transparent OBS Browser Source. The native application controls capture and appearance; OBS combines the overlay with your screen or application capture.

## 1. Install and launch

### Windows x64

Download the Windows ZIP from the project's GitHub Releases. Extract the **entire archive** into a writable folder and open `keycast-bridge.exe`. Keep the DLLs and subfolders beside the executable. There is no installer, driver or background service. Use the application as a normal user. The alpha is unsigned; secure desktops, UAC prompts and elevated applications are outside its supported capture target.

### Debian 13 amd64

Download the Debian package from Releases, open a terminal in its folder and run:

```sh
sudo apt install ./keycast-bridge_0.2.0.alpha.8-1_amd64.deb
```

Open Keycast Bridge from the application menu. Do **not** run the graphical application with sudo. Starting keyboard capture requests administrator authorization through Polkit; the capture helper manages the input devices. You do not need to add yourself to the `input` group.

OBS must provide a **Browser Source**. If your Linux OBS installation does not have it, the official OBS Flatpak is the documented option for this project. The Keycast Bridge Debian package does not install OBS.

### Other Linux distributions

Ubuntu 24.04+, Linux Mint 22.x, Fedora, Manjaro and Arch installation-from-source instructions are in the repository README. Use the dependencies and build commands for your distribution; the downloadable Debian package targets Debian 13. Other desktops/compositors need physical validation. Requirements include GTK 4.8+, Rust, Node.js 22, libudev and libxkbcommon development files.

### Updates and removal

For Windows, stop capture, close the old application and extract the new ZIP into a new folder. For Debian, install the new `.deb` with APT. Appearance/canvas preferences are stored separately from the executable and survive updates. To remove the Debian package, use `sudo apt remove keycast-bridge`; the user's preference file remains. Only one instance can use the local server port at a time.

## 2. First setup in OBS

1. Open Keycast Bridge. Use the language selector at the top if needed.
2. Click the **gear at the top right**, then open **Canvas and size**. Choose your source size, for example **1920 × 1080**.
3. Return to the main window and click **Copy OBS URL**.
4. In OBS, choose **Sources → + → Browser**. Create a source and paste the URL.
5. Enter the **same width and height** in the Browser Source properties. Keycast Bridge displays the current values beside the URL instructions.
6. Put the Browser Source **above** the screen/application capture in OBS's source list. Align their canvas rectangles. Keep the overlay background transparent.
7. Click **Test overlay** in Keycast Bridge. A sample shortcut appears briefly. **This button stops active capture**; it is intended for checking the output before recording.
8. Choose capture options, click **Start**, and authorize capture on Linux. Use a shortcut or hold Shift to verify the live display.

The URL changes whenever Keycast Bridge restarts. **Copy the new URL into OBS after every application launch.** An old URL does not reconnect to the new session. Do not share this private local URL.

## 3. Main window

The compact main window contains the capture status, mouse options, Start/Stop/Test buttons, OBS URL, current canvas dimensions and preview controls. It is sized to the detected displays. A scrollbar remains available on unusually small displays or at large scaling factors.

| Control | Purpose |
| --- | --- |
| Start | Begin capture with the options currently selected. |
| Stop | End capture and clear the overlay. |
| Ctrl + Alt + F12 | Emergency stop. The combination is also delivered to the active application. |
| Test overlay | Stop capture and display a sample shortcut for checking OBS. |
| Copy OBS URL | Copy this session's local overlay address. |
| Open preview | Open the overlay in your browser; it is transparent and can look empty when no input is displayed. |
| Gear | Open the reusable Settings window without stopping capture. |

Closing Settings keeps the application and capture running. Closing the main application stops capture. The appearance and canvas controls can be adjusted while recording. Keyboard selection, layout, text-key capture and the mouse-capture option require stopping and starting capture.

## 4. Settings — Position and colors

### Place the overlay

Drag the sample block in the schematic preview. Alternatively choose one of the **nine position presets**, or enter **X/Y percentages** using the keyboard.

- X = 0 means the left edge; X = 100 means the right edge.
- Y = 0 means the top edge; Y = 100 means the bottom edge.
- 50/50 centers the block. The default is 50/100, bottom center.
- Percentages describe the **available travel space**, accounting for the block's own size. A safety margin keeps the block away from the canvas edges.

The shortcut row and live modifier/mouse row move together. Their alignment follows the horizontal position. The preview shows the chosen canvas proportions, but its key sample is schematic: use the actual OBS output to check exact font size and content. Cropping the Browser Source in OBS can still hide content.

### Customize the palette

| Color | Affects |
| --- | --- |
| Background | The translucent background of each row. |
| Key background | The surface of each key. |
| Text | Letters, separators and mouse outline. |
| Accent / left click | Held-key outlines, left mouse button and Windows click ring. |
| Right click | Right mouse button highlight. |
| Middle click | Middle mouse button highlight. |

Click a swatch to open the color chooser. The **Dark palette** and **Light palette** buttons restore coordinated colors while preserving position and canvas dimensions. **Reset all** in this tab resets position and colors only; it does not change canvas size or capture options.

## 5. Settings — Canvas and size

**Size** controls key text in logical canvas pixels, from 20 to 96. **Duration** controls how long the last shortcut remains visible, from 300 to 5000 ms. Held modifiers remain visible until release, irrespective of Duration.

The canvas presets are 1280 × 720, 1920 × 1080, 2560 × 1440, 3840 × 2160, 1080 × 1920 and 1080 × 1080. Custom width and height each range from 160 to 7680 pixels.

These values define the overlay's **logical canvas** and update the preview and rendering. **They do not change OBS Browser Source properties remotely.** Enter the same dimensions in OBS for the intended output. If the actual browser viewport has different dimensions, the canvas scales uniformly to fit it; a different aspect ratio adds transparent margins rather than distorting the overlay. Consequently, text size may appear scaled in a mismatched viewport.

Example: for a vertical 1080 × 1920 tutorial, choose that preset in the application, set the OBS Browser Source to 1080 × 1920 and align it with your vertical scene. For a Windows pointer ring over a full monitor capture, choose dimensions with the monitor's aspect ratio and align both OBS sources without cropping.

## 6. Settings — Capture

### Keyboard selection and layout

On **Linux**, the default captures all recognized keyboards, including newly attached keyboards. You can instead select specific keyboards in Settings and refresh the device list. Set the keyboard layout to match your desktop: French AZERTY, US or UK QWERTY, or German QWERTZ. Unplugging a keyboard drops its held-key state. Reconnecting or replacing keyboards in automatic mode does not require restarting OBS.

On **Windows**, all keyboards in the current session are captured and the layout follows the foreground window. Individual keyboard selection is not available in this alpha.

### Shortcuts, text and held modifiers

By default, ordinary text characters are filtered. Shortcuts such as Ctrl+C and named control keys can still appear. **Shift, Ctrl, Alt, Win/Super and AltGr** are displayed in a separate live row while held, even without another keyboard key. Left/right variants are combined so releasing one side does not hide a modifier still held on the other side.

This supports workflows such as the Photoshop/Affinity Pen tool: hold Shift, Ctrl or Alt, then click or drag. Enable **Show mouse clicks** to show the mouse buttons beside the modifiers. The last shortcut remains in a separate temporary row.

**Also show text keys** expands capture to ordinary text. It can reveal private information and passwords. The application does not detect password fields; stop capture before entering secrets, including when using shortcut-only mode.

### Mouse and multiple monitors

Enable **Show mouse clicks** before starting capture. Left, right and middle buttons have independent colors. A very short click is held visually for at least 150 ms; a held button remains highlighted through a drag. There is no scroll-wheel movement display.

On **Windows**, enable **Click ring** before starting: this also enables **Show mouse clicks**. The ring works with one monitor, selected automatically. If other monitors are unplugged, it automatically follows the sole remaining monitor. With multiple monitors, select the monitor captured in OBS. In **Settings → Capture → Monitor captured in OBS**, choose the display used in OBS. Use its resolution and desktop position to identify it; DISPLAY numbers need not match OBS's source order. The selection can change during capture. Keyboard overlay position does not move the pointer ring. Clicks on another monitor show button feedback but no ring on the selected monitor. If the selected monitor disappears and multiple monitors remain connected, its ring is suspended until an available monitor is selected; keyboard and button feedback continue.

The ring is drawn inside the OBS overlay, not directly on the Windows desktop. Accurate alignment requires matching the selected monitor capture and Browser Source rectangles and aspect ratio. Window captures or crops are not mapped automatically.

On **Linux/Wayland**, keyboard and button feedback work across screens, but pointer-position rings are not available.

## 7. Saved settings and privacy

Position, palette and canvas dimensions are saved automatically. Other controls—including size, duration, capture options, selected monitor and language—remain session-only in this alpha. A message in Settings reports a save failure while retaining the live changes for the current session.

Preference locations:

- Windows: `%APPDATA%\keycast-bridge\appearance.json`
- Linux: `$XDG_CONFIG_HOME/keycast-bridge/appearance.json`, or `~/.config/keycast-bridge/appearance.json`

This file contains only appearance/canvas preferences: no keystrokes, mouse history or OBS URL. If it is corrupt, the application falls back to defaults. To reset all saved preferences, close the application and rename this file; restart to use defaults.

The server listens only on IPv4 localhost, port 48732. There is no telemetry, key history or automatic capture at startup. The random session URL and Origin checks limit access but do not protect against malicious software running as the same user. Capture does not stop automatically when locking the session; stop it first. On Linux, use the emergency shortcut on one captured keyboard with all modifiers held on that keyboard.

## 8. Troubleshooting

| Problem | What to check |
| --- | --- |
| Browser Source is missing in Linux OBS | Use an OBS build that includes it; see the official OBS Flatpak option in the README. |
| Overlay is empty | Click Test overlay; check source visibility/order and the current session URL. Ordinary text is filtered by default. |
| OBS shows an old or failed page | Copy the URL again after restarting Keycast Bridge and refresh the Browser Source. |
| Position or size differs from the preview | The preview is schematic. Match canvas and OBS dimensions; check OBS crops/transforms and key Size. |
| Mouse buttons do not appear | Stop, enable Show mouse clicks, then Start again. |
| Ring is offset or missing | Windows only; enable it, select the correct monitor, match aspect ratios and align uncropped OBS sources. |
| Linux capture authorization fails | Check the installed package/helper and Polkit prompt. Do not launch the GUI as root. |
| Settings disappear after restarting | Only appearance/canvas settings persist. Check the save-failure message and preference-directory permissions. |
| Windows executable fails to launch | Extract the entire ZIP and keep its DLL/data folders. Do not copy the EXE alone. |
| Application fails immediately | Close another instance using port 48732. If needed, include the OS and version in a bug report. |

For a useful report, include the application version, OS, desktop/display scaling, chosen canvas, OBS source dimensions and reproducible steps. Screenshots are useful; hide the session URL and any private captured text.

### The guide or preview does not open

On Windows, the button uses the system default browser. If Windows rejects the launch, a dialog offers **Copy link**: paste it into your browser and keep Keycast Bridge running. Check the default browser in Windows settings. The offline guide is also available in `guide/en.html` beside the executable.
