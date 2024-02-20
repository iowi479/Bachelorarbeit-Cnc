# Bachelorarbeit-CNC

libraries like libyang2, libnetconf2 and openssl need to be installed. Should be told by the program anyways.


## run tests
single threaded is important for no contests on netconf connections and filereads.
Otherwise tests might fail.
```console
cargo test -- --test-threads=1 --show-output --nocapture
```

## run
```console
cargo build --release
```

```console
./target/release/ba
```
