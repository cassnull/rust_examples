На Linux или macOS:

```console
$ RUST_LOG="warn,enable_log_levels_per_module::foo=info,enable_log_levels_per_module::foo::bar=debug" cargo run
```

В Windows:

```powershell
> $Env:RUST_LOG='warn,enable_log_levels_per_module::foo=info,enable_log_levels_per_module::foo::bar=debug'; cargo run
```

```cmd
> set RUST_LOG=warn,enable_log_levels_per_module::foo=info,enable_log_levels_per_module::foo::bar=debug
> cargo run
```
