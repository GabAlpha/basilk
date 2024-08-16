use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Clear, HighlightSpacing, List, ListItem},
    Frame,
};
use tui_input::Input;

use crate::{project::Project, task::Task, ui::Ui, App, ViewMode};

pub struct View {}

impl View {
    pub fn show_add_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input("Add", f, area, input)
    }

    pub fn show_rename_modal(f: &mut Frame, area: Rect, input: &Input) {
        Ui::create_input("Rename", f, area, input)
    }

    pub fn show_delete_modal(app: &mut App, f: &mut Frame, area: Rect) {
        let title = match app.view_mode {
            ViewMode::DeleteTask => &Task::get_current(app).title,
            ViewMode::DeleteProject => &Project::get_current(app).title,
            _ => "",
        };

        Ui::create_question_modal(
            "Are you sure to delete?",
            format!("\"{}\"", title).as_str(),
            "Delete",
            f,
            area,
        )
    }

    pub fn show_select_task_status_modal(
        app: &mut App,
        status_items: &Vec<ListItem>,
        f: &mut Frame,
        area: Rect,
    ) {
        let area = Ui::create_rect_area(20, 5, area);

        let block = Block::bordered().title("Status");

        let task_status_list_widget = List::new(status_items.clone())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        f.render_widget(Clear, area);
        f.render_stateful_widget(task_status_list_widget, area, app.use_state())
    }

    pub fn show_items(app: &mut App, items: &Vec<ListItem>, f: &mut Frame, area: Rect) {
        let block: Block = match app.view_mode {
            ViewMode::ViewProjects
            | ViewMode::AddProject
            | ViewMode::RenameProject
            | ViewMode::DeleteProject => Block::bordered(),
            _ => {
                let project_title = Project::get_current(app).title.clone();
                Block::bordered().title(project_title)
            }
        };

        // Iterate through all elements in the `items` and stylize them.
        let items = items.clone();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        if app.view_mode != ViewMode::ChangeStatusTask {
            f.render_stateful_widget(items, area, app.use_state());
        } else {
            f.render_widget(items, area)
        }
    }
}
