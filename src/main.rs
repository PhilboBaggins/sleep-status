use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use humantime::parse_duration;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("duration")
                .help("The duration to sleep for")
                .required(true)
                .action(clap::ArgAction::Set),
        )
        .get_matches();

    let duration_str: &String = matches.get_one("duration").unwrap();

    match parse_duration(duration_str) {
        Ok(duration) => {
            do_sleep(duration);
        }
        Err(err) => {
            eprintln!("Invalid duration format: {}", err);
            std::process::exit(1);
        }
    }
}

fn do_sleep(duration: Duration) {
    let timer = Instant::now();

    let pb = ProgressBar::new(duration.as_secs());
    pb.set_style(
        ProgressStyle::default_bar()
            .template("Sleeping {bar} [{elapsed_precise} of {duration_precise}]")
            .unwrap()
            .progress_chars("=> "),
    );

    while timer.elapsed() < duration {
        pb.set_position(timer.elapsed().as_secs());
        thread::sleep(Duration::from_secs(1));
    }

    pb.finish_and_clear();
}
