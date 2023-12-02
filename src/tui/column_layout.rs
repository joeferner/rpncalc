use crate::error::RpnCalcError;
use crate::tui::console::Console;
use crate::tui::control::Control;
use crate::tui::HandleKeyEventResult;
use crossterm::event::KeyEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ColumnLayoutInit {
    pub top: u16,
    pub controls: Vec<Rc<RefCell<dyn Control>>>,
}

pub struct ColumnLayout {
    top: u16,
    controls: Vec<Rc<RefCell<dyn Control>>>,
    focused_control_index: usize,
}

impl ColumnLayout {
    pub fn new(init: ColumnLayoutInit) -> Self {
        let mut layout = ColumnLayout {
            top: 0,
            controls: init.controls,
            focused_control_index: 0,
        };
        layout.set_top(init.top);
        return layout;
    }

    pub fn set_focused_control_index(&mut self, idx: usize) -> () {
        self.focused_control_index = idx;
    }
}

impl Control for ColumnLayout {
    fn get_top(&self) -> u16 {
        return self.top;
    }

    fn set_top(&mut self, top: u16) -> () {
        self.top = top;
        let mut y = top;
        for c in &self.controls {
            let mut c = c.borrow_mut();
            c.set_top(y);
            y += c.get_height();
        }
    }

    fn get_height(&self) -> u16 {
        return self.controls.iter().map(|c| c.borrow().get_height()).sum();
    }

    fn set_width(&mut self, width: u16) -> () {
        for c in &self.controls {
            c.borrow_mut().set_width(width);
        }
    }

    fn redraw(&self, console: &mut dyn Console) -> Result<(), RpnCalcError> {
        for (idx, c) in self.controls.iter().enumerate() {
            if idx != self.focused_control_index {
                c.borrow().redraw(console)?;
            }
        }

        // draw focused control last to allow cursor to be placed in correct location
        if let Some(c) = self.controls.get(self.focused_control_index) {
            c.borrow().redraw(console)?;
        }

        return Ok(());
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<HandleKeyEventResult, RpnCalcError> {
        if let Some(c) = self.controls.get(self.focused_control_index) {
            let mut c = c.borrow_mut();
            return c.handle_key_event(key);
        }
        return Ok(HandleKeyEventResult::Continue);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tui::console::tests::MockConsole;
    use crate::tui::control::tests::MockControl;
    use crossterm::event::{KeyCode, KeyEventKind, KeyEventState, KeyModifiers};

    #[test]
    fn test_top() {
        let control1 = Rc::new(RefCell::new(MockControl::new()));
        let control2 = Rc::new(RefCell::new(MockControl::new()));

        let mut layout = ColumnLayout::new(ColumnLayoutInit {
            top: 3,
            controls: vec![control1.clone(), control2.clone()],
        });

        assert_eq!(3, layout.get_top());
        assert_eq!(layout.get_top(), control1.borrow().get_top());
        assert_eq!(layout.get_top() + control1.borrow().height, control2.borrow().get_top());

        layout.set_top(10);
        assert_eq!(10, layout.get_top());
        assert_eq!(layout.get_top(), control1.borrow().get_top());
        assert_eq!(layout.get_top() + control1.borrow().height, control2.borrow().get_top());
    }

    #[test]
    fn test_height() {
        let control1 = Rc::new(RefCell::new(MockControl::new()));
        let control2 = Rc::new(RefCell::new(MockControl::new()));

        let layout = ColumnLayout::new(ColumnLayoutInit {
            top: 3,
            controls: vec![control1.clone(), control2.clone()],
        });

        assert_eq!(
            control1.borrow().get_height() + control2.borrow().get_height(),
            layout.get_height()
        );

        control1.borrow_mut().height += 5;
        assert_eq!(
            control1.borrow().get_height() + control2.borrow().get_height(),
            layout.get_height()
        );
    }

    #[test]
    fn test_width() {
        let control1 = Rc::new(RefCell::new(MockControl::new()));
        let control2 = Rc::new(RefCell::new(MockControl::new()));

        let mut layout = ColumnLayout::new(ColumnLayoutInit {
            top: 3,
            controls: vec![control1.clone(), control2.clone()],
        });

        let new_width = control1.borrow().width + 1;
        layout.set_width(new_width);
        assert_eq!(new_width, control1.borrow().width);
        assert_eq!(new_width, control2.borrow().width);
    }

    #[test]
    fn test_redraw() {
        let control1 = Rc::new(RefCell::new(MockControl::new()));
        let control2 = Rc::new(RefCell::new(MockControl::new()));

        let layout = ColumnLayout::new(ColumnLayoutInit {
            top: 3,
            controls: vec![control1.clone(), control2.clone()],
        });
        let mut console = MockConsole::new(80, 40);
        layout.redraw(&mut console).unwrap();

        assert_eq!('a', console.get_ch(0, control1.borrow().top).unwrap());
        assert_eq!(
            'b',
            console.get_ch(control1.borrow().width, control1.borrow().top).unwrap()
        );
        assert_eq!(
            'c',
            console
                .get_ch(0, control1.borrow().top + control1.borrow().height - 1)
                .unwrap()
        );
        assert_eq!(
            'd',
            console
                .get_ch(
                    control1.borrow().width,
                    control1.borrow().top + control1.borrow().height - 1
                )
                .unwrap()
        );

        assert_eq!('a', console.get_ch(0, control2.borrow().top).unwrap());
        assert_eq!(
            'b',
            console.get_ch(control2.borrow().width, control2.borrow().top).unwrap()
        );
        assert_eq!(
            'c',
            console
                .get_ch(0, control2.borrow().top + control2.borrow().height - 1)
                .unwrap()
        );
        assert_eq!(
            'd',
            console
                .get_ch(
                    control2.borrow().width,
                    control2.borrow().top + control2.borrow().height - 1
                )
                .unwrap()
        );
    }

    #[test]
    fn test_handle_key_event() {
        let control1 = Rc::new(RefCell::new(MockControl::new()));
        let control2 = Rc::new(RefCell::new(MockControl::new()));

        let mut layout = ColumnLayout::new(ColumnLayoutInit {
            top: 3,
            controls: vec![control1.clone(), control2.clone()],
        });

        let key = KeyEvent {
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
            modifiers: KeyModifiers::NONE,
            code: KeyCode::Enter,
        };

        layout.set_focused_control_index(0);
        layout.handle_key_event(key).unwrap();
        assert_eq!(key, *control1.borrow().key_events.get(0).unwrap());
        assert_eq!(0, control2.borrow().key_events.len());

        control1.borrow_mut().key_events.clear();
        control2.borrow_mut().key_events.clear();

        layout.set_focused_control_index(1);
        layout.handle_key_event(key).unwrap();
        assert_eq!(0, control1.borrow().key_events.len());
        assert_eq!(key, *control2.borrow().key_events.get(0).unwrap());

        control1.borrow_mut().key_events.clear();
        control2.borrow_mut().key_events.clear();

        layout.set_focused_control_index(2);
        layout.handle_key_event(key).unwrap();
        assert_eq!(0, control1.borrow().key_events.len());
        assert_eq!(0, control2.borrow().key_events.len());
    }
}
