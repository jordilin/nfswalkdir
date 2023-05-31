# NFS walkdir

NFS parallel directory traversal using MPMC channels.

## Install

libnfs is required to compile the binary as it makes use of the libnfs Rust
bindings <https://github.com/cholcombe973/libnfs>. You might need to install
libnfs, libnfs-devel and the llvm tools for linking.

If you have nixpkgs + direnv, then you are all set.

Compilation:

```bash
cargo build --release
```

## Usage

```bash
./target/release/nfswalkdir --help
```

## License

This project is licensed under

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
