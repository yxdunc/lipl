use tui::widgets::{Chart, Block, Axis, Dataset, Marker, Widget, Gauge, Borders, Text, Paragraph};
use std::io;
use cmd_lib::run_fun;
use tui::style::{Style, Color, Modifier};
use termion::raw::{RawTerminal, IntoRawMode};
use tui::backend::TermionBackend;
use termion::event::Key;
use tui::{Terminal, Frame};
use std::io::Stdout;
use std::time::{SystemTime, UNIX_EPOCH};
use linreg::linear_regression_of;
use std::f64::INFINITY;
use std::borrow::{Cow, Borrow};
use tui::layout::Alignment;

pub struct UI<'a> {
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    start_time: f64,
    command: & 'a str,
    target: Option<f64>,
    history_len: usize,
    cmd_result_history: Vec<(f64, f64)>,
    cmd_txt_result_history: Vec<(f64, String)>,
    hide_regression_line: bool,
}

fn sample_line(a: f64, b: f64, min: (f64, f64), max: (f64, f64), sample_rate: f64) -> Vec<(f64, f64)> {
    let mut x = min.0;
    let mut sampled_line: Vec<(f64, f64)> = vec![];

    while x < max.0 {
        x += sample_rate;
        sampled_line.push((x, a * x + b));
    }

    sampled_line
}

impl <'a> UI <'a> {
    pub fn new(command: & 'a str, target: Option<f64>, history_len: usize, hide_regression_line: bool) -> Self {
        let stdout= io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        terminal.clear();
        terminal.hide_cursor();

        UI {
            terminal,
            start_time,
            command,
            target,
            history_len,
            cmd_result_history: [].to_vec(),
            cmd_txt_result_history: [].to_vec(),
            hide_regression_line,
        }
    }

    fn append_result(&mut self, time: f64, result: f64) {
        self.cmd_result_history.push((time, result));
        if self.cmd_result_history.len() > self.history_len {
            self.cmd_result_history.remove(0);
        }
    }

    fn append_txt_result(&mut self, time: f64, result: String) {
        self.cmd_txt_result_history.push((time, result));

        if self.cmd_txt_result_history.len() > self.history_len {
            self.cmd_txt_result_history.remove(0);
        }
    }

    fn progress_bar(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
                    regression: (f64, f64),
                    min_time: f64,
                    max_time: f64,
                    target: Option<f64>) {
        if let Some(target) = target {
            let ETA: f64 = (target - regression.1) / regression.0;
            let remaining_time = ETA - min_time;
            let completion_percentage = (max_time / ETA * 100.0) as i16;
            let gauge_title= &format!(
                "Estimated time of completion: {:.*}s (remaining: {:.*}s)",
                2, if ETA < 0.0 { INFINITY } else { ETA },
                2, if ETA < 0.0 { INFINITY } else { (ETA - max_time).max(0.0) }
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

    fn text_output(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, text_result_history: &Vec<(f64, String)>) {
        if !text_result_history.is_empty() {
            let last_result = text_result_history.last().unwrap();
            Paragraph::new(vec!(Text::Raw(Cow::from(&last_result.1))).iter())
                .block(Block::default().title(&format!("Elapsed time: {:.*} seconds", 2, last_result.0)))
                .alignment(Alignment::Left)
                .render(frame, frame.size());
        }
    }

    fn main_chart(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>,
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

    fn draw(&mut self) {
        if self.cmd_result_history.is_empty() {
            let cmd_txt_result_history = &self.cmd_txt_result_history;
            self.terminal.draw(|mut f| {
                UI::text_output(&mut f, cmd_txt_result_history);
            });
            return ;
        }
        let data = self.cmd_result_history.as_slice();
        let min_time = data.iter().min_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
        let max_time = data.iter().max_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
        let min_value = data.iter().min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap().1 - 10.0;
        let max_value = data.iter().max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap().1 + 10.0;
        let command = self.command;
        let regression: (f64, f64) = linear_regression_of(data).or(Some((0.0, 0.0))).unwrap();
        let target = self.target;
        let hide_regression_line = self.hide_regression_line;

        self.terminal.draw(|mut f| {
            UI::main_chart(&mut f, regression, min_time, max_time, min_value, max_value, command, data, target, hide_regression_line);
            UI::progress_bar(&mut f, regression, min_time, max_time, target);
        });
    }

    pub fn clean_up_terminal(&mut self) {
        self.terminal.flush();
        self.terminal.clear();
        self.terminal.show_cursor();
    }

    fn result_handler(&mut self, result: String) {
        let number = result.trim().parse::<f64>();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match number {
            Ok(value) => {
                self.append_result(current_time - self.start_time, value);
            }
            Err(error) => {
                self.append_txt_result(current_time - self.start_time, result);
            }
        }

        self.draw();
    }
    pub fn event_handler(&mut self, key: Option<Result<Key, std::io::Error>>) -> bool {
        if let Some(k) = key {
            match k {
                // Exit.
                Ok(Key::Char('q')) => return false,
                Ok(Key::Ctrl('c')) => return false,
                Ok(Key::Ctrl('d')) => return false,
                Ok(Key::Esc) => return false,
                _ => (),
            }
        }
        return true;
    }

    pub fn evaluate(&mut self) {
        let result = run_fun!("{}", self.command);

        result.and_then(|content| Ok(self.result_handler(content)));
    }
}