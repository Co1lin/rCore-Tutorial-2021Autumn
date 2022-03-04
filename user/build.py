import os
import argparse

parser = argparse.ArgumentParser()
parser.add_argument('--release', action='store_true')
parser.add_argument('--debug', action='store_true')
args = parser.parse_args()

base_address = 0x80400000
step = 0x20000
linker = 'src/linker.ld'

app_id = 0
apps = os.listdir('build/app')
apps.sort()
for app in apps:
    app = app[:app.find('.')]
    lines = []
    lines_before = []
    with open(linker, 'r') as f:
        for line in f.readlines():
            lines_before.append(line)
            line = line.replace(hex(base_address), hex(base_address+step*app_id))
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)
    build_args = ""
    if args.release:
        build_args += "--release"
    os.system(f'cargo build --bin {app} {build_args}')
    print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
    with open(linker, 'w+') as f:
        f.writelines(lines_before)
    app_id = app_id + 1
