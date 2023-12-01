
## rpncalc [![master](https://github.com/joeferner/rpncalc/actions/workflows/master.yml/badge.svg)](https://github.com/joeferner/rpncalc/actions/workflows/master.yml) [![codecov](https://codecov.io/gh/joeferner/rpncalc/branch/master/graph/badge.svg?token=SFH1NL79H4)](https://codecov.io/gh/joeferner/rpncalc)

rpncalc is a command line Reverse Polish Notation calculator.

Usage: rpncalc [option]... [stack item]...

Reverse Polish Notation calculator


  -h, --help        display this help and exit

  -i, --interactive enter interactive mode even if stack items are supplied


By default, rpncalc will enter interactive mode if no stack items are supplied. If stack items are supplied you can use -i to force interactive mode.


In interactive mode typing "help" will show additional help.


## Examples

#### add -1 plus 2
rpncalc -1 2 +


#### add 0x42 plus 16 decimal and display the result as hex
rpncalc hex 0x42 16 +


#### convert 5ft to meters
rpncalc "5 ft" "0 m" +


## BASICS
To enter a negative number, you can either enter the positive number and negate it (neg or _) or enter a space followed by the negative number.


To enter scientific notation number, enter the number followed by "e" then the power (e.g. 1.23e4).


## ARITHMETIC FUNCTIONS
#### * or mul
Multiply the top two items on the stack

#### + or add
Add the top two items on the stack

#### - or sub
Subtract the top two items on the stack

#### / or div
Divides the top two items on the stack

#### ^ or pow
Raise the second item on the stack to the first item on the stack.

#### _ or neg
Negate the first item on the stack.

#### sqrt
Take the square root of the top item on the stack


## BASE FUNCTIONS
#### bin
Sets the current display to base 2.

#### dec
Sets the current display to base 10.

#### hex
Sets the current display to base 16.

#### oct
Sets the current display to base 8.


## STACK FUNCTIONS
#### copy
Copy the top item on the stack to the clipboard.

#### drop
Drop the top item on the stack.

#### dup
Duplicate the top item on the stack.

#### paste
Paste the clipboard text onto the stack.


## TRIG FUNCTIONS
#### cos
Calculates the cosine of the top item on the stack using the current angle mode

#### deg
Sets the current angle mode to degrees. Functions taking angles will assume the given angle is in degrees. Functions returning angles will return the angle in degrees.

#### grad
Sets the current angle mode to gradians. Functions taking angles will assume the given angle is in gradians. Functions returning angles will return the angle in gradians.

#### rad
Sets the current angle mode to radians. Functions taking angles will assume the given angle is in radians. Functions returning angles will return the angle in radians.

#### sin
Calculates the sine of the top item on the stack using the current angle mode

#### tan
Calculates the tangent of the top item on the stack using the current angle mode


## CONSTANTS
#### c = 299792458 m/s
the speed of light in vacuum

#### pi or π = 3.141592653589793
the ratio of a circle's circumference to its diameter





## Development Workspace Setup

The main facility used for interacting with this project's lifecycle (build/test/format/lint) is
[cargo-make](https://sagiegurari.github.io/cargo-make). Therefore, this is the only dependency you need to install on your machine:

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
cargo make generate-readme  # Updates README.md
cargo make pre-commit       # Runs pre-commit tasks
```

