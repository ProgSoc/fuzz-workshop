
# ProgSoc Fuzzing Workshop Demo

This repo is a basic json parser that I wrote the other day, with a few bugs intentionally included.

See if you can spot them before they are found by the fuzzer!

## Setup

First, make sure you have cargo-fuzz installed:
```
cargo install --locked cargo-fuzz
```

Keep in mind that rust-fuzz requires nightly rust, so make sure you have that installed **and up to date**.

## Looking at the project

This project contains a `lib.rs` file which has all the json parsing logic, along with a `main.rs` if you want to quickly run it and `test.rs` to run tests on "every" edge case that I could think of. Surely that's enough for right?

Feel free to play around with it and see if you can break it yourself.

## Setting up fuzzing

Run `cargo fuzz init` to set up the fuzzing in this folder. This creates a folder called `fuzz`.

Under `fuzz/fuzz_targets`, rename the target to whatever you like, make sure you update `fuzz/Cargo.toml` accordingly.

Inside the fuzzing macro, set up the function to fuzz as you prefer, for example:
```rs
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = json::parse_json(s);
    }
});
```

It will throw random inputs at it until it panicks or crashes the process.

Now, just run `cargo fuzz <your target name>`, and watch it automatically find bugs!

## Results!

For spoilers on the results, check out [BUGS.md](BUGS.md). Though I recommend trying to find them yourself first.