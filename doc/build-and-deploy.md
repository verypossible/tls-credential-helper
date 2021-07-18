# Build

The following documents how to manually produce production-ready binaries for all supported targets.

## MacOS

When leveraging this guide to manually build for MacOS, the produced binary will not be signed by an Apple Developer ID. See [MacOS binary signing](macos-binary-signing.md) to understand the implications of this.

### Native compilation

```shell
cargo build \
  --release \
  --target x86_64-apple-darwin
```

### Cross compilation

TODO: This is time consuming due to lack of support in Cross for OpenSSL.

# Deploy

Per [Source automation](source-automation.md), push a tag.
