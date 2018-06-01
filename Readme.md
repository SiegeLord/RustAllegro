# RustAllegro

[![Build Status](https://travis-ci.org/SiegeLord/RustAllegro.png)](https://travis-ci.org/SiegeLord/RustAllegro)
[![](http://meritbadge.herokuapp.com/allegro)](https://crates.io/crates/allegro)

A thin [Rust](http://www.rust-lang.org/) wrapper of [Allegro 5](http://liballeg.org/).

## Game loop example

```rust
#[macro_use]
extern crate allegro;
extern crate allegro_font;

use allegro::*;
use allegro_font::*;

allegro_main!
{
    let core = Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();

    let display = Display::new(&core, 800, 600).unwrap();
    let timer = Timer::new(&core, 1.0 / 60.0).unwrap();
    let font = Font::new_builtin(&font_addon).unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();
    'exit: loop
    {
        if redraw && queue.is_empty()
        {
            core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));
            core.draw_text(&font, Color::from_rgb_f(1.0, 1.0, 1.0),
                (display.get_width() / 2) as f32, (display.get_height() / 2) as f32,
                FontAlign::Centre, "Welcome to RustAllegro!");
            core.flip_display();
            redraw = false;
        }

        match queue.wait_for_event()
        {
            DisplayClose{..} => break 'exit,
            TimerTick{..} => redraw = true,
            _ => (),
        }
    }
}
```

## Documentation

See [here](http://siegelord.github.io/RustAllegro/doc/allegro/index.html). Note
that it is very incomplete. You'll likely want to refer back to allegro's
[documentation](http://liballeg.org/api.html) somewhat heavily at this time.

## Packages

The included packages are:

Wrappers:

* [allegro](https://crates.io/crates/allegro)
* [allegro_acodec](https://crates.io/crates/allegro_acodec)
* [allegro_audio](https://crates.io/crates/allegro_audio)
* [allegro_dialog](https://crates.io/crates/allegro_dialog)
* [allegro_font](https://crates.io/crates/allegro_font)
* [allegro_image](https://crates.io/crates/allegro_image)
* [allegro_primitives](https://crates.io/crates/allegro_primitives)
* [allegro_ttf](https://crates.io/crates/allegro_ttf)

Bindings:

* [allegro-sys](https://crates.io/crates/allegro-sys)
* [allegro_acodec-sys](https://crates.io/crates/allegro_acodec-sys)
* [allegro_audio-sys](https://crates.io/crates/allegro_audio-sys)
* [allegro_dialog-sys](https://crates.io/crates/allegro_dialog-sys)
* [allegro_font-sys](https://crates.io/crates/allegro_font-sys)
* [allegro_image-sys](https://crates.io/crates/allegro_image-sys)
* [allegro_primitives-sys](https://crates.io/crates/allegro_primitives-sys)
* [allegro_ttf-sys](https://crates.io/crates/allegro_ttf-sys)

Examples:

* [allegro_examples](https://crates.io/crates/allegro_examples)

## General usage notes

The `allegro-sys` package (and, transitively, the rest of the packages) detects
which version of Allegro to bind by parsing the C header. The build script will
look for it in some common locations, but sometimes you will need to help it by
specifying the `ALLEGRO_INCLUDE_DIR` environment variable when invoking `cargo
build`. This directory should contain the `allegro5` directory with all of the
headers inside it. The build script will define the following two metadata
entries that the crates that depend on it can use to determine which version is
used:

* `sub_version` - The sub version of Allegro (e.g. for 5.1.10 the sub version is 1)

* `wip_version` - The wip version of Allegro (e.g. for 5.1.10 the wip version is 10).

Note that the `Core::init()` will attempt to verify that the binding
corresponds to the version of the library you're linking to.

There are a few features that might come in useful:

* `link_none` - Do not try to link the standard Allegro libraries, in
                case you want to link the monolith library or have other
                needs.
* `link_debug` - Link to the debug versions of the Allegro libraries. Can
                 be combined with `link_static`.
* `link_static` - Link to the static versions of the Allegro libraries.
                  Note that you'll have to link the various dependency
                  libraries yourself. Can be combined with `link_debug`.

Additionally, you can specify a link directory by setting a `ALLEGRO_LINK_DIR`.

## Windows notes

RustAllegro works well with the official pre-compiled binaries. First,
download the official binaries from http://liballeg.org. You'll want to
match the ABI of your Rust installation. GNU ABI on 32 bit can load
Allegro 32 bit MSVC binaries, but otherwise you'll want to match the
platform and ABI exactly. Let's say you extract the binaries to
`C:/allegro`. That directory will contain the include, bin and lib
directories. To compile and run the RustAllegro examples, do the
following from the RustAllegro's `examples` directory:

* If you're using MSYS:

```
export ALLEGRO_INCLUDE_DIR=C:/allegro/include
export ALLEGRO_LINK_DIR=C:/allegro/lib
cargo build
```

* If you're using cmd directly:

```
set ALLEGRO_INCLUDE_DIR=C:/allegro/include
set ALLEGRO_LINK_DIR=C:/allegro/lib
cargo build
```

Now you need to copy the Allegro DLLs next to the generated executables
(which will probably be under target/debug directory). Now you should
be able to run the examples (make sure to run them from RustAllegro's
`examples` directory, so they can find the various data files they
require).
