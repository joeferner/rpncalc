use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::tui::nroff::{nroff_format, nroff_to_markdown};
use crate::tui::{create_help_string, run_tui};
use color_eyre::eyre;
use std::{env, process};

mod error;
mod functions;
mod number;
mod rpn_calc;
mod stack;
mod stack_item;
mod tui;
mod units;

struct Args {
    // set to force interactive mode even if stack items are presented
    interactive: bool,

    stack: Vec<String>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let mut args = Args {
        stack: Vec::new(),
        interactive: false,
    };
    for arg in env::args().skip(1) {
        if arg == "-i" || arg == "--interactive" {
            args.interactive = true;
        } else if arg == "-h" || arg == "--help" {
            let (width, _) = crossterm::terminal::size().unwrap_or((80, 80));
            eprintln!("{}", nroff_format(get_usage().as_str(), width as usize));
            process::exit(exitcode::USAGE);
        } else if arg == "--help-readme" {
            let rpn_calc = RpnCalc::new();
            let rpn_calc_help = create_help_string(&rpn_calc);
            println!("{}", get_readme_header().as_str());
            println!("{}", nroff_to_markdown(get_usage().as_str()));
            println!("{}", nroff_to_markdown(rpn_calc_help.as_str()));
            println!("{}", get_readme_footer().as_str());
            process::exit(exitcode::OK);
        } else {
            args.stack.push(arg);
        }
    }

    if let Err(err) = run(args) {
        eprintln!("{}", err);
        process::exit(exitcode::DATAERR);
    }
    return Ok(());
}

fn run(args: Args) -> Result<(), RpnCalcError> {
    let interactive_mode = args.stack.is_empty() || args.interactive;

    let mut rpn_calc = RpnCalc::new();
    for arg in args.stack {
        rpn_calc.push_str(arg.as_str())?;
    }

    if interactive_mode {
        run_tui(rpn_calc)?;
    } else {
        for stack_item in &rpn_calc.stack.items {
            println!("{}", rpn_calc.format_stack_item(stack_item));
        }
    }

    return Ok(());
}

fn get_usage() -> String {
    return "Usage: rpncalc [option]... [stack item]...
Reverse Polish Notation calculator

  -h, --help        display this help and exit
  -i, --interactive enter interactive mode even if stack items are supplied

By default, rpncalc will enter interactive mode if no stack items are supplied. If stack items \
are supplied you can use -i to force interactive mode.

In interactive mode typing \"help\" will show additional help.

.SH Examples

.IP \"add -1 plus 2\"
rpncalc -1 2 +

.IP \"add 0x42 plus 16 decimal and display the result as hex\"
rpncalc hex 0x42 16 +

.IP \"convert 5ft to meters\"
rpncalc \"5 ft\" \"0 m\" +"
        .to_string();
}

fn get_readme_header() -> String {
    return "
## rpncalc \
[![master](https://github.com/joeferner/rpncalc/actions/workflows/master.yml/badge.svg)]\
(https://github.com/joeferner/rpncalc/actions/workflows/master.yml) \
[![codecov](https://codecov.io/gh/joeferner/rpncalc/branch/master/graph/badge.svg?token=SFH1NL79H4)]\
(https://codecov.io/gh/joeferner/rpncalc)

rpncalc is a command line Reverse Polish Notation calculator.
"
    .to_string();
}

fn get_readme_footer() -> String {
    return "
## Development Workspace Setup

The main facility used for interacting with this project's lifecycle (build/test/format/lint) is
[cargo-make](https://sagiegurari.github.io/cargo-make). Therefore, this is the only dependency \
you need to install on your machine:

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
cargo make test-coverage-ci # Run all unit tests and write a code coverage report to a text file \
in LCOV format
cargo make format           # Formats (rewrites) every applicable file in the project
cargo make format-ci        # Formats (report only) every applicable file in the project
cargo make lint             # Lints (report only) every applicable file in the project
cargo make lint-watch       # Lints (report only) every applicable file in the project and \
re-lints whenever files change
cargo make generate-readme  # Updates README.md
cargo make pre-commit       # Runs pre-commit tasks
```
"
    .to_string();
}
