На Linux или macOS:

```console
$ RUST_BACKTRACE=1 cargo run
```

```console
$ export RUST_BACKTRACE=1
$ cargo run
```

В Windows:

```powershell
> $env:RUST_BACKTRACE=1
> $env:RUST_BACKTRACE=1; cargo run
```

```cmd
> set RUST_BACKTRACE=1
> cargo run
```