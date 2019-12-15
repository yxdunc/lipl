use std::io;
use cmd_lib::run_fun;
use termion::raw::{RawTerminal, IntoRawMode};
use tui::backend::TermionBackend;
use termion::event::Key;
use tui::Terminal;
use std::io::Stdout;
use std::time::{SystemTime, UNIX_EPOCH};
use linreg::linear_regression_of;

use crate::widget_text_output::text_output;
use crate::widget_progress_bar::progress_bar;
use crate::widget_main_chart::main_chart;

pub struct UI<'a> {
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    start_time: f64,
    command: & 'a str,
    target: Option<f64>,
    history_len: usize,
    cmd_result_history: Vec<(f64, f64)>,
    cmd_txt_result_history: Vec<(f64, String)>,
    show_regression_line: bool,
    show_target_line: bool,
}

impl <'a> UI <'a> {
    pub fn new(command: & 'a str, target: Option<f64>, history_len: usize, show_regression_line: bool, show_target_line: bool) -> Self {
        let stdout= io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        let _ = terminal.clear();
        let _ = terminal.hide_cursor();

        UI {
            terminal,
            start_time,
            command,
            target,
            history_len,
            cmd_result_history: [].to_vec(),
            cmd_txt_result_history: [].to_vec(),
            show_regression_line,
            show_target_line,
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

    fn draw(&mut self) {
        if self.cmd_result_history.is_empty() {
            let cmd_txt_result_history = &self.cmd_txt_result_history;
            self.terminal.draw(|mut f| {
                text_output(&mut f, cmd_txt_result_history);
            }).unwrap();
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
        let show_regression_line = self.show_regression_line;
        let show_target_line = self.show_target_line;

        self.terminal.draw(|mut f| {
            main_chart(&mut f, regression, min_time, max_time, min_value, max_value, command, data, target, show_regression_line, show_target_line);
            progress_bar(&mut f, regression, min_time, max_time, target);
        }).unwrap();
    }

    pub fn clean_up_terminal(&mut self) {
        let _ = self.terminal.flush();
        let _ = self.terminal.clear();
        let _ = self.terminal.show_cursor();
    }

    pub fn evaluate(&mut self) {
        let result = run_fun!("{}", self.command);

        result.and_then(|content| Ok(self.result_handler(content))).unwrap();
    }

    pub fn result_handler(&mut self, result: String) {
        let number = result.trim().parse::<f64>();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        match number {
            Ok(value) => {
                self.append_result(current_time - self.start_time, value);
            }
            Err(_error) => {
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
}