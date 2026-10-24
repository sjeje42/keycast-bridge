import Clutter from "gi://Clutter";
import Gio from "gi://Gio";
import GLib from "gi://GLib";
import Shell from "gi://Shell";
import St from "gi://St";
import Meta from "gi://Meta";
import { Extension } from "resource:///org/gnome/shell/extensions/extension.js";
import * as Main from "resource:///org/gnome/shell/ui/main.js";
import { Scene, validate, zoomOrigin } from "./model.js";
import { draw } from "./drawing.js";

Gio._promisify(Shell.Screenshot.prototype, "screenshot_stage_to_content");
const XML = `<node><interface name="fr.jeromelab.KeycastBridge.Presentation"><method name="Update"><arg type="s" direction="in"/><arg type="s" direction="out"/></method><method name="State"><arg type="s" direction="out"/></method></interface></node>`;
const now = () => GLib.get_monotonic_time() / 1e6;

export default class Presentation extends Extension {
  enable() {
    this._owner = null;
    this._watch = 0;
    this._serial = -1;
    this._running = false;
    this._generation = 0;
    this._mode = "normal";
    this._scene = new Scene();
    this._buttons = 0;
    this._actor = new St.Widget({
      reactive: false,
      can_focus: true,
      visible: false,
    });
    this._image = new Clutter.Actor({ reactive: false });
    this._actor.add_child(this._image);
    this._canvas = new St.DrawingArea({ reactive: false });
    this._actor.add_child(this._canvas);
    this._canvas.connect("repaint", () => {
      if (!this._appearance) return;
      const c = this._canvas.get_context();
      try {
        draw(
          c,
          this._monitor.width,
          this._monitor.height,
          this._scene,
          this._appearance,
          this._mode,
          now(),
        );
      } finally {
        c.$dispose();
      }
    });
    this._actor.connect("event", (_actor, event) => this._event(event));
    Main.layoutManager.addTopChrome(this._actor, { affectsInputRegion: true });
    this._session = Main.sessionMode.connect("updated", () => this._stop());
    this._monitors = Main.layoutManager.connect("monitors-changed", () =>
      this._stop(),
    );
    this._overview = Main.overview.connect("showing", () => this._stop());
    this._bus = Gio.DBusExportedObject.wrapJSObject(XML, this);
    this._bus.export(
      Gio.DBus.session,
      "/fr/jeromelab/KeycastBridge/Presentation",
    );
    this._timer = GLib.timeout_add(GLib.PRIORITY_DEFAULT, 33, () => {
      try {
        this._tick();
      } catch (e) {
        console.error(e);
        this._stop();
      }
      return GLib.SOURCE_CONTINUE;
    });
  }
  UpdateAsync([payload], invocation) {
    try {
      const p = validate(payload),
        sender = invocation.get_sender();
      if (this._owner && sender !== this._owner)
        throw new Error("Presentation already owned by another client");
      if (!this._owner) {
        this._owner = sender;
        this._serial = -1;
        this._watch = Gio.bus_watch_name_on_connection(
          Gio.DBus.session,
          sender,
          Gio.BusNameWatcherFlags.NONE,
          null,
          () => {
            this._stop();
            this._releaseOwner();
          },
        );
      }
      this._heartbeat = now();
      this._appearance = p.appearance;
      const q = p.request;
      if (!q.enabled) this._stop();
      else if (q.serial !== this._serial) {
        this._serial = q.serial;
        if (
          Main.sessionMode.isLocked ||
          Main.sessionMode.isGreeter ||
          Main.overview.visible
        )
          this._stop();
        else {
          this._running = true;
          this._bindKeys();
          this._setMode(q.mode);
        }
      }
      invocation.return_value(
        new GLib.Variant("(s)", [this._running ? this._mode : "stopped"]),
      );
    } catch (e) {
      invocation.return_dbus_error(
        "fr.jeromelab.KeycastBridge.InvalidRequest",
        String(e.message),
      );
    }
  }
  State() {
    return JSON.stringify({
      enabled: this._running,
      mode: this._mode,
      grabbed: !!this._grab,
      strokes: this._scene.strokes.length,
      snapshot: !!this._image.get_content(),
      width: this._monitor?.width ?? 0,
      height: this._monitor?.height ?? 0,
    });
  }
  _releaseOwner() {
    if (this._watch) Gio.bus_unwatch_name(this._watch);
    this._watch = 0;
    this._owner = null;
  }
  _setMonitor() {
    const [x, y] = global.get_pointer();
    const m = Main.layoutManager.monitors.find(
      (r) => x >= r.x && y >= r.y && x < r.x + r.width && y < r.y + r.height,
    );
    if (!m) return false;
    if (
      !this._monitor ||
      m.x !== this._monitor.x ||
      m.y !== this._monitor.y ||
      m.width !== this._monitor.width ||
      m.height !== this._monitor.height
    )
      this._scene = new Scene();
    this._monitor = { x: m.x, y: m.y, width: m.width, height: m.height };
    this._actor.set_position(m.x, m.y);
    this._actor.set_size(m.width, m.height);
    this._actor.set_clip(0, 0, m.width, m.height);
    this._canvas.set_size(m.width, m.height);
    return true;
  }
  _bindKeys() {
    if (this._keys) return;
    const settings = this.getSettings(
      "org.gnome.shell.extensions.keycast-bridge",
    );
    this._keys = true;
    for (const [key, mode] of [
      ["keycast-draw", "annotate"],
      ["keycast-whiteboard", "whiteboard"],
      ["keycast-zoom", "zoom"],
      ["keycast-stop", null],
    ]) {
      Main.wm.addKeybinding(
        key,
        settings,
        Meta.KeyBindingFlags.NONE,
        Shell.ActionMode.ALL,
        () => {
          if (mode) this._setMode(mode);
          else this._stop();
        },
      );
    }
  }
  _unbindKeys() {
    if (this._keys)
      for (const key of [
        "keycast-draw",
        "keycast-whiteboard",
        "keycast-zoom",
        "keycast-stop",
      ])
        Main.wm.removeKeybinding(key);
    this._keys = false;
  }
  _ungrab() {
    if (this._grab) {
      const grab = this._grab;
      this._grab = null;
      try {
        Main.popModal(grab);
      } catch (error) {
        console.error(error);
      }
    }
    this._actor.reactive = false;
  }
  _setMode(mode) {
    this._generation++;
    this._ungrab();
    this._image.set_content(null);
    this._image.hide();
    this._scene.clear();
    if (!this._setMonitor()) {
      this._stop();
      return;
    }
    this._mode = mode;
    this._zoom = this._appearance.presentation.zoom;
    if (mode === "zoom") {
      this._snapshot(this._generation);
      return;
    }
    this._actor.show();
    if (mode !== "normal") this._grabInput();
  }
  _grabInput() {
    this._actor.reactive = true;
    this._grab = Main.pushModal(this._actor, {
      actionMode: Shell.ActionMode.ALL,
    });
    if (this._grab.get_seat_state() !== Clutter.GrabState.ALL) {
      this._stop();
      return;
    }
    this._actor.grab_key_focus();
  }
  async _snapshot(generation) {
    this._actor.hide();
    try {
      const [content] =
        await new Shell.Screenshot().screenshot_stage_to_content();
      if (generation !== this._generation || !this._running) return;
      this._image.set_content(content);
      this._image.set_size(global.stage.width, global.stage.height);
      this._image.show();
      this._actor.show();
      this._grabInput();
      this._pan();
    } catch (e) {
      console.error(e);
      this._stop();
    }
  }
  _pan() {
    if (this._mode !== "zoom") return;
    const m = this._monitor,
      [x, y] = this._scene.pointer;
    const [sx, sy] = zoomOrigin(x, y, m.width, m.height, this._zoom);
    this._image.set_scale(this._zoom, this._zoom);
    this._image.set_position(
      -(m.x + sx) * this._zoom,
      -(m.y + sy) * this._zoom,
    );
  }
  _tick() {
    if (!this._running) return;
    if (now() - this._heartbeat > 2 || Main.sessionMode.isLocked) {
      this._stop();
      return;
    }
    if (this._mode === "normal" && !this._setMonitor()) {
      this._stop();
      return;
    }
    const [x, y, buttons] = global.get_pointer(),
      m = this._monitor,
      t = now();
    this._scene.move(x - m.x, y - m.y, t, this._appearance.presentation.shake);
    const mask =
      Clutter.ModifierType.BUTTON1_MASK |
      Clutter.ModifierType.BUTTON2_MASK |
      Clutter.ModifierType.BUTTON3_MASK;
    if (buttons & mask & ~this._buttons)
      this._scene.clicks.push([...this._scene.pointer, t]);
    this._buttons = buttons & mask;
    this._pan();
    this._canvas.queue_repaint();
  }
  _event(event) {
    if (!this._running || this._mode === "normal")
      return Clutter.EVENT_PROPAGATE;
    const type = event.type();
    if (type === Clutter.EventType.KEY_PRESS) {
      const key = event.get_key_symbol(),
        ctrl = !!(event.get_state() & Clutter.ModifierType.CONTROL_MASK);
      if (key === Clutter.KEY_Escape) {
        this._setMode("normal");
        return Clutter.EVENT_STOP;
      }
      if (ctrl && (key === Clutter.KEY_z || key === Clutter.KEY_Z))
        this._scene.undo();
      if (key === Clutter.KEY_BackSpace) this._scene.clear();
      if (this._mode === "zoom") {
        if (key === Clutter.KEY_plus || key === Clutter.KEY_equal)
          this._zoom = Math.min(8, this._zoom + 0.25);
        if (key === Clutter.KEY_minus)
          this._zoom = Math.max(1.25, this._zoom - 0.25);
      }
      if (
        ctrl &&
        event.get_state() & Clutter.ModifierType.MOD1_MASK &&
        key === Clutter.KEY_F12
      )
        this._stop();
    } else if (
      type === Clutter.EventType.BUTTON_PRESS &&
      event.get_button() === 1 &&
      this._mode !== "zoom"
    ) {
      const [x, y] = event.get_coords();
      this._scene.move(x - this._monitor.x, y - this._monitor.y, now(), false);
      this._scene.begin(this._appearance.presentation);
    } else if (type === Clutter.EventType.BUTTON_RELEASE)
      this._scene.drawing = false;
    else if (type === Clutter.EventType.MOTION) {
      const [x, y] = event.get_coords();
      this._scene.move(x - this._monitor.x, y - this._monitor.y, now(), false);
    } else if (type === Clutter.EventType.SCROLL && this._mode === "zoom") {
      const direction = event.get_scroll_direction();
      let delta = direction === Clutter.ScrollDirection.UP ? 0.25 : -0.25;
      if (direction === Clutter.ScrollDirection.SMOOTH) {
        const [, dy] = event.get_scroll_delta();
        delta = -dy * 0.15;
      }
      this._zoom = Math.max(1.25, Math.min(8, this._zoom + delta));
    }
    this._canvas.queue_repaint();
    return Clutter.EVENT_STOP;
  }
  _stop() {
    this._running = false;
    this._generation++;
    this._unbindKeys();
    this._ungrab();
    this._mode = "normal";
    this._scene = new Scene();
    this._image.set_content(null);
    this._actor.hide();
  }
  disable() {
    this._stop();
    this._releaseOwner();
    if (this._timer) GLib.Source.remove(this._timer);
    this._bus.unexport();
    Main.sessionMode.disconnect(this._session);
    Main.layoutManager.disconnect(this._monitors);
    Main.overview.disconnect(this._overview);
    Main.layoutManager.removeChrome(this._actor);
    this._actor.destroy();
  }
}
