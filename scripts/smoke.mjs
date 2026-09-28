// Run after: cargo build --no-default-features --bin keycast-bridge-demo
import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { fileURLToPath } from 'node:url';
import { setTimeout as delay } from 'node:timers/promises';
const root = fileURLToPath(new URL('../', import.meta.url));
const binary = process.env.KEYCAST_DEMO || root + 'target/debug/keycast-bridge-demo' + (process.platform === 'win32' ? '.exe' : '');
const child = spawn(binary, [], { cwd: root });
let output = '';
let socket;
child.stdout.on('data', chunk => { output += chunk; });
child.stderr.on('data', chunk => process.stderr.write(chunk));
try {
  for (let i = 0; i < 100 && !output.includes('/overlay/'); i++) await delay(50);
  const url = output.match(/http:\/\/127\.0\.0\.1:48732\/overlay\/[a-f0-9]+/)?.[0];
  assert.ok(url, 'Demo failed to start (is the port in use?)');
  assert.equal((await fetch(url)).status, 200);
  for (const language of ['en', 'fr']) {
    const help = url.replace('/overlay/', '/help/') + '/' + language + '.html';
    const response = await fetch(help);
    assert.equal(response.status, 200);
    assert.ok((await response.text()).includes('0.2.0-alpha.7'));
    assert.equal((await fetch(help.replace(/\/help\/[^/]+\//, '/help/wrong/'))).status, 403);
  }
  assert.equal((await fetch(url.replace(/[^/]+$/, 'wrong'))).status, 403);
  assert.equal((await fetch(url, { headers: { Origin: 'https://foreign.example' } })).status, 403);
  socket = new WebSocket(url.replace('http:', 'ws:').replace('/overlay/', '/ws/'));
  const events = [];
  socket.onmessage = message => events.push(JSON.parse(message.data));
  await Promise.race([once(socket, 'open'), delay(5000).then(() => { throw Error('WebSocket timeout'); })]);
  for (let i = 0; i < 100 && !events.some(e => e.type === 'key'); i++) await delay(50);
  assert.equal(events[0].type, 'config');
  assert.ok(events.some(e => e.type === 'key' && e.label === 'Ctrl + C'));
  console.log('PASS: HTTP overlay, token and Origin rejection, live WebSocket config + synthetic shortcut.');
} finally {
  socket?.close();
  child.kill('SIGTERM');
}
