#!/usr/bin/env python

import argparse
import fileinput
import re
from subprocess import Popen

crate_list="""
allegro
allegro_image
allegro_audio
allegro_acodec
allegro_dialog
allegro_primitives
allegro_font
allegro_ttf
examples
"""

parser = argparse.ArgumentParser(description='Update crate versions and publish.')
parser.add_argument('--version', metavar='VERSION', default='0.0.0', help='set the version to VERSION')
parser.add_argument('--publish', action='store_true', help='publish the crates')

args = parser.parse_args()

for crate in crate_list.split('\n'):
	if len(crate) == 0:
		continue

	cargo_toml = crate + '/Cargo.toml'
	print 'Processing', cargo_toml

	for line in fileinput.input(cargo_toml, inplace=1):
		line = re.sub('version = "(=?).*" #auto', 'version = "\g<1>' + args.version + '" #auto', line)
		print line,

	if args.publish:
		print 'Publishing', crate
		Popen(['cargo', 'publish'], cwd=crate).communicate()
