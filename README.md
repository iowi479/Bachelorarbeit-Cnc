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

### Building

In the release-build, the main-entrypoint in ./src/main.rs is used.
Therefore the specified modules in that main function will be loaded and used.

```console
cargo build --release
```

```console
./target/release/ba
```

### Changing Topology

In the Topology-Component there are function to load to different Topologies.
These topologies are set by adding the nodes and the corresponding paths between them.
The datastructures include the general information about the node (ID, IP-Adress, Ports, ...) and the configuration interface for the netconf connection.

For a different Topology, on of those examples can be used as a startingpoing and be modified.
Generally needed to be added is:
- the Node itself including all the information specified in the Node-Datastructure
- Ports and their information
- Paths between nodes

To simply change the needed Netconf-parameters, the config_params datastructure for the node has to be modified.
This datastructure includes the ip, port and ssh-params for the netconf-over-ssh communication.


### Changing Northbound-Functionality

Currently the Nortbound-Interface executes the run() function.
In this function you can specify all the actions to take.
Since there is no protcoll implemented yet, these actions have to be specified in code.
Sample Actions are provided in the different implementations of the Nortbound-Interface-Component.

To test actions you can build components that manage streams with adding, modifing or deleting to generate the configuration needed.
After a Timeout another action like computing the shedules can be executed.
That way a simulated cuc and their action can be implemented.


## Extensibility

In order to implement a new Component like a SNMP-Southbound Interface, you can just simply copy a existing Implementation of a Southbound-Component.
All the Functions described by the SouthboundAdapter-Trait have to be implemented as described in their docs.

After the Implementation is finished, you should be able to replace the used component in the ./src/main.rs main-function and use your implementation instead.
By compiling and starting the CNC, your component will be loaded and used as the southbound-Interface.
