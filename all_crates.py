#!/usr/bin/env python

import argparse
import fileinput
import re
from subprocess import Popen

crate_list="""
allegro-sys
allegro_image-sys
allegro_audio-sys
allegro_acodec-sys
allegro_dialog-sys
allegro_primitives-sys
allegro_font-sys
allegro_ttf-sys
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

parser = argparse.ArgumentParser(description='Perform an operation on all crates.')
parser.add_argument('--version', metavar='VERSION', default='', help='set the version to VERSION')
parser.add_argument('--publish', action='store_true', help='publish the crates')
parser.add_argument('--build', action='store_true', help='build the crates')
parser.add_argument('--clean', action='store_true', help='clean the crates')

args = parser.parse_args()

crate_list = crate_list.split('\n')
crate_list = filter(lambda crate: len(crate) > 0, crate_list)

if len(args.version) > 0:
	for crate in crate_list:
		cargo_toml = crate + '/Cargo.toml'
		print 'Processing', cargo_toml

		for line in fileinput.input(cargo_toml, inplace=1):
			line = re.sub('version = "(=?).*" #auto', 'version = "\g<1>' + args.version + '" #auto', line)
			print line,

if args.publish:
	for crate in crate_list:
		print 'Publishing', crate
		Popen(['cargo', 'publish'], cwd=crate).communicate()

if args.build:
	for crate in crate_list:
		print 'Building', crate
		Popen(['cargo', 'build'], cwd=crate).communicate()

if args.clean:
	for crate in crate_list:
		print 'Cleaning', crate
		Popen(['cargo', 'clean'], cwd=crate).communicate()
