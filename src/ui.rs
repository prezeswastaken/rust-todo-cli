use crate::app::{App, AppState};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

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

    if app.app_state == AppState::Main {
        for (i, task) in entry_list.iter().enumerate() {
            let completion_status: String;
            if task.completed {
                completion_status = "󰄬".to_string();
            } else {
                completion_status = "".to_string();
            }

            let color: Color;
            if i == app.current_position as usize {
                color = Color::Green;
            } else {
                color = Color::Black;
            }

            let entry = Paragraph::new(format!("{} {} {}", i+1, completion_status, task.text))
                .style(Style::default().fg(Color::White).bg(color))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::BOTTOM)
                        .style(Style::default().fg(Color::White))
                        .border_type(BorderType::Plain),
                );
            frame.render_widget(entry, entries_chunks[(i + 1) as usize]);
        }
    }

    frame.render_widget(
        Paragraph::new("Controls: ENTER - create new task | o - rename selected task | j - down | k - up | TAB - switch status of selected task")
            .block(
                Block::default()
                    .title(format!("{}", app.current_position+1))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left),
            
        entries_chunks[9],
    );

    let insert_name_popup: Paragraph = Paragraph::new(format!("{}", app.buffer))
        .block(
            Block::default()
                .title("Enter a name for your task")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);

    if app.app_state == AppState::Typing {
        frame.render_widget(insert_name_popup, centered_rect(50, 20, frame.size()));
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
