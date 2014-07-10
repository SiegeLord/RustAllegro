#RustAllegro

[![Build Status](https://travis-ci.org/SiegeLord/RustAllegro.png)](https://travis-ci.org/SiegeLord/RustAllegro)

A very much WIP binding of [Allegro 5](http://liballeg.org/) to the [Rust](http://www.rust-lang.org/) programming language.

Currently it is targeting Allegro version 5.0.10.1.

## Documentation

See [here](http://siegelord.github.io/RustAllegro/doc/allegro5/index.html). Note that it is very incomplete.

## Building

### Via Cargo:

The included packages are:

Main crates:

* allegro
* allegro_acodec
* allegro_audio
* allegro_dialog
* allegro_font
* allegro_image
* allegro_primitives
* allegro_ttf

Examples:

* allegro_examples

### Via CMake 2.8:

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix>
make -j
make install
~~~
