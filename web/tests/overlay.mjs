import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { chromium } from 'playwright';

const server = createServer(async (req, res) => {
  const path = req.url === '/assets/overlay.js' ? 'assets/overlay.js' : req.url === '/assets/overlay.css' ? 'assets/overlay.css' : 'index.html';
  res.setHeader('Content-Type', path.endsWith('.js') ? 'text/javascript' : path.endsWith('.css') ? 'text/css' : 'text/html');
  res.end(await readFile(new URL('../dist/' + path, import.meta.url)));
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
let browser;
try {
  browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  await page.addInitScript(() => {
    window.WebSocket = class {
      constructor() { window.overlaySocket = this; }
      close() { this.onclose?.(); }
    };
  });
  await page.goto(`http://127.0.0.1:${server.address().port}/overlay/test`);
  await page.waitForFunction(() => window.overlaySocket);
  const send = async event => {
    await page.evaluate(event => window.overlaySocket.onmessage({data: JSON.stringify(event)}), event);
  };
  await send({type:'config', size:40, duration:300, dark:true, halo:false});
  await send({type:'modifiers', keys:['Ctrl','Shift']});
  await send({type:'mouse', button:1, pressed:true});
  await send({type:'key', label:'Ctrl + C'});
  await page.waitForTimeout(700);
  assert.deepEqual(await page.locator('kbd.held').allTextContents(), ['Ctrl','Shift']);
  assert.equal(await page.locator('.history').count(), 0, 'shortcut expires independently');
  assert.equal(await page.locator('svg path').first().getAttribute('fill'), '#a879ff');
  await send({type:'modifiers', keys:['Shift']});
  assert.deepEqual(await page.locator('kbd.held').allTextContents(), ['Shift']);
  await send({type:'modifiers', keys:[]});
  assert.equal(await page.locator('kbd.held').count(), 0, 'release updates during drag');
  await send({type:'mouse', button:1, pressed:false});
  await page.waitForTimeout(400);
  assert.equal(await page.locator('.keys').count(), 0);
  await send({type:'modifiers', keys:['AltGr']});
  assert.deepEqual(await page.locator('kbd.held').allTextContents(), ['AltGr']);
  await send({type:'clear'});
  assert.equal(await page.locator('.keys').count(), 0);
  await send({type:'modifiers', keys:['Alt']});
  await page.evaluate(() => window.overlaySocket.close());
  assert.equal(await page.locator('.keys').count(), 0, 'disconnect clears held state');
  console.log('PASS: held modifiers, drag, independent shortcut expiry, releases, AltGr, clear and disconnect.');
} finally {
  await browser?.close();
  server.close();
}
