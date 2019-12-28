use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use std::f64::INFINITY;
use tui::widgets::{Gauge, Block, Widget, Borders};
use tui::style::{Style, Color, Modifier};
use std::ops::Range;

pub fn progress_bar(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
                regression: (f64, f64),
                time_bounds: &Range<f64>,
                target: Option<f64>) {
    if let Some(target) = target {
        let eta: f64 = (target - regression.1) / regression.0;
        let remaining_time = (eta - time_bounds.end).max(0.0);
        let completion_percentage = (time_bounds.end / eta * 100.0) as i16;
        let gauge_title = &format!(
            "Estimated time of completion: {:.*}s (remaining: {:.*}s)",
            2, if remaining_time <= 0.0 { INFINITY } else { eta },
            2, if remaining_time <= 0.0 { INFINITY } else { remaining_time }
        );
        let mut gauge_size = frame.size();

        gauge_size.height = frame.size().height / 10;
        gauge_size.y = frame.size().height - frame.size().height / 10;
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(gauge_title))
            .style(Style::default().fg(Color::White).bg(Color::Black).modifier(Modifier::ITALIC))
            .percent(completion_percentage.max(0).min(100) as u16)
            .render(frame, gauge_size);
    }
}