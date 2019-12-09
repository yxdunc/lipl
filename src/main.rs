use structopt::StructOpt;
use std::time::{SystemTime, UNIX_EPOCH};
use termion::async_stdin;
use termion::input::TermRead;
use app::UI;
use core::time;


mod app;

static ONE_BILLION: f32 = 1000000000.0;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "n", long = "refresh-rate", default_value = "1.0")]
    refresh_rate: f32,

    #[structopt(short = "t", long = "target")]
    target_result: Option<f64>,

    #[structopt(short = "l", long = "history-len", default_value = "100")]
    history_len: usize,

    command: String,
}

fn main() {
    let args = Cli::from_args();
    let mut last_time = 0.0;
    let refresh_rate = time::Duration::from_nanos((args.refresh_rate * ONE_BILLION) as u64);
    let mut ui = UI::new(&args.command, args.target_result, args.history_len);
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