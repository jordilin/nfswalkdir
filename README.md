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

## Performance

This makes use of NFS `READDIRPLUS` operations to retrieve the files and its
metadata. It should be faster than using a sequential directory traversal using
an NFS mount which can be potentially slow as it can involve `stat` sys calls and
`GETTATTR, READDIRPLUS` RPC calls depending on the client and mount.

 `nfswalkdir` relies on libnfs to do the RPC calls, so the application just
fans-out directory scanning to multiple threads if found. Number of threads
defaults to 5 and it adjustable using the `--numworkers` flag.

Local test with a mount with approximately 80K files and few subdirs is approx
55x faster than using Python3 `os.walk`. Your mileage may vary.

## License

This project is licensed under

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
