На Linux или macOS:

```console
$ CASE_INSENSITIVE=1 cargo run to poem.txt
```

```console
$ export CASE_INSENSITIVE=1
$ cargo run to poem.txt
```

В Windows:

```powershell
> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
```

```powershell
> $Env:CASE_INSENSITIVE=1
> cargo run to poem.txt
```

```cmd
> set CASE_INSENSITIVE=1
> cargo run to poem.txt
```