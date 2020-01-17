use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use tui::widgets::{Dataset, Marker, Chart, Block, Axis, Widget};
use tui::style::{Style, Color};
use crate::plotting_utils::sample_line;
use std::ops::Range;
use std::cmp::max;

const SAMPLE_RATE: f64 = 0.01;

pub struct UserParams<'a>{
    pub command: &'a str,
    pub target: Option<f64>,
    pub show_regression_line: bool,
    pub show_target_line: bool,
}

pub fn main_chart(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
                  data: &[(f64, f64)],
                  regression: (f64, f64),
                  time_bounds: &Range<f64>,
                  value_bounds: &Range<f64>,
                  user_params: UserParams,
){
    let mut chart_size = frame.size();
    let sampled_regression_line;
    let sampled_target_line;
    let mut datasets_to_draw = vec![
        Dataset::default()
            .name("command result")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(data)
    ];

    if user_params.show_regression_line {
        sampled_regression_line = sample_line(
            regression.0,
            regression.1,
            (time_bounds.start, value_bounds.end),
            (time_bounds.end, value_bounds.end),
            SAMPLE_RATE
        );
        datasets_to_draw.push(Dataset::default()
            .name("regression")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::DarkGray))
            .data(sampled_regression_line.as_slice()));
    }

    if let Some(target) = user_params.target {
        if frame.size().height <= 4 {
            chart_size.height = 0;
        } else {
            chart_size.height = frame.size().height - 4;
        }
        if user_params.show_target_line {
            sampled_target_line = sample_line(
                0.0,
                target,
                (time_bounds.start, value_bounds.end),
                (time_bounds.end, value_bounds.end),
                SAMPLE_RATE,
            );
            datasets_to_draw.push(Dataset::default()
                .name("target")
                .marker(Marker::Braille)
                .style(Style::default().fg(Color::LightGreen))
                .data(sampled_target_line.as_slice()));
        }

    }

    Chart::default()
        .block(Block::default().title(&format!("\"{}\"", user_params.command)))
        .style(Style::default().fg(Color::White))
        .x_axis(Axis::default()
            .title("X Axis")
            .title_style(Style::default().fg(Color::Red))
            .style(Style::default().fg(Color::White))
            .bounds([time_bounds.start, time_bounds.end])
            .labels(&[
                &format!("{:.*}", 2, time_bounds.start),
                &format!("{:.*}", 2, (time_bounds.end - time_bounds.start) / 2.0 + time_bounds.start),
                &format!("{:.*}", 2, time_bounds.end)]))
        .y_axis(Axis::default()
            .title("Y Axis")
            .title_style(Style::default().fg(Color::Red))
            .style(Style::default().fg(Color::White))
            .bounds([value_bounds.start, value_bounds.end])
            .labels(&[
                &format!("{:.*}", 2, value_bounds.end),
                &format!("{:.*}", 2, (value_bounds.end - value_bounds.start) / 2.0 + value_bounds.end),
                &format!("{:.*}", 2, value_bounds.end)]))
        .datasets(&datasets_to_draw)
        .render(frame, chart_size);
}
