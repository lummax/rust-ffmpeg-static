rust-ffmpeg-static
==================

This is a showcase for a [rust](https://www.rust-lang.org/) project based on
the [`rust-ffmpeg`](https://github.com/meh/rust-ffmpeg) with statically
compiled [FFmpeg](http://ffmpeg.org/) (and dependencies).

*Note*: This currently only works on 64 bit linux maschines.

Usage
-----

Please initialize the git submodules and run the shell build script. After
that you can build the project using cargo and run the main program without
having `ffmpeg` or its dependencies installed.

    git submodule update --init
    sh build.sh
    cargo build
    ./target/debug/rust-ffmpeg-static mp3 libmp3lame h264 ogg

The output to that last command is (on my maschine) the following. Note that
the dependencies for `mp3` and `h264` were explicitly compiled and enabled
using `ffmpeg` configure flags in the `build.sh` script while `ogg` is not
supported.

    type: decoder
        id: MP3
        name: mp3
        description: MP3 (MPEG audio layer 3)
        medium: Audio
        [.. snip ..]

    type: encoder
        id: MP3
        name: libmp3lame
        description: libmp3lame MP3 (MPEG audio layer 3)
        medium: Audio
        [.. snip ..]
    type: decoder
        id: H264
        name: h264
        description: H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10
        medium: Video
        [.. snip ..]

How it works
------------

The `ffmpeg` source code together with the needed dependencies resides in the
directory `native/`. Whenever there is a public git repository the code was
added via a git submodule. There are unfortunately some dependencies that are
only available via release source code archives (`libmp3lame` for example).

The `build.sh` file sets up an environment and builds every project in the
`native/` directory. Please note that these projects are compiled statically.
It is also necessary to enable [position indipendent
code](https://en.wikipedia.org/wiki/Position-independent_code) and to disable
assember extensions to sucessfully build them statically. The resulting
libraries will be placed in `target/native/lib`.

We now can override the native link search path for the implicit
[`rust-ffmpeg-sys`](https://github.com/meh/rust-ffmpeg-sys) dependency to
ensure, that our just build libraries are used. See `.cargo/config` for this.

Inconveniently we need to explicitly add every static library to the file
`build.rs` to tell the `rustc` compiler to also link these into the final
binary.
