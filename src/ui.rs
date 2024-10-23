use ratatui::layout::Constraint::{Length, Min, Percentage};
use ratatui::layout::{Alignment, Position};
use ratatui::text::Text;
use ratatui::widgets::{Borders, List, ListDirection, ListItem, Paragraph};
use ratatui::{layout::Layout, widgets::Block, Frame};

use crate::state::RpnState;

pub fn draw(frame: &mut Frame, state: &mut RpnState) {
    let status_text = get_status_text(state);

    let vertical_main = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical_main.areas(frame.area());

    let horizontal = Layout::horizontal([Min(60), Percentage(100)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let vertical_stack = Layout::vertical([Min(0), Length(3)]);
    let [stack_area, input_area] = vertical_stack.areas(left_area);

    let mut items: Vec<ListItem> = state
        .stack
        .iter()
        .map(|item| {
            let text = Text::from(format!("{}", item)).alignment(Alignment::Right);
            ListItem::new(text)
        })
        .collect();
    items.reverse();

    let list = List::new(items).direction(ListDirection::BottomToTop);

    frame.render_widget(Block::new().title("RPN Calculator"), title_area);
    frame.render_widget(Block::new().title(status_text), status_area);

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
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .title(""),
        ),
        input_area,
    );
    frame.render_widget(Block::bordered().title("Info"), right_area);

    frame.set_cursor_position(Position::new(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        input_area.x + state.ui_input_state.get_character_index() as u16 + 1,
        // Move one line down, from the border to the input line
        input_area.y + 1,
    ));
}

fn get_status_text(state: &RpnState) -> String {
    if let Some(e) = &state.error {
        format!("{e}")
    } else {
        "".to_string()
    }
}
