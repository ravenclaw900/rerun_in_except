# rerun_in_except

Lets you rerun a build script if files in a directory have changed, excluding specific ones that are listed.

Example:
```rust
println!(rerun_in_except("frontend", &["frontend/node_modules", "frontend/artifacts"]))
```
