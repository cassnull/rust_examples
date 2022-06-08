На Linux или macOS:

```console
$ echo -e "127.0.0.1\n192.168.0.1\n" > hosts
$ cargo run
127.0.0.1
192.168.0.1
```

В Windows:

```cmd
> echo "127.0.0.1" "192.168.0.1" > hosts
> cargo run
127.0.0.1
192.168.0.1
```