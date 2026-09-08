use std::io::{self, Stdout};
use std::time::Instant;

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};

use crate::ops::progress::{ExportEvent, ExportProgress};

/// Renders a progress bar filled with a background color rather than ratatui's `Gauge`
/// glyph-based fill: many terminal fonts don't render the `█` block glyph tall enough to
/// cover the full cell height, leaving a thin sliver of the default background visible.
/// A background-color fill (using spaces) always covers the cell exactly.
fn render_progress_bar(
    frame: &mut Frame,
    area: Rect,
    ratio: f64,
    label: &str,
    fill: Color,
    track: Color,
) {
    if area.is_empty() {
        return;
    }
    let row = area.top();
    let end = area.left() + (f64::from(area.width) * ratio).round() as u16;
    let label_width = area.width.min(label.chars().count() as u16);
    let label_col = area.left() + (area.width - label_width) / 2;

    let buf = frame.buffer_mut();
    for x in area.left()..area.right() {
        let bg = if x < end { fill } else { track };
        buf[(x, row)].set_symbol(" ").set_fg(bg).set_bg(bg);
    }
    for (i, ch) in label.chars().enumerate() {
        let x = label_col + i as u16;
        if x >= area.right() {
            break;
        }
        let bg = if x < end { fill } else { track };
        let fg = if x < end { track } else { fill };
        buf[(x, row)].set_char(ch).set_fg(fg).set_bg(bg);
    }
}

pub struct TuiProgress {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    state: TuiState,
}

struct TuiState {
    total_files: usize,
    current_file_index: usize,
    current_file_name: String,
    total_pages: usize,
    current_page_index: usize,
    current_page_name: String,
    current_action: String,
    warnings: Vec<String>,
    files_completed: usize,
    start_time: Instant,
}

impl TuiState {
    fn new() -> Self {
        TuiState {
            total_files: 0,
            current_file_index: 0,
            current_file_name: String::new(),
            total_pages: 0,
            current_page_index: 0,
            current_page_name: String::new(),
            current_action: String::from("Discovering files..."),
            warnings: Vec::new(),
            files_completed: 0,
            start_time: Instant::now(),
        }
    }

    fn elapsed_display(&self) -> String {
        let secs = self.start_time.elapsed().as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else {
            format!("{}m{}s", secs / 60, secs % 60)
        }
    }

    fn file_progress_ratio(&self) -> f64 {
        if self.total_files == 0 {
            return 0.0;
        }
        self.files_completed as f64 / self.total_files as f64
    }

    fn page_progress_ratio(&self) -> f64 {
        if self.total_pages == 0 {
            return 0.0;
        }
        self.current_page_index as f64 / self.total_pages as f64
    }
}

impl TuiProgress {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::with_options(
            backend,
            TerminalOptions {
                viewport: Viewport::Inline(8),
            },
        )?;
        terminal.hide_cursor()?;

        Ok(TuiProgress {
            terminal,
            state: TuiState::new(),
        })
    }

    fn render(&mut self) {
        let state = &self.state;
        let elapsed = state.elapsed_display();
        let file_ratio = state.file_progress_ratio();
        let page_ratio = state.page_progress_ratio();
        let file_label = format!(
            "{}/{}  ({})",
            state.files_completed, state.total_files, elapsed
        );
        let current_file = state.current_file_name.clone();
        let page_label = format!(
            "{}/{}  {}",
            state.current_page_index, state.total_pages, state.current_page_name
        );
        let action = state.current_action.clone();
        let warning_count = state.warnings.len();

        let _ = self.terminal.draw(|frame| {
            let area = frame.area();
            let chunks = Layout::vertical([
                Constraint::Length(1), // Title
                Constraint::Length(1), // File progress bar
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Current file
                Constraint::Length(1), // Page progress bar
                Constraint::Length(1), // Status
                Constraint::Length(1), // Spacer
                Constraint::Length(1), // Warnings
            ])
            .split(area);

            // Title line
            let title = Line::from(vec![
                Span::styled(
                    " drawio-exporter ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  Exporting files"),
            ]);
            frame.render_widget(Paragraph::new(title), chunks[0]);

            // File progress bar
            render_progress_bar(
                frame,
                chunks[1],
                file_ratio,
                &file_label,
                Color::Green,
                Color::DarkGray,
            );

            // Current file
            let file_line = Line::from(vec![
                Span::styled(" File: ", Style::default().fg(Color::Blue)),
                Span::styled(
                    current_file,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]);
            frame.render_widget(Paragraph::new(file_line), chunks[3]);

            // Page progress bar
            render_progress_bar(
                frame,
                chunks[4],
                page_ratio,
                &page_label,
                Color::Cyan,
                Color::DarkGray,
            );

            // Status line
            let status_line = Line::from(vec![
                Span::styled(" Status: ", Style::default().fg(Color::Blue)),
                Span::styled(action, Style::default().fg(Color::Yellow)),
            ]);
            frame.render_widget(Paragraph::new(status_line), chunks[5]);

            // Warnings line
            if warning_count > 0 {
                let warn_line = Line::from(vec![Span::styled(
                    format!(" {} warning(s)", warning_count),
                    Style::default().fg(Color::Yellow),
                )]);
                frame.render_widget(Paragraph::new(warn_line), chunks[7]);
            }
        });
    }

    fn teardown(&mut self) {
        let _ = self.terminal.show_cursor();
        let _ = self.terminal.clear();
        let _ = disable_raw_mode();
    }

    fn print_summary(&self) {
        let elapsed = self.state.elapsed_display();
        println!(
            "Export complete: {} file(s) exported ({})",
            self.state.files_completed, elapsed
        );
        if !self.state.warnings.is_empty() {
            println!("{} warning(s) encountered:", self.state.warnings.len());
            for warning in &self.state.warnings {
                println!("  {}", warning);
            }
        }
    }
}

impl Drop for TuiProgress {
    fn drop(&mut self) {
        self.teardown();
    }
}

impl ExportProgress for TuiProgress {
    fn on_event(&mut self, event: ExportEvent<'_>) {
        match event {
            ExportEvent::ExportStart { total_files } => {
                self.state.total_files = total_files;
                self.state.current_action =
                    format!("Starting export of {} file(s)...", total_files);
            }
            ExportEvent::FileStart {
                path, file_index, ..
            } => {
                self.state.current_file_index = file_index;
                self.state.current_file_name = path.to_string();
                self.state.current_page_index = 0;
                self.state.total_pages = 0;
                self.state.current_page_name.clear();
                self.state.current_action = "Preparing file export...".to_string();
            }
            ExportEvent::PageStart {
                page_index,
                page_name,
                total_pages,
            } => {
                self.state.current_page_index = page_index;
                self.state.current_page_name = page_name.to_string();
                self.state.total_pages = total_pages;
                self.state.current_action =
                    format!("Exporting page {} of {}...", page_index, total_pages);
            }
            ExportEvent::AllPagesStart => {
                self.state.current_action = "Exporting all pages...".to_string();
            }
            ExportEvent::GenerateFile { format } => {
                self.state.current_action = format!("Generating {} file...", format);
            }
            ExportEvent::GenerateDocFile { format } => {
                self.state.current_action = format!("Generating {} documentation...", format);
            }
            ExportEvent::IncludeLinks { format } => {
                self.state.current_action = format!("Including links in {} file...", format);
            }
            ExportEvent::LinkIncluded { label, url } => {
                self.state.current_action = format!("Link '{}' -> {}", label, url);
            }
            ExportEvent::LinkWarning { message } => {
                self.state.warnings.push(message);
            }
            ExportEvent::FileComplete => {
                self.state.files_completed += 1;
            }
            ExportEvent::ExportComplete => {
                self.teardown();
                self.print_summary();
                return;
            }
        }
        self.render();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn progress_bar_fills_every_cell_with_a_solid_background() {
        let backend = TestBackend::new(20, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_progress_bar(
                    frame,
                    Rect::new(0, 0, 20, 1),
                    0.65,
                    "15/23",
                    Color::Cyan,
                    Color::DarkGray,
                );
            })
            .unwrap();
        let buf = terminal.backend().buffer().clone();
        for x in 0..20 {
            let cell = &buf[(x, 0)];
            let expected_bg = if x < 13 { Color::Cyan } else { Color::DarkGray };
            assert_eq!(cell.bg, expected_bg, "cell {x} has unexpected background");
            assert_ne!(cell.symbol(), "█", "cell {x} should not use a glyph fill");
        }
    }
}
