## Nomad Config file

This is a crate for working with nomad configuration files. These config files
contain information about the state of Nomad deployments.

It also includes an auto-generated WASM library

### Development note

To work around some `wasm-bindgen` limitations, we currently (unfortunately)
have to manually define TS types for the rust structs. These are found in the
`data` directory. When a rust struct is updated or added, the corresponding
definitions should be added in `data/definitions.ts` and `data/types.rs`. At
compile-time these files are combind to `src/wasm/types.rs`.

In the future it'd be cool to auto-generate this code :)

### Building

- `$ cargo build`

To build the wasm library:

- [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- `$ wasm-pack build --target nodejs --scope nomad-xyz --release`

`wasm-pack` docs are found [here](https://rustwasm.github.io/wasm-pack/book/).
