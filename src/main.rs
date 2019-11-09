use structopt::StructOpt;
use std::{thread, time};

use cmd_lib::run_cmd;

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
        let result = run_cmd!("{}", args.command);
        match result {
            Ok(content) => { println!("result: {:?}", content); }
            Err(error) => { println!("test error: {}", error); }
        }
        thread::sleep(refresh_rate);
    }  
}