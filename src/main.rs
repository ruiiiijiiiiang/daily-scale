use chrono::{Datelike, Utc};
use clap::Parser;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};

use daily_scale::lib::fret_board::{build_fret_board, FRET_SPAN, NUM_FRETS};
use daily_scale::lib::notes::{
    accidental_to_note, note_to_string, Accidental, Note, FLAT_ACCIDENTALS, NOTES, NUM_NOTES,
};
use daily_scale::lib::scales::{get_steps_by_scale, scale_to_string, Scale, SCALES};
use daily_scale::lib::tunings::{tuning_to_string, Tuning};

#[derive(Parser, Debug)]
#[command(name = "daily-scale", version, about = "Have you practiced today?", long_about = None)]
struct Args {
    #[arg(
        value_enum,
        required = false,
        short = 't',
        long,
        default_value = "standard-e6",
        help = "Your choice of tuning"
    )]
    tuning: Option<Tuning>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 'n',
        long,
        help = "Pool of root notes of the scale"
    )]
    root_notes: Option<Vec<Accidental>>,

    #[arg(
        value_enum,
        value_delimiter = ',',
        required = false,
        short = 's',
        long,
        help = "Pool of scales"
    )]
    scales: Option<Vec<Scale>>,

    #[arg(
        value_parser = value_parser!(usize),
        value_delimiter = ',',
        required = false,
        short = 'f',
        long,
        value_parser = |s: &str| {
            let num = s.parse::<usize>().map_err(|_| "Not a valid number")?;
            if num <= (NUM_FRETS - FRET_SPAN + 1) {
                Ok(num)
            } else {
                Err(format!("Number must be <= {}", NUM_FRETS - FRET_SPAN + 1))
            }
        },
        help = "Pool of frets to start the scale on"
    )]
    starting_frets: Option<Vec<usize>>,

    #[arg(
        required = false,
        short = 'r',
        long,
        help = "If true, the rng will no longer use today's date as seed"
    )]
    full_randomness: bool,
}

fn main() {
    let Args {
        tuning,
        root_notes,
        scales,
        starting_frets,
        full_randomness,
        ..
    } = Args::parse();

    let mut rng: Box<dyn RngCore> = if full_randomness {
        Box::new(rand::thread_rng())
    } else {
        let seed = Utc::now().date_naive().num_days_from_ce() as u64;
        Box::new(StdRng::seed_from_u64(seed))
    };

    let tuning = tuning.unwrap();

    let mut flat = false;
    let root_note = if let Some(ref arg_notes) = root_notes {
        let arg_note = arg_notes.choose(&mut rng).unwrap();
        if FLAT_ACCIDENTALS.contains(arg_note) {
            flat = true
        }
        &(accidental_to_note(arg_note))
    } else {
        NOTES.choose(&mut rng).unwrap()
    };

    let scale = if let Some(ref arg_scales) = scales {
        arg_scales.choose(&mut rng).unwrap()
    } else {
        SCALES.choose(&mut rng).unwrap()
    };

    let all_frets: Vec<usize> = (0..=NUM_FRETS - FRET_SPAN).collect();
    let starting_fret = if let Some(ref arg_frets) = starting_frets {
        arg_frets.choose(&mut rng).unwrap()
    } else {
        all_frets.choose(&mut rng).unwrap()
    };

    let root_note_index = NOTES.iter().position(|&note| note == *root_note).unwrap();
    let steps = get_steps_by_scale(*scale);
    let notes_in_scale = steps
        .iter()
        .map(|step| {
            let note_index = (root_note_index + step) % NUM_NOTES;
            NOTES[note_index]
        })
        .collect::<Vec<Note>>();

    let fret_board: Vec<String> = build_fret_board(&tuning, *starting_fret, &notes_in_scale, flat);

    for string in fret_board {
        println!("{}", string);
    }

    println!(
        "Here's the scale of the day: {} {} starting at fret {} in {} tuning",
        note_to_string(*root_note, flat),
        scale_to_string(*scale),
        starting_fret,
        tuning_to_string(tuning),
    );

    println!(
        "The notes in this scale are: {}",
        notes_in_scale
            .iter()
            .map(|note| note_to_string(*note, flat))
            .collect::<Vec<&str>>()
            .join(", ")
    );
}
