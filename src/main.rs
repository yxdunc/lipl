use structopt::StructOpt;
use std::{thread, time, io};
use tui::widgets::{Chart, Block, Axis, Dataset, Marker, Widget, List};

use cmd_lib::{run_fun, FunResult};
use tui::style::{Style, Color};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::Terminal;
use std::io::Stdout;
use tui::widgets::canvas::Points;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static DEFAULT_REFRESH_RATE: f32 = 1.0;
static ONE_BILLION: f32 = 1000000000.0;

#[derive(StructOpt)]
struct Cli {
    refresh_rate: f32,
    command: String,
}

struct UI {
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    start_time: f64,
    cmd_result_history: Vec<(f64, f64)>,
}

impl UI {
    fn new() -> UI {
        let stdout= io::stdout().into_raw_mode().unwrap();
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        UI {
            terminal,
            start_time,
            cmd_result_history: [].to_vec()
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

        self.terminal.draw(|mut f| {
            let size = f.size();

            Chart::default()
                .block(Block::default().title("Chart"))
                .x_axis(Axis::default()
                    .title("X Axis")
                    .title_style(Style::default().fg(Color::Red))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, max_time])
                    .labels(&["0.0", "5.0", "10.0"]))
                .y_axis(Axis::default()
                    .title("Y Axis")
                    .title_style(Style::default().fg(Color::Red))
                    .style(Style::default().fg(Color::White))
                    .bounds([min_value, max_value])
                    .labels(&["0.0", "5.0", "10.0"]))
                .datasets(&[Dataset::default()
                    .name("data1")
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(data)])
                .render(&mut f, size);
        });
    }

    fn result_handler(&mut self, result: String) {
        let number = result.trim().parse::<f64>();

        match number {
            Ok(value) => {
                let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                self.append_result(current_time - self.start_time, value)
            }
            Err(error) => { println!("NaN error: {}", error); }
        }

//        println!("{:?}", self.cmd_result_history);
    }
}

fn main() {
    let args = Cli::from_args();
    let refresh_rate = time::Duration::from_nanos((args.refresh_rate * ONE_BILLION) as u64);
    let mut ui = UI::new();

    loop{
        let result = run_fun!("{}", args.command);
        match result {
            Ok(content) => { ui.result_handler(content); }
            Err(error) => { println!("test error: {}", error); }
        }
        thread::sleep(refresh_rate);
    }  
}