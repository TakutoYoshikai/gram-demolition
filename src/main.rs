use ofiles::opath;
use std::thread;
extern crate nix;

use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};
use std::time;
use getopts::Options;
use std::process;
use std::env;

fn run(file_path: &str, interval_ms: u64) {
    loop {
        let pids = opath(file_path).unwrap();
        for pid in pids {
            signal::kill(Pid::from_raw(u32::from(pid) as i32), Signal::SIGTERM).unwrap();
        }
        let interval = time::Duration::from_millis(interval_ms);
        thread::sleep(interval);
    }
}

struct Args {
    file: String,
    interval: u64,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "help");
    opts.optopt("f", "file", "file to observe", "FILE");
    opts.optopt("i", "interval", "interval to observe(ms)", "INTERVAL");
    if args.len() == 1 {
        print_usage(&program, &opts);
    }
    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| panic!(f.to_string()));
    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }
    if matches.opt_str("f") == None {
        print_usage(&program, &opts);
    }
    let file: String = matches.opt_str("f").unwrap();
    let interval_str: Option<String> = matches.opt_str("i");
    let mut interval_ms: u64 = 300;
    if interval_str != None {
        interval_ms = interval_str.unwrap().parse::<u64>().unwrap();
    }
    let result: Args = Args {
        file: file,
        interval: interval_ms,
    };
    return result;
}
fn main() {
    let args = parse_args();
    run(&args.file, args.interval);
}
