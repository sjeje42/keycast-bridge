<script lang="ts">
  import { onMount } from 'svelte';
  let label = $state('');
  let modifiers = $state<string[]>([]);
  // A separate live row keeps held keys visible after a shortcut's history expires.
  let size = $state(40);
  let duration = 1800;
  const defaults = {canvas_width:1920, canvas_height:1080, x:50, y:100, background:'#10151f', key_background:'#273142', text:'#f7f9ff', accent:'#a879ff', right_click:'#ff9e64', middle_click:'#74dfba'};
  let appearance = $state({...defaults});
  let viewportWidth = $state(1920);
  let viewportHeight = $state(1080);
  let canvasScale = $derived(Math.min(viewportWidth / appearance.canvas_width, viewportHeight / appearance.canvas_height));
  function readAppearance(value: unknown, dark: boolean) {
    const result = {...defaults};
    if (!dark) Object.assign(result, {background:'#f4f7ff', key_background:'#ffffff', text:'#172033'});
    if (value && typeof value === 'object') {
      const input = value as Record<string, unknown>;
      for (const dimension of ['canvas_width','canvas_height'] as const) {
        if (typeof input[dimension] === 'number' && Number.isFinite(input[dimension])) result[dimension] = Math.round(Math.min(7680, Math.max(160, input[dimension])));
      }
      for (const axis of ['x','y'] as const) {
        if (typeof input[axis] === 'number' && Number.isFinite(input[axis])) result[axis] = Math.min(100, Math.max(0, input[axis]));
      }
      for (const key of ['background','key_background','text','accent','right_click','middle_click'] as const) {
        if (typeof input[key] === 'string' && /^#[0-9a-f]{6}$/i.test(input[key])) result[key] = input[key];
      }
    }
    return result;
  }
  let halo = false;
  let buttons = $state<number[]>([]);
  const releaseTimers = new Map<number, ReturnType<typeof setTimeout>>();
  const pressedAt = new Map<number, number>();
  let mouseVisible = $state(false);
  let ring = $state<{x:number,y:number,id:number}|null>(null);
  let ringId = 0;
  let expiry: ReturnType<typeof setTimeout>;
  let mouseExpiry: ReturnType<typeof setTimeout>;
  let ringExpiry: ReturnType<typeof setTimeout>;
  function clear() { releaseTimers.forEach(clearTimeout); releaseTimers.clear(); pressedAt.clear(); clearTimeout(expiry); clearTimeout(mouseExpiry); clearTimeout(ringExpiry); label = ''; modifiers = []; buttons = []; mouseVisible = false; ring = null; }
  onMount(() => {
    let alive = true;
    let socket: WebSocket;
    let retry: ReturnType<typeof setTimeout>;
    const token = location.pathname.split('/').pop();
    function connect() {
      socket = new WebSocket(`ws://${location.host}/ws/${token}`);
      socket.onmessage = ({data}) => {
        try {
          const event = JSON.parse(data);
          if (event.type === 'clear') clear();
          if (event.type === 'modifiers' && Array.isArray(event.keys)) {
            modifiers = [...new Set<string>(event.keys.filter((key: unknown): key is string => typeof key === 'string' && ['Ctrl', 'Alt', 'Shift', 'Super', 'Win', 'AltGr'].includes(key)))];
          }
          if (event.type === 'config') {
            size = Math.min(96, Math.max(20, Number(event.size) || 40));
            duration = Math.min(5000, Math.max(300, Number(event.duration) || 1800));
            appearance = readAppearance(event.appearance, event.dark !== false); halo = !!event.halo;
            if (!halo) ring = null;
          }
          if (event.type === 'key' && typeof event.label === 'string') {
            clearTimeout(expiry); label = event.label.slice(0, 100);
            expiry = setTimeout(() => label = '', duration);
          }
          if (event.type === 'mouse' && [1,2,3].includes(event.button)) {
            clearTimeout(mouseExpiry);
            mouseVisible = true;
            clearTimeout(releaseTimers.get(event.button));
            if (event.pressed) {
              pressedAt.set(event.button, performance.now());
              buttons = [...new Set([...buttons,event.button])];
            } else {
              const button = event.button;
              const remaining = Math.max(0, 150 - (performance.now() - (pressedAt.get(button) ?? 0)));
              releaseTimers.set(button, setTimeout(() => {
                buttons = buttons.filter(b => b !== button);
                releaseTimers.delete(button); pressedAt.delete(button);
                if (!buttons.length) mouseExpiry = setTimeout(() => mouseVisible = false, duration);
              }, remaining));
            }
            if (halo && event.pressed && typeof event.x === 'number' && typeof event.y === 'number' && event.x >= 0 && event.x <= 1 && event.y >= 0 && event.y <= 1) {
              clearTimeout(ringExpiry);
              ring = {x:event.x,y:event.y,id:++ringId};
              ringExpiry = setTimeout(() => ring = null, 500);
            }
          }
        } catch { clear(); }
      };
      socket.onclose = () => { clear(); if (alive) retry = setTimeout(connect, 1500); };
      socket.onerror = () => socket.close();
    }
    connect();
    return () => { alive = false; clearTimeout(retry); clear(); socket?.close(); };
  });
</script>

<svelte:window bind:innerWidth={viewportWidth} bind:innerHeight={viewportHeight} />
<div class="canvas" style:width={`${appearance.canvas_width}px`} style:height={`${appearance.canvas_height}px`} style:left={`${(viewportWidth - appearance.canvas_width * canvasScale) / 2}px`} style:top={`${(viewportHeight - appearance.canvas_height * canvasScale) / 2}px`} style:transform={`scale(${canvasScale})`}>
<div class="theme" style:--background={`${appearance.background}e8`} style:--key-background={appearance.key_background} style:--text={appearance.text} style:--accent={appearance.accent} style:--border={`${appearance.text}38`}>
<div class="stage" aria-live="polite">
 <div class="overlay" style:left={`${appearance.x}%`} style:top={`${appearance.y}%`} style:transform={`translate(-${appearance.x}%, -${appearance.y}%)`} style:align-items={appearance.x < 34 ? 'flex-start' : appearance.x > 66 ? 'flex-end' : 'center'}>
  {#if label}
    <div class="keys history" style:font-size={`${size}px`}>
      {#each label.split(' + ') as key, i}
        {#if i}<span class="plus">+</span>{/if}<kbd>{key}</kbd>
      {/each}
    </div>
  {/if}
  {#if modifiers.length || mouseVisible}
    <div class="keys" style:font-size={`${size}px`}>
      {#each modifiers as key, i}
        {#if i}<span class="plus">+</span>{/if}<kbd class="held">{key}</kbd>
      {/each}
      {#if mouseVisible}
        <svg class="mouse" viewBox="0 0 48 64" role="img" aria-label="Mouse buttons">
          <rect x="3" y="3" width="42" height="58" rx="20" fill="none" stroke="currentColor" stroke-width="3" />
          <path d="M23 5 C10 5 5 14 5 29 L23 29Z" fill={buttons.includes(1) ? appearance.accent : 'transparent'} />
          <path d="M25 5 C38 5 43 14 43 29 L25 29Z" fill={buttons.includes(2) ? appearance.right_click : 'transparent'} />
          <path d="M24 3V30M4 30H44" fill="none" stroke="currentColor" stroke-width="2" />
          <rect x="20" y="11" width="8" height="15" rx="4" fill={buttons.includes(3) ? appearance.middle_click : 'currentColor'} />
        </svg>
      {/if}
    </div>
  {/if}
</div>
</div>
{#if ring}
  {#key ring.id}<div class="ring" style:left={`${ring.x*100}%`} style:top={`${ring.y*100}%`} style:width={`${size*1.5}px`} style:height={`${size*1.5}px`}></div>{/key}
{/if}
</div>
</div>

<style>
  :global(html), :global(body), :global(#app) { margin:0; width:100%; height:100%; background:transparent !important; overflow:hidden; }
  .canvas { position:fixed; transform-origin:0 0; pointer-events:none; }
  .stage { position:absolute; inset:32px; pointer-events:none; }
  .overlay { position:absolute; display:flex; flex-direction:column; gap:8px; width:max-content; max-width:100%; }
  .keys { max-width:100%; box-sizing:border-box; }
  kbd { max-width:100%; box-sizing:border-box; overflow-wrap:anywhere; min-width:0; }
  .held { outline:2px solid var(--accent); }
  .keys { display:flex; flex-wrap:wrap; gap:.24em; align-items:center; justify-content:center; font-family:system-ui,sans-serif; color:var(--text); padding:.3em; border-radius:.4em; background:var(--background); box-shadow:0 8px 32px #0004; }
  kbd { font-family:inherit; font-weight:650; background:var(--key-background); border:1px solid var(--border); border-bottom:3px solid var(--border); border-radius:.22em; padding:.18em .42em; }
  .mouse { height:1.5em; width:1.13em; margin:0 .2em; }
  .plus { font-size:.6em; opacity:.65; }
  .ring { position:absolute; pointer-events:none; border:3px solid var(--accent); border-radius:50%; transform:translate(-50%,-50%); animation:pulse .5s ease-out forwards; }
  @keyframes pulse { from { opacity:1; scale:.7; } to { opacity:0; scale:1.4; } }
</style>
