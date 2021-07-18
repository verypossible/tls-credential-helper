# Usage

This document assumes you are interacting with a compiled TCH binary. If you wish to leverage
Cargo to exercise functionality see `contributing.md`.

**WARNING** If you are having trouble executing a TCH MacOS binary, see [MacOS binary signing](macos-binary-signing.md).

## create-ca-certificate

Create a root certificate authority.

```shell
tch create-ca-certificate \
  --self-signed \
  --days-valid 10950 \
  --common-name root
```

Create an intermediate certificate authority.

```shell
tch create-ca-certificate \
  --signer-private-key-path root-ca-private-key.pem \
  --signer-certificate-path root-ca-certificate.pem \
  --days-valid 10950 \
  --common-name intermediate
```

## create-certificate

Create a certificate.

```shell
tch create-certificate \
  --signer-private-key-path intermediate-ca-private-key.pem \
  --signer-certificate-path intermediate-ca-certificate.pem \
  --days-valid 10950 \
  --common-name end-entity-1
```

Create a self-signed certificate.

```shell
tch create-certificate \
  --self-signed
  --days-valid 10950 \
  --common-name end-entity-2
```
