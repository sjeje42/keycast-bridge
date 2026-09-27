<script lang="ts">
  import { onMount } from 'svelte';
  let label = $state('');
  let modifiers = $state<string[]>([]);
  // A separate live row keeps held keys visible after a shortcut's history expires.
  let size = $state(40);
  let duration = 1800;
  let dark = $state(true);
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
            dark = !!event.dark; halo = !!event.halo;
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

<div class="stage" aria-live="polite">
  {#if label}
    <div class:light={!dark} class="keys history" style:font-size={`${size}px`}>
      {#each label.split(' + ') as key, i}
        {#if i}<span class="plus">+</span>{/if}<kbd>{key}</kbd>
      {/each}
    </div>
  {/if}
  {#if modifiers.length || mouseVisible}
    <div class:light={!dark} class="keys" style:font-size={`${size}px`}>
      {#each modifiers as key, i}
        {#if i}<span class="plus">+</span>{/if}<kbd class="held">{key}</kbd>
      {/each}
      {#if mouseVisible}
        <svg class="mouse" viewBox="0 0 48 64" role="img" aria-label="Mouse buttons">
          <rect x="3" y="3" width="42" height="58" rx="20" fill="none" stroke="currentColor" stroke-width="3" />
          <path d="M23 5 C10 5 5 14 5 29 L23 29Z" fill={buttons.includes(1) ? '#a879ff' : 'transparent'} />
          <path d="M25 5 C38 5 43 14 43 29 L25 29Z" fill={buttons.includes(2) ? '#ff9e64' : 'transparent'} />
          <path d="M24 3V30M4 30H44" fill="none" stroke="currentColor" stroke-width="2" />
          <rect x="20" y="11" width="8" height="15" rx="4" fill={buttons.includes(3) ? '#74dfba' : 'currentColor'} />
        </svg>
      {/if}
    </div>
  {/if}
</div>
{#if ring}
  {#key ring.id}<div class="ring" style:left={`${ring.x*100}%`} style:top={`${ring.y*100}%`} style:width={`${size*1.5}px`} style:height={`${size*1.5}px`}></div>{/key}
{/if}

<style>
  :global(html), :global(body), :global(#app) { margin:0; width:100%; height:100%; background:transparent !important; overflow:hidden; }
  .stage { position:fixed; inset:0; display:flex; flex-direction:column; gap:8px; align-items:center; justify-content:flex-end; padding:32px; box-sizing:border-box; pointer-events:none; }
  .held { outline:2px solid #a879ff; }
  .keys { display:flex; flex-wrap:wrap; gap:.24em; align-items:center; justify-content:center; font-family:system-ui,sans-serif; color:#f7f9ff; padding:.3em; border-radius:.4em; background:#10151fe8; box-shadow:0 8px 32px #0004; }
  kbd { font-family:inherit; font-weight:650; background:#273142; border:1px solid #ffffff26; border-bottom:3px solid #ffffff38; border-radius:.22em; padding:.18em .42em; }
  .mouse { height:1.5em; width:1.13em; margin:0 .2em; }
  .plus { font-size:.6em; opacity:.65; }
  .light { color:#172033; background:#f4f7ffed; }
  .light kbd { background:white; border-color:#17203330; }
  .ring { position:fixed; pointer-events:none; border:3px solid #a879ff; border-radius:50%; transform:translate(-50%,-50%); animation:pulse .5s ease-out forwards; }
  @keyframes pulse { from { opacity:1; scale:.7; } to { opacity:0; scale:1.4; } }
</style>
