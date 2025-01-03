use super::notes::{note_to_string, Note, NOTES, NUM_NOTES};
use super::tunings::{get_notes_by_tuning, Tuning};

pub const NUM_FRETS: usize = 24;

pub const FRET_SPAN: usize = 5;

pub fn build_fret_board(
    tuning: &Tuning,
    starting_fret: usize,
    notes_in_scale: &[Note],
    flat: bool,
) -> Vec<String> {
    let mut fret_board = Vec::new();
    let notes_in_tuning = get_notes_by_tuning(*tuning);
    for (string_counter, string) in notes_in_tuning.iter().enumerate() {
        let string_char = if string_counter < (notes_in_tuning.len() - NUM_THICK_STRINGS) {
            '='
        } else {
            '-'
        };
        let fret_board_string =
            build_fret_board_string(starting_fret, notes_in_scale, string, string_char, flat);
        fret_board.insert(0, fret_board_string);
    }
    let fret_num_string = build_fret_num_string(starting_fret);
    fret_board.push(fret_num_string);
    fret_board
}

const NUM_THICK_STRINGS: usize = 3;

const FRET_LENGTH: [usize; 25] = [
    0, 10, 10, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5,
];

fn format_note(note: Note, string_char: char, flat: bool) -> String {
    let note_string = String::from(note_to_string(note, flat));
    if note_string.len() == 1 {
        format!("{}{}", note_string, string_char)
    } else {
        note_string
    }
}

fn format_fret_num(fret_num: usize) -> String {
    let fret_num_string = fret_num.to_string();
    if fret_num_string.len() == 1 {
        format!("{} ", fret_num_string)
    } else {
        fret_num_string
    }
}

fn build_fret_board_string(
    starting_fret: usize,
    notes_in_scale: &[Note],
    string: &Note,
    string_char: char,
    flat: bool,
) -> String {
    let mut fret_board_string = String::new();
    let empty_string_note_index = NOTES.iter().position(|&note| note == *string).unwrap();
    let notes_in_string = (0..=NUM_NOTES)
        .map(|fret| {
            let note_index = (empty_string_note_index + fret) % NUM_NOTES;
            NOTES[note_index]
        })
        .collect::<Vec<Note>>();
    for fret in starting_fret..(starting_fret + FRET_SPAN) {
        let note = notes_in_string[fret % NUM_NOTES];
        if fret == 0 {
            if notes_in_scale.contains(&note) {
                fret_board_string.push_str(format_note(note, string_char, flat).as_str());
            } else {
                fret_board_string.push(string_char);
                fret_board_string.push(string_char);
            }
        } else {
            fret_board_string.push('|');
            let fret_length = FRET_LENGTH[fret];
            if notes_in_scale.contains(&note) {
                let fret_length_odd = fret_length % 2 != 0;
                let first_half_fret_length = fret_length / 2 - if fret_length_odd { 0 } else { 1 };
                let second_half_fret_length = fret_length / 2 - 1;
                for _ in 0..first_half_fret_length {
                    fret_board_string.push(string_char);
                }
                fret_board_string.push_str(format_note(note, string_char, flat).as_str());
                for _ in 0..second_half_fret_length {
                    fret_board_string.push(string_char);
                }
            } else {
                for _ in 0..fret_length {
                    fret_board_string.push(string_char);
                }
            }
        }
    }
    fret_board_string.push('|');
    fret_board_string
}

fn build_fret_num_string(starting_fret: usize) -> String {
    let mut fret_num_string = String::new();
    (starting_fret..(starting_fret + FRET_SPAN)).for_each(|fret| {
        if fret == 0 {
            fret_num_string.push(' ');
            fret_num_string.push(' ');
        } else {
            fret_num_string.push('|');
            let fret_length = FRET_LENGTH[fret];
            let fret_length_odd = fret_length % 2 != 0;
            let first_half_fret_length = fret_length / 2 - if fret_length_odd { 0 } else { 1 };
            let second_half_fret_length = fret_length / 2 - 1;
            for _ in 0..first_half_fret_length {
                fret_num_string.push(' ');
            }
            fret_num_string.push_str(format_fret_num(fret).as_str());
            for _ in 0..second_half_fret_length {
                fret_num_string.push(' ');
            }
        }
    });
    fret_num_string.push('|');
    fret_num_string
}
