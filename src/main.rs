use structopt::StructOpt;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use termion::async_stdin;
use termion::input::TermRead;
use app::UI;

mod app;
mod plotting_utils;
mod widget_progress_bar;
mod widget_main_chart;
mod widget_text_output;

static ONE_BILLION: f32 = 1_000_000_000.0;
const EVENT_TICK: Duration = Duration::from_millis(20);

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "n", long = "refresh-rate", default_value = "1.0")]
    refresh_rate: f32,

    #[structopt(short = "t", long = "target")]
    target_result: Option<f64>,

    #[structopt(long = "show-regression-line")]
    show_regression_line: Option<bool>,

    #[structopt(long = "show-target-line")]
    show_target_line: Option<bool>,

    #[structopt(short = "l", long = "history-len", default_value = "100")]
    history_len: usize,

    command: String,
}

fn main() {
    let args = Cli::from_args();
    let refresh_rate = Duration::from_nanos((args.refresh_rate * ONE_BILLION) as u64);
    let mut ui = UI::new(
        &args.command,
        args.target_result,
        args.history_len,
        args.show_regression_line.or(Some(true)).unwrap(),
        args.show_target_line.or(Some(false)).unwrap()
    );
    let mut keys = async_stdin().keys();
    let mut exit = false;

    while !exit {
        ui.evaluate();
        let mut current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let last_time = current_time;
        while refresh_rate.as_secs_f64() > current_time - last_time {
            current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            if !ui.event_handler(keys.next()) {
                exit = true;
                break
            }
            thread::sleep(EVENT_TICK);
        }
    }
    ui.clean_up_terminal();
}
