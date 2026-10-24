// Pure bounded scene state; no Shell or input access in this module.
export class Scene {
    constructor() { this.pointer = [0, 0]; this.trail = []; this.clicks = []; this.strokes = []; this.drawing = false; this.shake = []; this.until = 0; this.cooldown = 0; }
    move(x, y, t, locate) {
        if (Math.hypot(x - this.pointer[0], y - this.pointer[1]) >= 2) {
            const p = [x, y, t]; this.trail.push(p); this.shake.push(p);
            if (this.drawing) {
                const stroke = this.strokes.at(-1);
                if (stroke.points.length >= 2048) stroke.points = stroke.points.filter((_, i) => i % 2 === 0);
                stroke.points.push([x, y]);
            }
        }
        this.pointer = [x, y];
        this.trail = this.trail.filter(p => t - p[2] <= 0.65).slice(-128);
        this.clicks = this.clicks.filter(p => t - p[2] <= 0.5).slice(-8);
        this.shake = this.shake.filter(p => t - p[2] <= 0.6).slice(-64);
        let distance = 0, reversals = 0;
        for (let i = 1; i < this.shake.length; i++) {
            const a = this.shake[i - 1], b = this.shake[i];
            distance += Math.hypot(b[0] - a[0], b[1] - a[1]);
            if (i > 1) { const c = this.shake[i - 2]; if ((a[0] - c[0]) * (b[0] - a[0]) + (a[1] - c[1]) * (b[1] - a[1]) < -100) reversals++; }
        }
        if (locate && t >= this.cooldown && distance > 500 && reversals >= 3) { this.until = t + 0.9; this.cooldown = t + 1.5; this.shake = []; }
    }
    begin(settings) {
        this.strokes = this.strokes.slice(-127);
        this.strokes.push({points: [this.pointer], color: settings.pen_color, width: settings.pen_width});
        this.drawing = true;
    }
    undo() { this.drawing = false; this.strokes.pop(); }
    clear() { this.drawing = false; this.strokes = []; }
}
export function zoomOrigin(x, y, w, h, z) {
    return [Math.max(0, Math.min(w - w / z, x - w / z / 2)), Math.max(0, Math.min(h - h / z, y - h / z / 2))];
}
export function validate(payload) {
    if (typeof payload !== 'string' || payload.length > 16384) throw new Error('Invalid request size');
    const p = JSON.parse(payload), a = p.appearance, s = a?.presentation, r = a?.ring, q = p.request;
    if (!q || typeof q.enabled !== 'boolean' || !Number.isSafeInteger(q.serial) || q.serial < 0 || !['normal','annotate','whiteboard','zoom'].includes(q.mode)) throw new Error('Invalid command');
    if (!s || !r) throw new Error('Missing settings');
    for (const k of ['ring','trail','shake','spotlight']) if (typeof s[k] !== 'boolean') throw new Error('Invalid flag');
    for (const k of ['neon','pulse']) if (typeof r[k] !== 'boolean') throw new Error('Invalid ring flag');
    if (!['solid','dash','dot','glow','particle','star'].includes(s.trail_style) || !['circle','square'].includes(s.spotlight_shape)) throw new Error('Invalid style');
    for (const c of [s.pen_color, r.color]) if (!/^#[0-9a-f]{6}$/i.test(c)) throw new Error('Invalid color');
    for (const [v, lo, hi] of [[s.spotlight_radius,40,600],[s.dimming,0,90],[s.pen_width,1,40],[s.zoom,1.25,8],[r.radius,10,200],[r.thickness,1,20]]) if (!Number.isFinite(v) || v < lo || v > hi) throw new Error('Invalid dimension');
    return p;
}
