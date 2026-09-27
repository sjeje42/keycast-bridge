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
  const appearance = {x:0, y:0, background:'#123456', key_background:'#234567', text:'#fedcba', accent:'#11aa33', right_click:'#ee2244', middle_click:'#2299ee'};
  const config = {type:'config', size:40, duration:5000, dark:true, halo:true, appearance};
  await send(config);
  await send({type:'modifiers', keys:['Ctrl','Shift']});
  await send({type:'key', label:'Ctrl + Shift + F12'});
  const css = await page.locator('kbd.held').first().evaluate(el => ({text:getComputedStyle(el).color, background:getComputedStyle(el).backgroundColor, outline:getComputedStyle(el).outlineColor}));
  assert.deepEqual(css, {text:'rgb(254, 220, 186)', background:'rgb(35, 69, 103)', outline:'rgb(17, 170, 51)'});
  for (const viewport of [{width:1920,height:1080}, {width:1280,height:720}, {width:720,height:1280}]) {
    await page.setViewportSize(viewport);
    for (const [x,y] of [[0,0],[100,0],[0,100],[100,100],[50,50],[23,61]]) {
      await send({...config, appearance:{...appearance,x,y}});
      const stage = await page.locator('.stage').boundingBox();
      const box = await page.locator('.overlay').boundingBox();
      assert.ok(Math.abs(box.x - (stage.x + (stage.width-box.width)*x/100)) < 1, 'horizontal free placement');
      assert.ok(Math.abs(box.y - (stage.y + (stage.height-box.height)*y/100)) < 1, 'vertical free placement');
      assert.ok(box.x >= stage.x-1 && box.y >= stage.y-1 && box.x+box.width <= stage.x+stage.width+1 && box.y+box.height <= stage.y+stage.height+1, 'overlay remains in frame');
    }
  }
  await page.setViewportSize({width:1280,height:720});
  await send(config);
  for (const button of [1,2,3]) await send({type:'mouse',button,pressed:true,x:0.7,y:0.6});
  assert.equal(await page.locator('svg path').nth(0).getAttribute('fill'), appearance.accent);
  assert.equal(await page.locator('svg path').nth(1).getAttribute('fill'), appearance.right_click);
  assert.equal(await page.locator('svg rect').nth(1).getAttribute('fill'), appearance.middle_click);
  const ringStyle = await page.locator('.ring').evaluate(el => ({x:parseFloat(getComputedStyle(el).left),y:parseFloat(getComputedStyle(el).top),color:getComputedStyle(el).borderTopColor}));
  assert.deepEqual(ringStyle, {x:896,y:432,color:'rgb(17, 170, 51)'}, 'pointer ring independent of keyboard position');
  await send({...config, appearance:{x:-50,y:900,accent:'url(https://invalid.test)'}});
  assert.equal(await page.locator('.overlay').evaluate(el => el.style.left), '0%');
  assert.equal(await page.locator('.overlay').evaluate(el => el.style.top), '100%');
  assert.equal(await page.locator('kbd.held').first().evaluate(el => getComputedStyle(el).outlineColor), 'rgb(168, 121, 255)');
  await send({type:'clear'});
  await send({type:'modifiers', keys:['Alt']});
  await page.evaluate(() => window.overlaySocket.close());
  assert.equal(await page.locator('.keys').count(), 0, 'disconnect clears held state');
  console.log('PASS: held modifiers, releases, clear/disconnect, free placement, landscape/portrait bounds, custom colors, pointer ring and invalid settings.');
} finally {
  await browser?.close();
  server.close();
}
