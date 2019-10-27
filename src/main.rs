use structopt::StructOpt;
use cmd_lib::run_cmd;
use std::{thread, time};

static DEFAULT_REFRESH_RATE: f32 = 1.0;
static ONE_BILLION: f32 = 1000000000.0;

#[derive(StructOpt)]
struct Cli {
    refresh_rate: f32,
    command: String,
}

fn main() {
    let args = Cli::from_args();
    let refresh_rate = time::Duration::from_nanos((args.refresh_rate * ONE_BILLION) as u64);

    loop{
        run_cmd!("{}", args.command).unwrap();
        thread::sleep(refresh_rate);
    }
}
