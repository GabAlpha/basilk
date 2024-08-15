use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_input::Input;

pub struct Ui {}

impl Ui {
    pub fn create_rect_area(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Min(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Min(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }

    pub fn create_input(title: &str, f: &mut Frame, area: Rect, input: &Input) {
        let area = Ui::create_rect_area(50, 3, area);

        let width = area.width.max(3) - 3;
        let scroll = input.visual_scroll(width as usize);

        let input_to_show = Paragraph::new(input.value())
            .block(Block::default().borders(Borders::ALL).title(title))
            .scroll((0, scroll as u16));

        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(input_to_show, area);

        f.set_cursor(
            // Put cursor past the end of the input text
            area.x + ((input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            area.y + 1,
        )
    }
}
