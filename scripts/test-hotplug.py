"""Linux CI only: synthetic uinput devices; never run during a real recording."""
import fcntl
import json
import os
from pathlib import Path
import queue
import struct
import subprocess
import threading
import time

uid = int(os.environ['TEST_UID'])
assert os.geteuid() == 0 and uid != 0

class Keyboard:
    def __init__(self, name):
        self.fd = os.open('/dev/uinput', os.O_WRONLY | os.O_NONBLOCK)
        fcntl.ioctl(self.fd, 0x40045564, 1)  # UI_SET_EVBIT EV_KEY
        for key in range(1, 256):
            fcntl.ioctl(self.fd, 0x40045565, key)
        data = name.encode().ljust(80, b'\0') + struct.pack('HHHHI', 3, 0x1234, 0x5678, 1, 0) + bytes(1024)
        os.write(self.fd, data)
        fcntl.ioctl(self.fd, 0x5501)  # UI_DEV_CREATE
    def key(self, code, value):
        os.write(self.fd, struct.pack('llHHi', 0, 0, 1, code, value))
        os.write(self.fd, struct.pack('llHHi', 0, 0, 0, 0, 0))
    def close(self):
        if self.fd is not None:
            fcntl.ioctl(self.fd, 0x5502)
            os.close(self.fd)
            self.fd = None

process = subprocess.Popen(
    ['target/debug/keycast-bridge-capture', 'all', 'us', 'shortcuts', 'no-mouse'],
    env={**os.environ, 'PKEXEC_UID': str(uid)}, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
    stderr=subprocess.PIPE, text=True, bufsize=1)
events = queue.Queue()
threading.Thread(target=lambda: [events.put(json.loads(line)) for line in process.stdout], daemon=True).start()
stop = threading.Event()
def heartbeat():
    while not stop.is_set():
        try:
            process.stdin.write('ping\n'); process.stdin.flush()
        except (BrokenPipeError, ValueError):
            break
        stop.wait(.25)
pinger = threading.Thread(target=heartbeat, daemon=True)
pinger.start()
def until(predicate, timeout=8):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            event = events.get(timeout=.2)
        except queue.Empty:
            if process.poll() is not None:
                raise AssertionError('Helper exited: ' + process.stderr.read())
            continue
        if predicate(event): return event
    raise AssertionError('Missing expected capture event')
def connected(name):
    until(lambda e: e['type']=='device_status' and e['message'].startswith('Connecté') and name in e['message'])
def label(value):
    until(lambda e: e.get('label') == value)

keyboards = []
try:
    until(lambda e: e['type']=='ready')
    children = Path(f'/proc/{process.pid}/task/{process.pid}/children').read_text().split()
    assert len(children) == 1
    status = Path(f'/proc/{children[0]}/status').read_text()
    assert next(l for l in status.splitlines() if l.startswith('Uid:')).split()[1:] == [str(uid)] * 4
    a = Keyboard('Keycast CI first'); keyboards.append(a); connected('Keycast CI first')
    a.key(29, 1); a.key(30, 1); label('Ctrl + A')
    # Unplug while Ctrl is held, then attach another keyboard.
    a.close()
    until(lambda e: e['type']=='device_status' and e['message'].startswith('Déconnecté'))
    b = Keyboard('Keycast CI second'); keyboards.append(b); connected('Keycast CI second')
    b.key(30, 1); b.key(30, 0)
    time.sleep(.25)
    pending=[]
    while not events.empty(): pending.append(events.get_nowait())
    assert not any(e['type']=='key' for e in pending), pending
    b.key(29, 1); b.key(46, 1); label('Ctrl + C')
    b.close()
    until(lambda e: e['type']=='device_status' and e['message'].startswith('Déconnecté'))
    stop.set(); pinger.join(); process.stdin.close()
    assert process.wait(timeout=4) == 0, process.stderr.read()
    print('PASS: udev hotplug, descriptor transfer, unprivileged reader, removal with Ctrl held, replacement keyboard and heartbeat EOF.')
finally:
    stop.set(); pinger.join(timeout=1)
    for keyboard in keyboards: keyboard.close()
    if process.poll() is None:
        process.stdin.close()
        try: process.wait(timeout=4)
        except subprocess.TimeoutExpired: process.kill(); process.wait()
