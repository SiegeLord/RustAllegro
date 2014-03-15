#RustAllegro

[![Build Status](https://travis-ci.org/SiegeLord/RustAllegro.png)](https://travis-ci.org/SiegeLord/RustAllegro)

A very much WIP binding of [Allegro 5](http://liballeg.org/) to the [Rust](http://www.rust-lang.org/) programming language.

Currently it is targeting Allegro version 5.0.10.1.

For (a skeleton) documentation, See:

* [core](http://www.rust-ci.org/SiegeLord/RustAllegro/doc/allegro5/)
* [image](http://www.rust-ci.org/SiegeLord/RustAllegro/doc/allegro_image/)
* [font](http://www.rust-ci.org/SiegeLord/RustAllegro/doc/allegro_font/)

## Building

You will need CMake 2.8+ to build this library.

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix>
make -j
make install
~~~
