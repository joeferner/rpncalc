## rpncalc

rpncalc is a command line RPN calculator.

## Development Workspace Setup

The main facility used for interacting with this project's lifecycle (build/test/format/lint) is
[cargo-make](https://sagiegurari.github.io/cargo-make). Therefore, this is the only dependency you
need to install on your machine:

```bash
cargo install cargo-make
```

## Commands

With `cargo-make`, all of this project's commands will become available to you:

```bash
cargo make clean            # Clean up temporary files
cargo make build            # Lint and build the project
cargo make run              # Run the application
cargo make test             # Run all unit tests
cargo make test-coverage    # Run all unit tests and write a code coverage report to STDOUT
cargo make test-coverage-ci # Run all unit tests and write a code coverage report to a text file in LCOV format
cargo make format           # Formats (rewrites) every applicable file in the project
cargo make format-ci        # Formats (report only) every applicable file in the project
cargo make lint             # Lints (report only) every applicable file in the project
cargo make lint-watch       # Lints (report only) every applicable file in the project and re-lints whenever files change
```
