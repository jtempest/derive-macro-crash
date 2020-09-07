Toolchain (updated 07/09/2020): 
- stable-x86_64-pc-windows-msvc (default)
- rustc 1.46.0 (04488afe3 2020-08-24)

Command: 
- `cargo test`

Output:

```
> cargo test
   Compiling derive-macro-crash v0.1.0 (<path>)
thread 'rustc' panicked at '`"0"` is not a valid identifier', src\librustc_expand\proc_macro_server.rs:335:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0 (04488afe3 2020-08-24) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

error: proc-macro derive panicked
 --> tests\derive-macro-crash.rs:3:10
  |
3 | #[derive(DeriveMacroCrash)]
  |          ^^^^^^^^^^^^^^^^
  |
  = help: message: `"0"` is not a valid identifier

error: aborting due to previous error

error: could not compile `derive-macro-crash`.

To learn more, run the command again with --verbose.
```