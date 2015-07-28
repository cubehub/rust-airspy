#rust-airspy
This library is a Rust wrapper around [AirSpy SDR library](https://github.com/airspy/host/tree/master/libairspy).

## Dependencies

Build [libairspy](https://github.com/airspy/host/tree/master/libairspy)

    cd ~/Downloads
    git clone https://github.com/airspy/host.git airspy
    cd airspy/libairspy
    mkdir build
    cd build
    cmake ../
    make
    sudo make install

## Usage
Put this in your `Cargo.toml`:

```toml
[dependencies.airspy]
git = "https://git@github.com:cubehub/rust-airspy.git"
```

And this in your crate root:

```rust
extern crate airspy;
```

## Examples

Connect AirSpy SDR to your computer and run the [example](https://github.com/cubehub/rust-airspy/blob/master/examples/airspyrx.rs):

    cargo build --example airspyrx
    ./target/release/examples/airspyrx > /dev/null

This example program can be used to listen FM radio. Make sure [demod](https://github.com/cubehub/demod) and [sox](https://github.com/uklauer/sox) are installed.
NB: On Mac OS X use `play` and on Linux use `aplay`. This pipeline plays FM radio at 103.4 MHz:

    ./target/release/examples/airspyrx 103.4 | demod --samplerate 2500000 --resamplerate 230400 --intype i16 --outtype i16 --bandwidth 100000 fm --deviation 75000 | play -t raw -r 230.4k -e signed-integer -b16 -c 1 -V1 -


## For rust-airspy developers

Tool called rust-bindgen is used to generate Rust functions from C files.

Mac OS X:

    echo export DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/:$DYLD_LIBRARY_PATH >> ~/.profile

Build [rust-bindgen](https://github.com/crabtw/rust-bindgen)

    git clone https://github.com/crabtw/rust-bindgen.git
    cd rust-bindgen
    cargo build

Generate Rust bindings

    ./target/debug/bindgen -l airspy -match airspy.h -match airspy_commands.h -o ~/Development/rust-airspy/src/ffiairspy.rs ~/Downloads/airspy/libairspy/src/airspy.h

Notice that there are still things like `uint32_t`, `uint8_t` etc in just generated ffiairspy.rs file. Replace them manually using your text editor:

    uint64_t -> u64
    uint32_t -> u32
    uint16_t -> u16
    uint8_t  -> u8
