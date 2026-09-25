<script lang="ts">
  import { onMount } from 'svelte';
  let label = $state('');
  let size = $state(40);
  let duration = 1800;
  let dark = $state(true);
  let expiry: ReturnType<typeof setTimeout>;
  function clear() { clearTimeout(expiry); label = ''; }
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
          if (event.type === 'config') {
            size = Math.min(96, Math.max(20, Number(event.size) || 40));
            duration = Math.min(5000, Math.max(300, Number(event.duration) || 1800));
            dark = !!event.dark;
          }
          if (event.type === 'key' && typeof event.label === 'string') {
            clear(); label = event.label.slice(0, 100);
            expiry = setTimeout(clear, duration);
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
    <div class:light={!dark} class="keys" style:font-size={`${size}px`}>
      {#each label.split(' + ') as key, i}
        {#if i}<span class="plus">+</span>{/if}<kbd>{key}</kbd>
      {/each}
    </div>
  {/if}
</div>

<style>
  :global(html), :global(body), :global(#app) { margin:0; width:100%; height:100%; background:transparent !important; overflow:hidden; }
  .stage { position:fixed; inset:0; display:flex; align-items:flex-end; justify-content:center; padding:32px; box-sizing:border-box; }
  .keys { display:flex; flex-wrap:wrap; gap:.24em; align-items:center; justify-content:center; font-family:system-ui,sans-serif; color:#f7f9ff; padding:.3em; border-radius:.4em; background:#10151fe8; box-shadow:0 8px 32px #0004; }
  kbd { font-family:inherit; font-weight:650; background:#273142; border:1px solid #ffffff26; border-bottom:3px solid #ffffff38; border-radius:.22em; padding:.18em .42em; }
  .plus { font-size:.6em; opacity:.65; }
  .light { color:#172033; background:#f4f7ffed; }
  .light kbd { background:white; border-color:#17203330; }
</style>
