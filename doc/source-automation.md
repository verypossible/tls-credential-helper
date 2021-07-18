# Source automation

## SSH into GitHub Actions

If a job fails or you manually start it while providing the `debug_enabled=true` parameter the "Setup tmate session" step will run printing an ssh command every 5 seconds. The step will timeout after five minutes, but this is configurable within the config as `timeout-minutes`.

## Triggers

This repository takes action automatically when any branch or tag is pushed to GitHub via GitHub Actions.

### Pushing a branch

Tests the branch and builds and publishes binaries associated with it.

### Pushing a tag

TODO: Tests the branch and builds and publishes binaries associated with it. The binaries are available as GitHub Release artifacts associated with the pushed tag.

**WARNING** Binaries built for MacOS via source automation are not currently signed with an Apple Developer ID. See [MacOS binary signing](macos-binary-signing.md).
