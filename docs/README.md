# First Project Rust

This is a simple Rust project that generates a random number between 1 and 100 and prompts the user to guess the number.

## Project Structure

```
my_project_rust/
├── .gitignore
├── [Cargo.lock](../Cargo.lock)
├── [Cargo.toml](../Cargo.toml)
├── docs/
│   └── [README.md](README.md)
├── src/
│   └── [main.rs](../src/main.rs)
└── target/
    ├── [target/.rustc_info.json](../target/.rustc_info.json)
    ├── [target/CACHEDIR.TAG](../target/CACHEDIR.TAG)
    ├── debug/
    │   ├── .cargo-lock
    │   ├── .fingerprint/
    │   │   └── ...
    │   ├── build/
    │   │   └── ...
    │   ├── deps/
    │   ├── examples/
    │   ├── incremental/
    │   ├── my_project_rust
    │   └── [my_project_rust.d](../target/debug/my_project_rust.d)
```

## Dependencies

This project uses the following dependencies:

- `rand` crate for generating random numbers.

## Usage

To run the project, use the following command:

```sh
cargo run
```

## How It Works

The program generates a random number between 1 and 100 and repeatedly prompts the user to guess the number. It provides feedback on whether the guess is too high or too low until the correct number is guessed.

## Example

```
Devine un nombre entre 1 et 100 !
Entre ta proposition :
50
Trop petit !
Entre ta proposition :
75
Trop grand !
Entre ta proposition :
60
Bravo, tu as trouvé !
```

## License

This project is licensed under the MIT License.
