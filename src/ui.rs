



//use diesel_demo_step_1_sqlite::*;




use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    // "Welcome to amazing TODO app by prezes!"

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title("Welcome to blazingly fast TODO app by PREZES!")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    );

    let entry_list = &app.tasks;
    let entries_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(frame.size());

    for (i, task) in entry_list.iter().enumerate() {
        let completion_status: String;
        if task.completed {
            completion_status = "󰄬".to_string();
        }

        else {
            completion_status = "".to_string();
        }

        let color: Color;  
        if i == app.current_position as usize{
            color = Color::Green;
        } else {
            color = Color::Black;
        }

        let entry = Paragraph::new(format!("{} {} {}", task.id, completion_status, task.text))
            .style(Style::default().fg(Color::White).bg(color))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );
        frame.render_widget(entry, entries_chunks[(i+1) as usize]);
    }

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::default()
                    .title(format!("{}", app.current_position))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center),
            entries_chunks[9]
    );
}
