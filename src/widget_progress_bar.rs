use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use std::f64::INFINITY;
use tui::widgets::{Gauge, Block, Widget, Borders};
use tui::style::{Style, Color, Modifier};

pub fn progress_bar(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
                regression: (f64, f64),
                min_time: f64,
                max_time: f64,
                target: Option<f64>) {
    if let Some(target) = target {
        let ETA: f64 = (target - regression.1) / regression.0;
        let remaining_time = (ETA - max_time).max(0.0);
        let completion_percentage = (max_time / ETA * 100.0) as i16;
        let gauge_title = &format!(
            "Estimated time of completion: {:.*}s (remaining: {:.*}s)",
            2, if remaining_time <= 0.0 { INFINITY } else { ETA },
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