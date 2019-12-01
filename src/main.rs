use structopt::StructOpt;
use std::{thread, time, io};
use tui::widgets::{Chart, Block, Axis, Dataset, Marker, Widget};

use cmd_lib::run_fun;
use tui::style::{Style, Color};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use termion::event::Key;
use tui::Terminal;
use std::io::{Stdout, Stdin};
use std::time::{SystemTime, UNIX_EPOCH};
use std::borrow::{Borrow, BorrowMut};
use termion::input::{TermRead, Keys};
use std::ops::Deref;
use std::rc::Rc;
use termion::{async_stdin, AsyncReader};
use std::fmt::Error;

static DEFAULT_REFRESH_RATE: f32 = 1.0;
static ONE_BILLION: f32 = 1000000000.0;

#[derive(StructOpt)]
struct Cli {
    refresh_rate: f32,
    command: String,
}

struct UI<'a> {
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    start_time: f64,
    command: & 'a str,
    cmd_result_history: Vec<(f64, f64)>,
}

impl <'a> UI <'a> {
    fn new(command: & 'a str) -> Self {
        let stdout= io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        terminal.clear();

        UI {
            terminal,
            start_time,
            command,
            cmd_result_history: [].to_vec(),
        }
    }
    fn append_result(&mut self, time: f64, result: f64) {
        self.cmd_result_history.push((time, result));
        self.draw();
    }

    fn draw(&mut self) {
        let data = self.cmd_result_history.as_slice();
        let max_time = data.iter().max_by(|x, y| x.0.partial_cmp(&y.0).unwrap()).unwrap().0;
        let min_value = data.iter().min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap().1 - 10.0;
        let max_value = data.iter().max_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap().1 + 10.0;
        let command = self.command;

        self.terminal.draw(|mut f| {
            let size = f.size();

            Chart::default()
                .block(Block::default().title(&format!("\"{}\"", command)))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .x_axis(Axis::default()
                    .title("X Axis")
                    .title_style(Style::default().fg(Color::Red))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, max_time])
                    .labels(&[
                        "0.0",
                        &format!("{:.*}", 2, max_time / 2.0),
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
                .datasets(&[Dataset::default()
                    .name("data1")
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(data)])
                .render(&mut f, size);
        });
    }

    fn clean_up_terminal(&mut self) {
        self.terminal.flush();
        self.terminal.clear();
    }

    fn result_handler(&mut self, result: String) {
        let number = result.trim().parse::<f64>();

        match number {
            Ok(value) => {
                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                self.append_result(current_time - self.start_time, value)
            }
            Err(error) => { println!("NaN error: {} [{}]", error, result); }
        }
    }
    fn event_handler(&mut self, key: Option<Result<Key, std::io::Error>>) -> bool {
        if let Some(k) = key {
            match k {
                // Exit.
                Ok(Key::Char('q')) => return false,
                _ => println!("Other"),
            }
        }
        return true;
    }

    fn evaluate(&mut self) {
        let result = run_fun!("{}", self.command);

        result.and_then(|content| Ok(self.result_handler(content)));
    }
}

fn main() {
    let args = Cli::from_args();
    let mut last_time = 0.0;
    let refresh_rate = time::Duration::from_nanos((args.refresh_rate * ONE_BILLION) as u64);
    let mut ui = UI::new(&args.command);
    let mut keys = async_stdin().keys();

    loop{
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        if refresh_rate.as_secs_f64() < current_time - last_time {
            ui.evaluate();
            last_time = current_time;
        }
        if !ui.event_handler(keys.next()) {
            ui.clean_up_terminal();
            break
        }
    }  
}