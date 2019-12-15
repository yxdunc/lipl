use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use tui::widgets::{Dataset, Marker, Chart, Block, Axis, Widget};
use tui::style::{Style, Color};

fn sample_line(a: f64, b: f64, min: (f64, f64), max: (f64, f64), sample_rate: f64) -> Vec<(f64, f64)> {
    let mut x = min.0;
    let mut sampled_line: Vec<(f64, f64)> = vec![];

    while x < max.0 {
        x += sample_rate;
        sampled_line.push((x, a * x + b));
    }

    sampled_line
}

pub fn main_chart(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
              regression: (f64, f64),
              min_time: f64,
              max_time: f64,
              min_value: f64,
              max_value: f64,
              command: &str,
              data: &[(f64, f64)],
              target: Option<f64>,
              hide_regression_line: bool){
    let mut chart_size = frame.size();
    let mut sampled_line: Vec<(f64, f64)> = [].to_vec();
    let mut datasets_to_draw = vec![
        Dataset::default()
            .name("command result")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(data)
    ];
    if !hide_regression_line {
        sampled_line = sample_line(
            regression.0,
            regression.1,
            (min_time, min_value),
            (max_time, max_value),
            0.01
        );
        datasets_to_draw.push(Dataset::default()
            .name("regression")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::LightGreen))
            .data(sampled_line.as_slice()))
    }

    if target.is_some() {
        chart_size.height = frame.size().height - frame.size().height / 10;

    }

    Chart::default()
        .block(Block::default().title(&format!("\"{}\"", command)))
        .style(Style::default().fg(Color::White).bg(Color::Black))
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