```console
$ cargo run -- Rust
This is not the answer.
$ cargo run -- 42
This is the answer!
$ cargo run -- do something
error: second argument not an integer
usage:
match_args <string>
    Check whether given string is the answer.
match_args {increase|decrease} <integer>
    Increase or decrease given integer by one.
$ cargo run -- do 42
error: invalid command
usage:
match_args <string>
    Check whether given string is the answer.
match_args {increase|decrease} <integer>
    Increase or decrease given integer by one.
$ cargo run -- increase 42
43
```