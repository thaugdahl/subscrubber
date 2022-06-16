**NOTE:** This was purely written as an effort in learning Rust. It should not
be interpreted as a serious software project. Do not expect the repository to receive any
frequent updates. You are free to make a pull request :)

# Subscrubber

Subscrubber shifts the timestamps of any SRT file by some amount set by the user.

## Usage

### With Cargo
```
cargo run [--release] -- {shift amount} {input file} {output file}
```

### Without Cargo
```
./subscrubber {shift amount} {input file} {output file}
```
