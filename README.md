## Generating [Cloudflare workerd](https://github.com/cloudflare/workerd) configs with [capnproto-rust](https://github.com/capnproto/capnproto-rust)

This is a brief example to demonstrate how you can generate configuration files for [Cloudflare workerd](https://github.com/cloudflare/workerd)
using the serialisation feature of [capnproto-rust](https://github.com/capnproto/capnproto-rust).

## Requirements

- Cap'n Proto built from source, as there are some changes required that are not on the latest release.
    - Take a look at the 'From Git' heading: https://capnproto.org/install.html
- You will need the Rust plugin for Cap'n Proto's code generation.
  - `cargo install capnpc-rust`
- The schema for workerd's config files, available at https://github.com/cloudflare/workerd/blob/main/src/workerd/server/workerd.capnp
  - The current version, at the time of writing, is included at [schema/workerd.capnp](schema/workerd.capnp).

## Updating the schema

I've already included the resulting Rust code in this repository at [src/workerd.rs](src/workerd.rs) - but it can be updated with:

```shell
capnp compile -orust:src/ --src-prefix=schema/ ./schema/workerd.capnp
```

This will generate a file called `workerd_capnp.rs` in the `src` folder - you can rename this if preferred.

## Usage

It will output a binary config to stdout, which you can redirect to a file and then use with `workerd` like so:

```shell
cargo run > config.bin
workerd serve -b config.bin
```

- The `-b` flag indicates it is a binary config rather than text.