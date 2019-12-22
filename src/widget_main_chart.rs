use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use tui::widgets::{Dataset, Marker, Chart, Block, Axis, Widget};
use tui::style::{Style, Color};
use crate::plotting_utils::sample_line;

const SAMPLE_RATE: f64 = 0.01;

pub fn main_chart(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
                  regression: (f64, f64),
                  min_time: f64,
                  max_time: f64,
                  min_value: f64,
                  max_value: f64,
                  command: &str,
                  data: &[(f64, f64)],
                  target: Option<f64>,
                  show_regression_line: bool,
                  show_target_line: bool){
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

    if show_regression_line {
        sampled_regression_line = sample_line(
            regression.0,
            regression.1,
            (min_time, min_value),
            (max_time, max_value),
            SAMPLE_RATE
        );
        datasets_to_draw.push(Dataset::default()
            .name("regression")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::DarkGray))
            .data(sampled_regression_line.as_slice()));
    }

    if target.is_some() {
        chart_size.height = frame.size().height - frame.size().height / 10;
        if show_target_line {
            sampled_target_line = sample_line(
                0.0,
                target.unwrap(),
                (min_time, min_value),
                (max_time, max_value),
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
        .block(Block::default().title(&format!("\"{}\"", command)))
        .style(Style::default().fg(Color::White))
        .x_axis(Axis::default()
            .title("X Axis")
            .title_style(Style::default().fg(Color::Red))
            .style(Style::default().fg(Color::White))
            .bounds([min_time, max_time])
            .labels(&[
                &format!("{:.*}", 2, min_time),
                &format!("{:.*}", 2, (max_time - min_time) / 2.0 + min_time),
                &format!("{:.*}", 2, max_time)]))
        .y_axis(Axis::default()
            .title("Y Axis")
            .title_style(Style::default().fg(Color::Red))
            .style(Style::default().fg(Color::White))
            .bounds([min_value, max_value])
            .labels(&[
                &format!("{:.*}", 2, min_value),
                &format!("{:.*}", 2, (max_value - min_value) / 2.0 + min_value),
                &format!("{:.*}", 2, max_value)]))
        .datasets(&datasets_to_draw)
        .render(frame, chart_size);
}