use log::{info, LevelFilter};
use parking_lot::Mutex;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::UnifiedHelpMessage"), rename_all = "kebab-case")]
struct Opt {
    /// The character's starting HT.
    #[structopt(short, long, default_value = "10")]
    ht: u8,

    /// The character's lifetime medical TL.
    #[structopt(short, long, default_value = "8")]
    tl: u8,

    /// Optional modifiers to the aging roll (if any).
    #[structopt(short, long, default_value = "0")]
    add: i8,

    /// The HT the character is considered dead at.
    #[structopt(short, long, default_value = "4")]
    death: u8,

    /// Indicates the character has Longevity.
    #[structopt(short, long)]
    longevity: bool,

    /// Indicates the character has Self-Destruct.
    #[structopt(short = "D", long)]
    self_destruct: bool,

    /// The character's level of Extended Lifespan (if any).
    #[structopt(short = "x", long, default_value = "0")]
    extended_lifespan: f64,

    /// The character's level of Short Lifespan (if any).
    #[structopt(short, long, default_value = "0")]
    short_lifespan: f64,

    /// The number of iterations to calculate.
    #[structopt(short, long, default_value = "100000")]
    iterations: usize,

    /// Log verbose output.
    #[structopt(short, long)]
    verbose: bool,

    /// Log verbose output to specified file.
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn die(ht: u8, death: u8, bonus: i8, increment: f64, longevity: bool, self_destruct: bool) -> f64 {
    let mut ht = ht as i8;
    let mut age: f64 = increment * 50.0;

    let choices = [3, 4, 5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18];
    let weights = [1, 3, 6, 10, 15, 21, 25, 27, 27, 25, 21, 15, 10,  6,  3,  1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();
    let mut roll_3d6 = || choices[dist.sample(&mut rng)];

    loop {
        let roll = roll_3d6();

        // Check for HT loss.
        let loss = {
            if longevity && (roll == 18 || (roll == 17 && ht + bonus < 17)) { // With Longevity, any roll of 18, or a 17
                1                                                             // if modified HT is less than 17, loses 1 HT.
            } else if roll > 16 || roll > ht + bonus + 9 {                    // Otherwise, any roll of 17 or 18, or failure
                2                                                             // by more than 10, loses 2 HT.
            } else if roll > ht + bonus {                                     // Any ordinary failure...
                1                                                             // ...loses 1 HT.
            } else {                                                          // Nothing happens on a success.
                0
            }
        };

        ht -= loss;

        info!("Current HT is {}, effective HT is {}, current age is {}. Rolled a {}. Lost {} HT.",
              ht, ht + bonus, age, roll, loss);

        // Check for character death.
        if ht <= death as i8 {                                // They're dead, Jim.
            info!(" Dead at {}.", age);
            return age;
        } else {                                        // Age one increment.
            if self_destruct {                          // Characters with Self-Destruct roll daily.
                age += 1.0/365.0;
            } else if age < increment * 70.0 {          // Characters in the first bracket age one increment.
                age += increment;
            } else if age < increment * 90.0 {          // Characters in the second bracket age half an increment.
                age += increment / 2.0;
            } else {                                    // Characters in the third bracket age a quarter-increment.
                age += increment / 4.0;
            }
        }
    };
}

fn main() {
    let opt = Opt::from_args();

    let multiplier = 2.0f64;
    let increment  = multiplier.powf(opt.extended_lifespan - opt.short_lifespan);
    let bonus      = opt.add + opt.tl as i8 - 3 - if opt.self_destruct { 3 } else { 0 };

    // Get to work.
    let deaths: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));

    if opt.verbose {
        if let Some(ref file) = opt.output {
            simple_logging::log_to_file(file, LevelFilter::Info)
                .expect("Could not open file for writing!");
        } else {
            simple_logging::log_to_stderr(LevelFilter::Info);
        }
    }

    (0..opt.iterations).into_par_iter().for_each(|_| {
        let age = die(opt.ht, opt.death, bonus, increment, opt.longevity, opt.self_destruct);
        deaths.lock().push(age);
    });

    let mut deaths = deaths.lock();
    deaths.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let max = deaths.last().unwrap();
    let min = deaths.first().unwrap();
    let mean = statistical::mean(&deaths[..]);
    let median = statistical::median(&deaths[..]);
    let stddev = statistical::standard_deviation(&deaths[..], None);

    println!("Median age of death is {} (highest is {}, lowest is {}).", median, max, min);
    println!("Mean is {}; StdDev is {}.", mean, stddev);
}
