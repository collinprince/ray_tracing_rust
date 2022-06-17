use crate::threaded::ThreadParameters;
use std::env;

pub fn get_thread_parameters() -> ThreadParameters {
    let args: Vec<String> = env::args().collect();
    // remove flags from the argument list
    let args: Vec<String> = args
        .into_iter()
        .filter(|x| {
            if x.len() > 1 {
                if (&x[0..2]) == "--" {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect();

    if args.len() == 1 {
        // no args provided
        ThreadParameters {
            num_threads: 1,
            lines_per_thread: 1,
        }
    } else if args.len() == 2 {
        // num_threads provided
        // use 4 lines per thread by default
        eprintln!("arg1: {}", args[1]);
        let num_threads: usize = args[1].parse().unwrap();
        ThreadParameters {
            num_threads,
            lines_per_thread: 4,
        }
    } else {
        let num_threads: usize = args[1].parse().unwrap();
        let lines_per_thread: usize = args[2].parse().unwrap();
        ThreadParameters {
            num_threads,
            lines_per_thread,
        }
    }
}
