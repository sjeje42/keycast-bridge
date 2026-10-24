import assert from "node:assert/strict";
import fs from "node:fs";
// Load the pure ES module without imposing package.json on the Shell extension.
const source = fs.readFileSync(
  new URL("../data/gnome-extension/model.js", import.meta.url),
  "utf8",
);
const { Scene, validate, zoomOrigin } = await import(
  `data:text/javascript;base64,${Buffer.from(source).toString("base64")}`
);
const settings = {
  ring: true,
  trail: true,
  trail_style: "solid",
  shake: true,
  spotlight: true,
  spotlight_shape: "circle",
  spotlight_radius: 150,
  dimming: 65,
  pen_color: "#ff4444",
  pen_width: 4,
  zoom: 2,
};
const packet = {
  request: { enabled: true, mode: "normal", serial: 1 },
  appearance: {
    presentation: settings,
    ring: {
      color: "#a879ff",
      radius: 30,
      thickness: 3,
      neon: true,
      pulse: true,
    },
  },
};
for (const style of ["solid", "dash", "dot", "glow", "particle", "star"]) {
  packet.appearance.presentation.trail_style = style;
  assert.equal(
    validate(JSON.stringify(packet)).appearance.presentation.trail_style,
    style,
  );
}
for (const [key, value] of [
  ["dimming", 100],
  ["zoom", null],
  ["pen_width", 0],
  ["pen_color", "red"],
  ["trail", "yes"],
  ["trail_style", "unknown"],
  ["spotlight_shape", "triangle"],
]) {
  const invalid = structuredClone(packet);
  invalid.appearance.presentation[key] = value;
  assert.throws(() => validate(JSON.stringify(invalid)));
}
assert.throws(() => validate("x".repeat(16385)));
const straight = new Scene();
for (let i = 0; i < 10; i++) straight.move(i * 100, 0, i * 0.04, true);
assert.equal(straight.until, 0);
const jitter = new Scene();
for (let i = 0; i < 20; i++) jitter.move((i % 2) * 3, 0, i * 0.02, true);
assert.equal(jitter.until, 0);
const shake = new Scene();
for (let i = 0; i < 8; i++) shake.move(i % 2 ? 300 : 100, 0, i * 0.06, true);
assert.ok(shake.until > 0.8);
const until = shake.until;
shake.move(100, 0, 0.5, true);
assert.equal(shake.until, until);
const scene = new Scene();
scene.begin(settings);
for (let i = 0; i < 10000; i++) scene.move(i * 3, 0, i * 0.01, false);
assert.ok(scene.strokes[0].points.length <= 2048);
assert.ok(scene.trail.length <= 128);
scene.begin(settings);
scene.undo();
assert.equal(scene.strokes.length, 1);
scene.clear();
assert.equal(scene.strokes.length, 0);
for (let i = 0; i < 200; i++) scene.begin(settings);
assert.equal(scene.strokes.length, 128);
scene.move(0, 0, 200, false);
assert.equal(scene.trail.length, 1);
assert.deepEqual(zoomOrigin(0, 0, 1920, 1080, 2), [0, 0]);
assert.deepEqual(zoomOrigin(1920, 1080, 1920, 1080, 2), [960, 540]);
console.log(
  "Presentation: six styles, validation, shake discrimination, bounded memory, undo/clear, zoom bounds passed.",
);
