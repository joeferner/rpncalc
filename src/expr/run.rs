use anyhow::{anyhow, Context, Result};

use crate::{
    stack::item::StackItem,
    state::RpnState,
    undo_action::{multi::MultiUndoEvent, push::PushUndoEvent, UndoEvent},
};

use super::{lexer::ExprLexer, parser::parse_expression_from_tokenizer, Expr};

pub fn run_expression(s: &str, state: &mut RpnState) -> Result<()> {
    let tokenizer = ExprLexer::new(s).context("tokenizing failed")?;
    let expr = parse_expression_from_tokenizer(tokenizer).context("parse failed")?;

    let mut undos: Vec<Box<dyn UndoEvent>> = vec![];
    match run_expr(&expr, state, &mut undos) {
        Ok(_) => {
            if undos.is_empty() {
                // do nothing
                Ok(())
            } else if undos.len() == 1 {
                let undo = undos.remove(0);
                state.undo_stack.push_undo_stack(undo);
                Ok(())
            } else {
                state
                    .undo_stack
                    .push_undo_stack(Box::new(MultiUndoEvent::new(undos)));
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

fn run_expr(expr: &Expr, state: &mut RpnState, undos: &mut Vec<Box<dyn UndoEvent>>) -> Result<()> {
    match expr {
        Expr::StackItem(stack_item) => {
            state.stack.push(stack_item.clone());
            undos.push(Box::new(PushUndoEvent::new(stack_item.clone())));
            Ok(())
        }
        Expr::Identifier(ident) => run_ident(ident, state, undos),
        Expr::BinaryOp { lhs, op, rhs } => run_binary_op(lhs, op, rhs, state, undos),
        Expr::FunctionCall(ident, args) => run_function_call(ident, args, state, undos),
    }
}

fn run_function_call(
    fn_name: &str,
    args: &Vec<Expr>,
    state: &mut RpnState,
    undos: &mut Vec<Box<dyn UndoEvent>>,
) -> Result<()> {
    for arg in args {
        run_expr(arg, state, undos)?;
    }
    run_ident(fn_name, state, undos)
}

fn run_ident(ident: &str, state: &mut RpnState, undos: &mut Vec<Box<dyn UndoEvent>>) -> Result<()> {
    if let Some(c) = state.constants.get(ident) {
        let stack_item = StackItem::Number(c.value, 10);
        state.stack.push(stack_item.clone());
        undos.push(Box::new(PushUndoEvent::new(stack_item)));
        Ok(())
    } else if let Some(stack_item) = state.variables.get(ident) {
        state.stack.push(stack_item.clone());
        undos.push(Box::new(PushUndoEvent::new(stack_item.clone())));
        Ok(())
    } else if let Some(f) = state.functions.get(ident) {
        let undo = f.clone().execute(state)?;
        undos.push(undo);
        Ok(())
    } else {
        Err(anyhow!("unknown constant, variable, or function: {ident}"))
    }
}

fn run_binary_op(
    lhs: &Expr,
    op: &str,
    rhs: &Expr,
    state: &mut RpnState,
    undos: &mut Vec<Box<dyn UndoEvent>>,
) -> Result<()> {
    run_expr(lhs, state, undos)?;
    run_expr(rhs, state, undos)?;
    run_ident(op, state, undos)
}
