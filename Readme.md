# Noble Protocol Rust Libraries

This repository contains Rust libraries generated from the protobuf definitions of various Noble Chain modules. These libraries enable Rust applications to interact with the Noble Chain ecosystem.

## Included Modules

- **noble-aura-proto**: Protobuf definitions for the Noble Aura module
- **noble-dollar-proto**: Protobuf definitions for the Noble Dollar module
- **noble-fiattokenfactory-proto**: Protobuf definitions for the Fiat Token Factory module
- **noble-forwarding-proto**: Protobuf definitions for the Forwarding module
- **noble-halo-proto**: Protobuf definitions for the Noble Halo module
- **noble-cctp-proto**: Protobuf definitions for the Cross-Chain Transfer Protocol module

## Usage

Add the desired proto crate to your `Cargo.toml`:

```toml
[dependencies]
noble-cctp-proto = "0.1.0"
```

Then import and use the generated types in your Rust code:

```rust
use noble_cctp_proto::prost::noble::cctp::v1::SendMessageRequest;

// Use the generated protobuf types
let message = SendMessageRequest {
    // ...
};
```

## Building

The proto definitions are built using a custom build script that:

1. Updates git submodules to fetch the latest protocol definitions
2. Compiles the `.proto` files to Rust using `prost-build` and `tonic-build`
3. Applies necessary patches to the generated code
4. Formats the output with `rustfmt`

To rebuild the proto files:

```bash
cd proto-build
cargo run
```

## Dependencies

This project depends on:
- `cosmos-sdk-proto`: For Cosmos SDK related protobuf definitions
- `prost`: For Protocol Buffer support
- `tonic`: For gRPC services
- `tendermint-proto`: For Tendermint related protobuf definitions

## License

Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request
