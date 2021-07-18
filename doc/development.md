# Development

## Compile and checks

```shell
# compile
cargo build

# test
cargo test

# check for errors (faster than cargo-build)
cargo check

# check for common mistakes and areas for improvement
cargo clippy
```

For information regarding building manual production-ready binaries, see [Build and deploy](build-and-deploy.md).

## Exercise the CLI while developing

### With Cargo

```shell
# generally
cargo run <flags-opts-args>

# for example
cargo run \
  create-ca-certificate \
    --self-signed \
    --days-valid 10950 \
    --common-name root
```

### Manually

```shell
# generally
./target/<target>/release/tls-credential-helper <flags-opts-args>

# for example
./target/x86_64-apple-darwin/release/tls-credential-helper \
  create-ca-certificate \
    --self-signed \
    --days-valid 10950 \
    --common-name root
```

## Watcher (run commands on save)

```shell
cargo watch \
  -x test \
  -x check \
  -x clippy \
  -x 'run create-ca-certificate --days-valid 1 --common-name cat'
```
