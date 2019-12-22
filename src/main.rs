use app::UI;
use core::time;
use std::time::Instant;
use structopt::StructOpt;
use termion::async_stdin;
use termion::input::TermRead;

mod app;
mod plotting_utils;
mod widget_main_chart;
mod widget_progress_bar;
mod widget_text_output;

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
    let refresh_rate = time::Duration::from_secs_f32(args.refresh_rate);
    let mut ui = UI::new(
        &args.command,
        args.target_result,
        args.history_len,
        args.show_regression_line.or(Some(true)).unwrap(),
        args.show_target_line.or(Some(false)).unwrap(),
    );
    let mut keys = async_stdin().keys();
    // Substract refresh_rate to display the first frame immediatly
    let mut last_time = Instant::now() - refresh_rate;

    loop {
        let current_time = Instant::now();
        if refresh_rate < current_time - last_time {
            ui.evaluate();
            last_time = current_time;
        }
        if !ui.event_handler(keys.next()) {
            break;
        }
    }

    ui.clean_up_terminal();
}
