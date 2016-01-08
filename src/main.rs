#[macro_use]
extern crate clap;
extern crate rand;
extern crate statistical;

use std::io::{sink, stdout, BufWriter, Write};
use std::fs::File;
use std::thread;
use std::sync::{Arc, Barrier, Mutex};
use clap::{Arg, App, AppSettings};
use rand::distributions::{IndependentSample, Range};
use statistical::{mean, median, standard_deviation};

fn die(ht: &i32, death: &i32, bonus: &i32, increment: &f64, longevity: &bool, self_destruct: &bool) -> (f64, Vec<u8>) {
    let mut ht = *ht;
    let death = *death;
    let bonus = *bonus;
    let increment = *increment;
    let longevity = *longevity;
    let self_destruct = *self_destruct;

    let mut log = Vec::new();
    let mut age: f64 = increment * 50.0;

    let d6 = Range::new(1, 7);
    let mut rng = rand::thread_rng();

    loop {
        let roll = d6.ind_sample(&mut rng) + d6.ind_sample(&mut rng) + d6.ind_sample(&mut rng);
        write!(&mut log, "Current HT is {}, effective HT is {}, current age is {}. Rolled a {}.", ht, ht + bonus, age, roll).unwrap();

        // Check for HT loss.
        if longevity {                                  // Longevity only can fail on a 17 or 18.
            if roll == 18 {                             // Always fail on an 18. Lose 1 HT.
                ht = ht - 1;
                write!(&mut log, " Lost 1 HT.").unwrap();
            } else if roll == 17 && ht + bonus < 17 {   // 17 fails if modified HT is less than 17. Lose 1 HT.
                ht = ht - 1;
                write!(&mut log, " Lost 1 HT.").unwrap();
            }
        } else {                                        // Normal rules.
            if roll > 16 || roll > ht + bonus + 9 {     // Any roll of 17 or 18, or failure by more than 10, loses 2 HT.
                ht = ht - 2;
                write!(&mut log, " Lost 2 HT.").unwrap();
            } else if roll > ht + bonus {               // Normal failure loses 1 HT.
                ht = ht - 1;
                write!(&mut log, " Lost 1 HT.").unwrap();
            }
        }

        // Check for character death.
        if ht <= death {                                // He's dead, Jim.
            write!(&mut log, " Dead at {}.\n\n%\n\n", age).unwrap();
            return (age, log);
        } else {                                        // Age one increment.
            if self_destruct {                          // Characters with Self-Destruct roll daily.
                age = age + 1.0/365.0;
            } else if age < increment * 70.0 {          // Characters in the first bracket age one increment.
                age = age + increment;
            } else if age < increment * 90.0 {          // Characters in the second bracket age half an increment.
                age = age + increment / 2.0;
            } else {                                    // Characters in the third bracket age a quarter-increment.
                age = age + increment / 4.0;
            }
        }
        write!(&mut log, "\n").unwrap();                // Repeat
    };
}

fn main() {
    let matches = App::new("gurps-aging-calculator")
        .version("2.0.0")
        .author("Patrick Burroughs (Celti) <celti@celti.name>")
        .about("Iterative calculator for the GURPS aging rules.")
        .help_short("?")
        .setting(AppSettings::UnifiedHelpMessage)
        .version_short("V")
        .args(vec![
            Arg::with_name("HT").short("h").long("ht").takes_value(true)
                .help("The character's starting HT (default is 10)."),
            Arg::with_name("TL").short("t").long("tl").takes_value(true)
                .help("The character's lifetime medical TL (default is 8)."),
            Arg::with_name("ADD").short("a").long("add").takes_value(true)
                .help("Total additional optional modifiers to the aging roll."),
            Arg::with_name("DEATH").short("d").long("death").takes_value(true)
                .help("The HT the character is considered dead at (default is 4)."),
            Arg::with_name("LONGEVITY").short("l").long("longevity")
                .help("The character has Longevity."),
            Arg::with_name("SELF_DESTRUCT").short("D").long("self-destruct")
                .help("The character has Self-Destruct."),
            Arg::with_name("EXTENDED").short("x").long("extended-lifespan").takes_value(true)
                .conflicts_with("SHORT")
                .help("The character's level of Extended Lifespan (default is 0)."),
            Arg::with_name("SHORT").short("s").long("short-lifespan").takes_value(true)
                .conflicts_with("EXTENDED")
                .help("The character's level of Short Lifespan (default is 0)."),
            Arg::with_name("MAX_PROCS").short("p").long("max-procs").takes_value(true)
                .help("The maximum number of threads to spawn (default is 4)."),
            Arg::with_name("ITERATIONS").short("i").long("iterations").takes_value(true)
                .help("The number of iterations to calculate (default is 100,000)."),
            Arg::with_name("VERBOSE").short("v").long("verbose").takes_value(true)
                .help("Log verbose output to specified file ('-' indicates stdout)."),
        ]).get_matches();

    let verbose = matches.value_of("VERBOSE");
    let iterations = value_t!(matches.value_of("ITERATIONS"), usize).unwrap_or(100_000);
    let max_procs  = value_t!(matches.value_of("MAX_PROCS"),  usize).unwrap_or(4);

    let extended = value_t!(matches.value_of("EXTENDED"), f64).unwrap_or(0.0);
    let short    = value_t!(matches.value_of("SHORT"),    f64).unwrap_or(0.0);

    let ht    = value_t!(matches.value_of("HT"),    i32).unwrap_or(10);
    let tl    = value_t!(matches.value_of("TL"),    i32).unwrap_or(8);
    let add   = value_t!(matches.value_of("ADD"),   i32).unwrap_or(0);
    let death = value_t!(matches.value_of("DEATH"), i32).unwrap_or(4);

    let longevity     = matches.is_present("LONGEVITY");
    let self_destruct = matches.is_present("SELF_DESTRUCT");

    let multiplier = 2.0f64;
    let increment  = multiplier.powf(extended - short);
    let bonus      = tl - 3 + add - if self_destruct { 3 } else { 0 };

    let iter_per_thread: usize = iterations / max_procs;


    // Get to work.
    let barrier = Arc::new(Barrier::new(max_procs + 1));
    let deaths: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));

    // This works.
    //let output = Arc::new(stdout());

    let output = Arc::new(Mutex::new( match verbose {
        None       => Box::new(BufWriter::new(sink())) as Box<Write + Send>,
        Some("-")  => Box::new(BufWriter::new(stdout())) as Box<Write + Send>,
        Some(file) => Box::new(BufWriter::new(File::create(file).expect("Couldn't open file for writing!"))) as Box<Write + Send>
    }));

    for _ in 0..max_procs {
        let (deaths, barrier, output) = (deaths.clone(), barrier.clone(), output.clone());
        thread::spawn(move || {
            for _ in 0..iter_per_thread {
                let (age, log) = die(&ht, &death, &bonus, &increment, &longevity, &self_destruct);

                let mut deaths = deaths.lock().unwrap();
                deaths.push(age);

                let mut output = output.lock().unwrap();
                output.write(&log[..]).unwrap();
            }
            barrier.wait();
        });
    }

    barrier.wait();

    let mut deaths = deaths.lock().unwrap();
    deaths.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let max = deaths.last().unwrap();
    let min = deaths.first().unwrap();
    let mean = mean(&deaths[..]);
    let median = median(&deaths[..]);
    let stddev = standard_deviation(&deaths[..], None);

    println!("Median age of death is {} (highest is {}, lowest is {}).", median, max, min);
    println!("Mean is {}; StdDev is {}.", mean, stddev);
}
