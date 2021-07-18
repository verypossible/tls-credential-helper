# MacOS binary signing

When a binary is not signed by an Apple Developer ID, depending on how it is transported to a MacOS environment, the `com.apple.quarantine` extended file attribute may be applied to it. This will prevent it from executing and typically pop a system prompt saying as much.

There are two main ways to mitigate this until signing is implemented.

Use a terminal:

```shell
xattr -dr com.apple.quarantine tch
```

Use Apple's Finder application:

1. Navigate to the parent directory via Finder.
2. Right-click the file and select "Open", the prompt that opens will allow you to trust the file which removes the extended file attribute.
