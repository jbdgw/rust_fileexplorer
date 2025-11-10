#[cfg(feature = "tui")]
use crate::models::EntryKind;
#[cfg(feature = "tui")]
use crate::tui::app::App;
#[cfg(feature = "tui")]
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
#[cfg(feature = "tui")]
use std::io::{self, Write};

#[cfg(feature = "tui")]
/// Run the interactive TUI
pub fn run(app: &mut App) -> io::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // Main loop
    let result = main_loop(app, &mut stdout);

    // Cleanup terminal
    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        ResetColor
    )?;
    terminal::disable_raw_mode()?;

    result
}

#[cfg(feature = "tui")]
fn main_loop(app: &mut App, stdout: &mut io::Stdout) -> io::Result<()> {
    loop {
        draw_ui(app, stdout)?;

        if app.should_quit {
            break;
        }

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(app, key)?;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn handle_key_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Down | KeyCode::Char('j') => app.next(),
        KeyCode::Up | KeyCode::Char('k') => app.previous(),
        KeyCode::PageDown => app.page_down(10),
        KeyCode::PageUp => app.page_up(10),
        KeyCode::Enter => app.enter_selected().map_err(io::Error::other)?,
        KeyCode::Backspace if !app.filter.is_empty() => app.remove_filter_char(),
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => app.clear_filter(),
        KeyCode::Char('.') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_hidden().map_err(io::Error::other)?
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_dirs_first()
        }
        KeyCode::Char('-') | KeyCode::Left => app.go_up().map_err(io::Error::other)?,
        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.add_filter_char(c)
        }
        _ => {}
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn draw_ui(app: &App, stdout: &mut io::Stdout) -> io::Result<()> {
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let (width, height) = terminal::size()?;
    let content_height = height.saturating_sub(3) as usize;

    // Draw header
    draw_header(app, stdout, width)?;

    // Draw file list
    draw_file_list(app, stdout, content_height)?;

    // Draw footer
    draw_footer(app, stdout, width, height)?;

    stdout.flush()?;
    Ok(())
}

#[cfg(feature = "tui")]
fn draw_header(app: &App, stdout: &mut io::Stdout, width: u16) -> io::Result<()> {
    let separator = "─".repeat(width as usize);
    queue!(
        stdout,
        SetBackgroundColor(Color::Blue),
        SetForegroundColor(Color::White),
        Print(format!(
            " {:<width$}",
            app.path.display(),
            width = width as usize - 1
        )),
        ResetColor,
        cursor::MoveTo(0, 1),
        Print(&separator),
    )?;
    Ok(())
}

#[cfg(feature = "tui")]
fn draw_file_list(app: &App, stdout: &mut io::Stdout, content_height: usize) -> io::Result<()> {
    let start_index = if app.selected_index >= content_height {
        app.selected_index - content_height + 1
    } else {
        0
    };

    let end_index = (start_index + content_height).min(app.filtered_entries.len());

    for (i, entry) in app.filtered_entries[start_index..end_index]
        .iter()
        .enumerate()
    {
        let row = i as u16 + 2;
        queue!(stdout, cursor::MoveTo(0, row))?;

        let is_selected = start_index + i == app.selected_index;

        if is_selected {
            queue!(
                stdout,
                SetBackgroundColor(Color::DarkGrey),
                SetForegroundColor(Color::White),
            )?;
        }

        // Icon based on type
        let icon = match entry.kind {
            EntryKind::Dir => "📁",
            EntryKind::File => "📄",
            EntryKind::Symlink => "🔗",
        };

        // Format size
        let size_str = if entry.kind == EntryKind::File {
            humansize::format_size(entry.size, humansize::BINARY)
        } else {
            "-".to_string()
        };

        queue!(
            stdout,
            Print(format!(
                " {} {:40} {:>10}",
                icon,
                &entry.name.chars().take(40).collect::<String>(),
                size_str
            ))
        )?;

        if is_selected {
            queue!(stdout, ResetColor)?;
        }
    }

    // Show empty message if no entries
    if app.filtered_entries.is_empty() {
        queue!(
            stdout,
            cursor::MoveTo(0, 2),
            SetForegroundColor(Color::DarkGrey),
            Print("  No entries found"),
            ResetColor
        )?;
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn draw_footer(app: &App, stdout: &mut io::Stdout, width: u16, height: u16) -> io::Result<()> {
    let footer_row = height - 1;

    let separator = "─".repeat(width as usize);
    queue!(
        stdout,
        cursor::MoveTo(0, footer_row - 1),
        Print(&separator),
        cursor::MoveTo(0, footer_row),
    )?;

    // Status line
    let status = format!(
        " {} entries | Filter: {} | Hidden: {} | q:quit ↑↓:navigate ⏎:enter -:up",
        app.filtered_entries.len(),
        if app.filter.is_empty() {
            "<none>"
        } else {
            &app.filter
        },
        if app.show_hidden { "on" } else { "off" }
    );

    queue!(
        stdout,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::White),
        Print(format!("{:<width$}", status, width = width as usize)),
        ResetColor,
    )?;

    Ok(())
}
