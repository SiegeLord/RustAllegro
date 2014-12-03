#RustAllegro

[![Build Status](https://travis-ci.org/SiegeLord/RustAllegro.png)](https://travis-ci.org/SiegeLord/RustAllegro)

A very much WIP binding of [Allegro 5](http://liballeg.org/) to the [Rust](http://www.rust-lang.org/) programming language.

Currently it is targeting Allegro version 5.0.10.1.

## Documentation

See [here](http://siegelord.github.io/RustAllegro/doc/allegro/index.html). Note that it is very incomplete.

## Building

### Via Cargo:

The included packages are:

Main crates:

```
[dependencies.allegro]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_acodec]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_audio]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_dialog]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_font]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_image]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_primitives]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

```
[dependencies.allegro_ttf]

git = "https://github.com/SiegeLord/RustAllegro.git"
```

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
