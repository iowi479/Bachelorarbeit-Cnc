# Bachelorarbeit-CNC

## Dependencies
These are determined from a clean ubuntu 22.03 installation.

- [rust](https://www.rust-lang.org/tools/install)
- [libyang2](https://github.com/CESNET/libyang)
- [libnetconf2](https://github.com/CESNET/libnetconf2)
- required packages on top of the dependencies of the above:
```console
apt-get install pkg-config
```

## Testing

To run the provided or selfwritten tests, you can't just call cargo's test command.

Because the Tests are relliant on specific contents of the filestorage, you have to run the tests single threaded and therefore sequentially.
Otherwise the content of the Storage component is altered by all tests simultainiously and the tests will sometimes fail.

Showing the output and all prints for some manual checks of the date used in the tests.

```console
cargo test -- --test-threads=1 --show-output --nocapture
```

## Usage

In the release-build, the main-entrypoint in ./src/main.rs is used.
Therefore the specified modules in that main function will be loaded and used.

```console
cargo build --release
```

```console
./target/release/ba
```

## Extensibility

In order to implement a new Component like a SNMP-Southbound Interface, you can just simply copy a existing Implementation of a Southbound-Component.
All the Functions described by the SouthboundAdapter-Trait have to be implemented as described in their docs.

After the Implementation is finished, you should be able to replace the used component in the ./src/main.rs main-function and use your implementation instead.
By compiling and starting the CNC, your component will be loaded and used as the southbound-Interface.
