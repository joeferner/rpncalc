use crate::error::RpnCalcError;
use crate::rpn_calc::RpnCalc;
use crate::ui::run_interactive;
use clap::Parser;
use color_eyre::eyre;
use std::process;

mod error;
mod function;
mod functions;
mod number;
mod rpn_calc;
mod stack;
mod stack_item;
mod ui;
mod units;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // set to force interactive mode even if stack items are presented
    #[arg(short, long)]
    interactive: bool,

    stack: Vec<String>,
}

fn run(args: Args) -> Result<(), RpnCalcError> {
    let interactive_mode = args.stack.is_empty() || args.interactive;

    let mut rpn_calc = RpnCalc::new();
    for arg in args.stack {
        rpn_calc.push_str(arg.as_str())?;
    }

    if interactive_mode {
        run_interactive(rpn_calc)?;
    } else {
        for stack_item in &rpn_calc.stack.items {
            println!("{}", rpn_calc.format_stack_item(stack_item));
        }
    }

    return Ok(());
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        process::exit(exitcode::DATAERR);
    }
    return Ok(());
}
