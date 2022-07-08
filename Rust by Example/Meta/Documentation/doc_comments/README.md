```console
$ cd src
$ rustc lib.rs --crate-type lib
$ rustdoc --test --extern doc_comments="libdoc_comments.rlib" lib.rs
```