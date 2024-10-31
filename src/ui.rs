use annotate_snippets::{Level, Renderer};
use log::error;
use ratatui::layout::Constraint::{Length, Min, Percentage};
use ratatui::layout::{Alignment, Position};
use ratatui::symbols::{border, line};
use ratatui::text::Text;
use ratatui::widgets::{Borders, List, ListDirection, ListItem, Paragraph};
use ratatui::{layout::Layout, widgets::Block, Frame};

use crate::expr::run::run_expression;
use crate::expr::ExprError;
use crate::stack::item::{StackItem, StackItemToStringOpts};
use crate::state::angle_mode::AngleMode;
use crate::state::RpnState;

pub fn draw(frame: &mut Frame, state: &mut RpnState) {
    let status_right_text = get_status_right_text(state);

    let vertical_main = Layout::vertical([Min(0), Length(1)]);
    let [main_area, status_area] = vertical_main.areas(frame.area());

    let horizontal = Layout::horizontal([Min(60), Percentage(100)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let status_horizontal =
        Layout::horizontal([Percentage(100), Length(status_right_text.len() as u16)]);
    let [status_left_area, status_right_area] = status_horizontal.areas(status_area);

    let vertical_stack = Layout::vertical([Min(0), Length(3)]);
    let [stack_area, input_area] = vertical_stack.areas(left_area);

    let info_text = get_info_text(state, right_area.width as usize);
    let status_left_text = get_status_left_text(state, status_left_area.width as usize);

    let mut items: Vec<ListItem> = state
        .stack
        .iter()
        .map(|stack_item| {
            let v = stack_item.to_string_opts(
                &StackItemToStringOpts {
                    base: None,
                    precision: None,
                    left_pad_with_zeros: true,
                    include_base_prefix: true,
                },
                state,
            );
            // add a space to the right so that when a user double clicks
            // a value it doesn't select the right border as well
            let text = Text::from(format!("{v} ")).alignment(Alignment::Right);
            ListItem::new(text)
        })
        .collect();
    items.reverse();

    let list = List::new(items).direction(ListDirection::BottomToTop);

    frame.render_widget(Block::new().title(status_left_text), status_left_area);
    frame.render_widget(Block::new().title(status_right_text), status_right_area);

    frame.render_stateful_widget(
        list.block(
            Block::new()
                .borders(Borders::TOP | Borders::LEFT)
                .title("Stack"),
        ),
        stack_area,
        &mut state.ui_stack_state,
    );
    frame.render_widget(
        Paragraph::new(state.ui_input_state.get_input()).block(
            Block::new()
                .border_set(border::Set {
                    top_left: line::NORMAL.vertical_right,
                    ..border::PLAIN
                })
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .title(""),
        ),
        input_area,
    );
    frame.render_widget(
        Paragraph::new(info_text).block(
            Block::bordered()
                .border_set(border::Set {
                    top_left: line::NORMAL.horizontal_down,
                    bottom_left: line::NORMAL.horizontal_up,
                    ..border::PLAIN
                })
                .title("Info"),
        ),
        right_area,
    );

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        input_area.x + state.ui_input_state.get_character_index() as u16 + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

fn get_info_text(state: &mut RpnState, width: usize) -> String {
    if let Some(e) = &state.error {
        if let Some(e) = e.downcast_ref::<ExprError>() {
            let msg = Level::Error.title("").snippet(e.get_snippet());
            let s = format!("{}", Renderer::plain().term_width(width).render(msg));
            return s;
        }
    }

    let n = if state.ui_input_state.is_empty() {
        state.stack.peek(0).cloned()
    } else {
        let s = state.ui_input_state.get_input().to_string();
        match run_expression(&s, state) {
            Ok(_) => {
                let v = state.stack.peek(0).cloned();
                if let Err(e) = state.undo() {
                    error!("failed to undo; error = {e}");
                }
                v
            }
            Err(_) => None,
        }
    };

    if let Some(n) = n {
        let hex = if n.is_integer() {
            n.to_string_opts(
                &StackItemToStringOpts {
                    base: Some(16),
                    precision: None,
                    left_pad_with_zeros: true,
                    include_base_prefix: false,
                },
                state,
            )
        } else {
            "".to_string()
        };

        let dec = match n {
            StackItem::Number(_, _) => n.to_string_opts(
                &StackItemToStringOpts {
                    base: Some(10),
                    precision: None,
                    left_pad_with_zeros: false,
                    include_base_prefix: false,
                },
                state,
            ),
            StackItem::String(_) => "".to_string(),
            StackItem::Undefined => "".to_string(),
        };

        let oct = if n.is_integer() {
            n.to_string_opts(
                &StackItemToStringOpts {
                    base: Some(8),
                    precision: None,
                    left_pad_with_zeros: true,
                    include_base_prefix: false,
                },
                state,
            )
        } else {
            "".to_string()
        };

        let bin = if n.is_integer() {
            n.to_string_opts(
                &StackItemToStringOpts {
                    base: Some(2),
                    precision: None,
                    left_pad_with_zeros: true,
                    include_base_prefix: false,
                },
                state,
            )
        } else {
            "".to_string()
        };

        return format!("Hex: {}\nDec: {}\nOct: {}\nBin: {}\n", hex, dec, oct, bin);
    }
    "Hex:\nDec:\nOct:\nBin:\n".to_string()
}

fn get_status_right_text(state: &RpnState) -> String {
    let angle_mode = match state.angle_mode {
        AngleMode::Degrees => "DEG",
        AngleMode::Radians => "RAD",
    };
    format!(" {angle_mode} ")
}

fn get_status_left_text(state: &RpnState, width: usize) -> String {
    if let Some(e) = &state.error {
        format!("{e}")
    } else if let Some(completions) = &state.completions {
        let s = completions.join(" ");
        s[0..width.min(s.len())].to_string()
    } else {
        "".to_string()
    }
}
