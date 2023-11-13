use crate::rpn_calc::{RpnCalc, RpnCalcError};
use clap::Parser;
use exitcode;
use std::process;

mod operator;
mod rpn_calc;
mod stack;
mod stack_item;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    stack: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let mut rpn_calc = RpnCalc::new();
    if args.stack.len() == 0 {
        unimplemented!("not implemented")
    } else {
        if let Err(err) = run_args(&mut rpn_calc, args) {
            eprintln!("{}", err);
            process::exit(exitcode::DATAERR);
        }
    }
}

fn run_args(rpn_calc: &mut RpnCalc, args: Args) -> Result<(), RpnCalcError> {
    for arg in args.stack {
        rpn_calc.push_str(arg.as_str())?;
    }
    for stack_item in rpn_calc.stack().items() {
        println!("{}", rpn_calc.format_stack_item(stack_item));
    }
    return Ok(());
}
