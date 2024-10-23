use ratatui::layout::Constraint::{Fill, Length, Min};
use ratatui::layout::Position;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{layout::Layout, widgets::Block, Frame};

use crate::state::RpnState;

pub fn draw(frame: &mut Frame, state: &mut RpnState) {
    let status_text = get_status_text(state);

    let vertical_main = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical_main.areas(frame.area());

    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let vertical_stack = Layout::vertical([Min(0), Length(3)]);
    let [stack_area, input_area] = vertical_stack.areas(left_area);

    frame.render_widget(Block::new().title("RPN Calculator"), title_area);
    frame.render_widget(Block::new().title(status_text), status_area);

    frame.render_widget(
        Block::new()
            .borders(Borders::TOP | Borders::LEFT)
            .title("Stack"),
        stack_area,
    );
    frame.render_widget(
        Paragraph::new(state.input.get_input()).block(
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
        input_area.x + state.input.get_character_index() as u16 + 1,
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
