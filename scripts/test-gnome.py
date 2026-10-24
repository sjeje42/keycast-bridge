#!/usr/bin/env python3
"""Real session-bus ownership, modal input, snapshot and teardown acceptance tests."""
import json
import subprocess
import time
from gi.repository import Gio, GLib

address = Gio.dbus_address_get_for_bus_sync(Gio.BusType.SESSION, None)
def connection():
    return Gio.DBusConnection.new_for_address_sync(address, Gio.DBusConnectionFlags.AUTHENTICATION_CLIENT | Gio.DBusConnectionFlags.MESSAGE_BUS_CONNECTION, None, None)
bus = connection()
settings = dict(ring=True, trail=True, trail_style='solid', shake=True, spotlight=True, spotlight_shape='circle', spotlight_radius=150, dimming=65, pen_color='#ff4444', pen_width=4, zoom=2)
packet = dict(request=dict(enabled=True, mode='normal', serial=1), appearance=dict(presentation=settings, ring=dict(color='#a879ff', radius=30, thickness=3, neon=True, pulse=True)))
def call(method, parameters=None, client=None):
    return (client or bus).call_sync('org.gnome.Shell', '/fr/jeromelab/KeycastBridge/Presentation', 'fr.jeromelab.KeycastBridge.Presentation', method, parameters, None, Gio.DBusCallFlags.NO_AUTO_START, 1000, None).unpack()[0]
def update():
    return call('Update', GLib.Variant('(s)', (json.dumps(packet),)))
def state(): return json.loads(call('State'))
def wait(predicate, seconds=6):
    deadline=time.monotonic()+seconds
    while time.monotonic()<deadline:
        try:
            if predicate(): return
        except GLib.Error: pass
        time.sleep(0.1)
    raise AssertionError('Timed out waiting for GNOME presentation state')
def keys(*args): subprocess.run(['xdotool',*args],check=True)
wait(lambda: call('State') is not None, 40)
# GNOME may open Overview on a fresh session; Escape returns to the desktop.
keys('key','Escape')
time.sleep(0.5)
keys('mousemove','450','350')
assert update()=='normal'
assert state()['enabled'] and not state()['grabbed']
for style in ['solid','dash','dot','glow','particle','star']:
    settings['trail_style']=style;update();keys('mousemove','500','370');time.sleep(0.08);keys('mousemove','450','350')
for mode in ['annotate','whiteboard']:
    packet['request'].update(mode=mode,serial=packet['request']['serial']+1)
    assert update()==mode
    assert state()['grabbed']
    keys('mousedown','1','mousemove','600','400','mouseup','1');time.sleep(0.15)
    assert state()['strokes']==1,state()
    keys('key','ctrl+z');time.sleep(0.1);assert state()['strokes']==0
    keys('mousedown','1','mousemove','700','440','mouseup','1');time.sleep(0.1)
    keys('key','BackSpace');time.sleep(0.1);assert state()['strokes']==0
    keys('key','Escape');time.sleep(0.15);assert state()['mode']=='normal' and not state()['grabbed']
    assert update()=='normal' # Heartbeat must not reopen the mode after Escape.
packet['request'].update(mode='zoom',serial=packet['request']['serial']+1)
update()
wait(lambda: (update(),state()['snapshot'])[1])
assert state()['grabbed']
keys('key','Escape');time.sleep(0.15);assert not state()['snapshot'] and not state()['grabbed']
# A second bus client cannot take over an active lease.
other=connection()
try:
    call('Update',GLib.Variant('(s)',(json.dumps(packet),)),other)
    raise AssertionError('Second client unexpectedly took ownership')
except GLib.Error: pass
other.close_sync(None)
# Lost heartbeat clears all UI and never reactivates on the old command.
time.sleep(2.2)
assert not state()['enabled'] and not state()['snapshot'] and not state()['grabbed']
assert update()=='stopped'
packet['request']['serial']+=1;packet['request']['mode']='whiteboard';assert update()=='whiteboard'
bus.close_sync(None)
bus=connection()
wait(lambda: not state()['enabled'])
print('GNOME 48: six trails, modal drawing/undo/clear/Escape, frozen zoom, owner lease and crash cleanup passed.')
