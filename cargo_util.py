#!/usr/bin/env python

import argparse
import fileinput
import re
import os
import glob
from shutil import copy, rmtree
from subprocess import check_call

def split(s):
	ret = s.split('\n')
	return filter(lambda v: v, ret)

crate_list_sys = split("""
allegro_util
allegro-sys
allegro_image-sys
allegro_audio-sys
allegro_acodec-sys
allegro_dialog-sys
allegro_primitives-sys
allegro_font-sys
allegro_ttf-sys
allegro_color-sys""")

crate_list_wrapper = split("""
allegro
allegro_image
allegro_audio
allegro_acodec
allegro_dialog
allegro_primitives
allegro_font
allegro_ttf
allegro_color
examples""")

crate_list = crate_list_sys + crate_list_wrapper

parser = argparse.ArgumentParser(description='Perform an operation on all crates.')
parser.add_argument('--version', metavar='VERSION', default='', help='set the version to VERSION')
parser.add_argument('--publish', action='store_true', help='publish the crates')
parser.add_argument('--build', action='store_true', help='build the crates')
parser.add_argument('--test', action='store_true', help='test the crates')
parser.add_argument('--clean', action='store_true', help='clean the crates')
parser.add_argument('--doc', action='store_true', help='build the documentation')
parser.add_argument('--format', action='store_true', help='format all the non-sys crates')
parser.add_argument('--verbose', action='store_true', help='pass --verbose to cargo')

args = parser.parse_args()

def cargo_cmd(command):
	return ['cargo', command] + (['--verbose'] if args.verbose else [])

if len(args.version) > 0:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)

	for crate in crates_and_doc:
		cargo_toml = crate + '/Cargo.toml'
		print 'Processing', cargo_toml

		for line in fileinput.input(cargo_toml, inplace=1):
			line = re.sub('version = "(=?).*" #auto', 'version = "\g<1>' + args.version + '" #auto', line)
			print line,

if args.publish:
	for crate in crate_list:
		print 'Publishing', crate
		check_call(cargo_cmd('publish'), cwd=crate)

if args.build:
	check_call(cargo_cmd('build'), cwd='doc')
	check_call(cargo_cmd('build'), cwd='examples')

if args.format:
	for crate in crate_list_wrapper:
		check_call(cargo_cmd('fmt'), cwd=crate)

if args.test:
	crates_no_examples = filter(lambda crate: crate != 'examples', crate_list)
	for crate in crates_no_examples:
		check_call(cargo_cmd('test') + ['-p', crate], cwd='doc')

if args.clean:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)
	for crate in crates_and_doc:
		print 'Cleaning', crate
		lock = crate + '/Cargo.lock'
		if os.path.exists(lock):
			os.remove(lock)
		check_call(cargo_cmd('clean'), cwd=crate)

if args.doc:
	rmtree('doc/target/doc', ignore_errors=True)
	print 'Building docs'
	check_call(['cargo', 'doc'], cwd='doc')
	print 'Fixing up the search index'
	found = False
	for line in fileinput.input('doc/target/doc/search-index.js', inplace=1):
		new_line = re.sub(r'searchIndex\["delete_me"\].*', '', line)
		if new_line != line:
			found = True
		print new_line,
	if not found:
		raise Exception("Couldn't find the line in search-index.js!")
	print 'Copying new CSS'
	copy('doc/rustdoc.css', 'doc/target/doc/rustdoc.css')
	copy('doc/light.css', 'doc/target/doc/light.css')
	copy('doc/dark.css', 'doc/target/doc/dark.css')

