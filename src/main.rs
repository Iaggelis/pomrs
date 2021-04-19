use std::{
    env,
    io::{self, Write},
    thread, time,
};

fn divmod(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

enum TickMode {
    Work,
    Rest,
}

fn tick(duration: i32, mode: TickMode) {
    let work_mins: i32 = duration * 60;
    let dt: time::Duration = time::Duration::from_secs(1);
    let mode_text = match mode {
        TickMode::Work => "Work",
        TickMode::Rest => "Rest",
    };

    (0..1 + work_mins).rev().for_each(|remaining| {
        let (hours, rem) = divmod(remaining, 3600);
        let (mins, secs) = divmod(rem, 60);
        print!("\r");
        print!(
            "{} time left: {}:{:0>2}:{:0>2}",
            mode_text, hours, mins, secs
        );
        io::stdout().flush().unwrap();
        thread::sleep(dt);
    });
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("./pomrs <work_mins> <rest_mins> <cycles>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            print_usage();
        }
        4 => {
            let work_mins: i32 = args[1].parse::<i32>().unwrap();
            let rest_mins: i32 = args[2].parse::<i32>().unwrap();
            let cycles: i32 = args[3].parse::<i32>().unwrap();
            println!(
                "Starting pomodoro session with: {} cycles of {} working mins and {} rest mins!",
                cycles, work_mins, rest_mins
            );
            (1..cycles + 1).for_each(|c| {
                println!("Time to work!");
                tick(work_mins, TickMode::Work);
                println!("Time to rest!");
                tick(rest_mins, TickMode::Rest);
                println!("Cycle {} done!", c);
            });
        }
        _ => {
            print_usage();
        }
    }
}
