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
    let copyright = Paragraph::new(format!("{}", app.bottom_text))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        );

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

    let entry_list: [(i32, String, bool); 8] = [
        (1, String::from("Task 1"), false),
        (2, String::from("Task 2"), true),
        (3, String::from("Task 3"), false),
        (4, String::from("Task 4"), true),
        (5, String::from("Task 5"), true),
        (6, String::from("Task 6"), false),
        (7, String::from("Task 7"), true),
        (8, String::from("Task 8"), false),
    ];

    let entries_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
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

    for (id, text, completed) in &entry_list {
        let entry = Paragraph::new(format!("{}", text))
            .style(Style::default().fg(Color::White).bg(Color::Green))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );
        frame.render_widget(entry, entries_chunks[*id as usize]);
    }
}
