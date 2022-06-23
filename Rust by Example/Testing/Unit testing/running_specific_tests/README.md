```console
$ cargo test test_any_panic
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests (target\debug\deps\running_specific_tests-b35a2853673b088d.exe)

running 1 test
test tests::test_any_panic - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.02s
$ cargo test panic    
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests (target\debug\deps\running_specific_tests-b35a2853673b088d.exe)

running 2 tests
test tests::test_any_panic - should panic ... ok
test tests::test_specific_panic - should panic ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```