# Puissance4Rust

Build in release mode for a stronger and faster default AI.
```
cargo build [--release] --workspace
```

2 binaries
-   connect4_client
-   connect4_server

When playing "online", start the server first. The client closes at the end of
the game, but the server keeps functioning.

The "online" default server ip address is the same as the one used when playing
locally so playing against the AI without first stopping the server can result
in an error.

The code is not resilient to dumb inputs, the only incorrect action handled
properly is when someone tries to play in a column that is already full.

For more informations, see connect4_client --help and connect4_server --help.
