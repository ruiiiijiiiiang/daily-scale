use super::notes::Note;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Tuning {
    StandardE6,
    OpenG6,
    OpenE6,
    OpenD6,
    OpenC6,
    OpenA6,
    DropD6,
    StandardD6,
    DropCSharp6,
    StandardCSharp6,
    DropC6,
    StandardC6,
    StandardB7,
    DropA7,
    StandardA7,
    AllFourths7,
}

pub const fn tuning_to_string(tuning: Tuning) -> &'static str {
    match tuning {
        Tuning::StandardE6 => "Standard E (6 string)",
        Tuning::OpenG6 => "Open G (6 string)",
        Tuning::OpenE6 => "Open E (6 string)",
        Tuning::OpenD6 => "Open D (6 string)",
        Tuning::OpenC6 => "Open C (6 string)",
        Tuning::OpenA6 => "Open A (6 string)",
        Tuning::DropD6 => "Drop D (6 string)",
        Tuning::StandardD6 => "Standard D (6 string)",
        Tuning::DropCSharp6 => "Drop C# (6 string)",
        Tuning::StandardCSharp6 => "Standard C# (6 string)",
        Tuning::DropC6 => "Drop C (6 string)",
        Tuning::StandardC6 => "Standard C (6 string)",
        Tuning::StandardB7 => "Standard B (7 string)",
        Tuning::DropA7 => "Drop A (7 string)",
        Tuning::StandardA7 => "Standard A (7 string)",
        Tuning::AllFourths7 => "All fourths (7 string)",
    }
}

pub const fn get_notes_by_tuning(tuning: Tuning) -> &'static [Note] {
    match tuning {
        Tuning::StandardE6 => &[Note::E, Note::A, Note::D, Note::G, Note::B, Note::E],
        Tuning::OpenG6 => &[Note::D, Note::G, Note::D, Note::G, Note::B, Note::D],
        Tuning::OpenE6 => &[Note::E, Note::B, Note::E, Note::GSharp, Note::B, Note::E],
        Tuning::OpenD6 => &[Note::D, Note::A, Note::D, Note::FSharp, Note::A, Note::D],
        Tuning::OpenC6 => &[Note::C, Note::G, Note::C, Note::G, Note::C, Note::E],
        Tuning::OpenA6 => &[Note::E, Note::A, Note::E, Note::A, Note::CSharp, Note::E],
        Tuning::DropD6 => &[Note::D, Note::A, Note::D, Note::G, Note::B, Note::E],
        Tuning::StandardD6 => &[Note::D, Note::G, Note::C, Note::F, Note::A, Note::D],
        Tuning::DropCSharp6 => &[
            Note::CSharp,
            Note::GSharp,
            Note::CSharp,
            Note::FSharp,
            Note::ASharp,
            Note::DSharp,
        ],
        Tuning::StandardCSharp6 => &[
            Note::CSharp,
            Note::FSharp,
            Note::CSharp,
            Note::E,
            Note::GSharp,
            Note::CSharp,
        ],
        Tuning::DropC6 => &[Note::C, Note::G, Note::C, Note::F, Note::A, Note::D],
        Tuning::StandardC6 => &[
            Note::C,
            Note::F,
            Note::ASharp,
            Note::DSharp,
            Note::G,
            Note::C,
        ],
        Tuning::StandardB7 => &[
            Note::B,
            Note::E,
            Note::A,
            Note::D,
            Note::G,
            Note::B,
            Note::E,
        ],
        Tuning::DropA7 => &[
            Note::A,
            Note::E,
            Note::A,
            Note::D,
            Note::G,
            Note::B,
            Note::E,
        ],
        Tuning::StandardA7 => &[
            Note::A,
            Note::D,
            Note::G,
            Note::C,
            Note::F,
            Note::A,
            Note::D,
        ],
        Tuning::AllFourths7 => &[
            Note::B,
            Note::E,
            Note::A,
            Note::D,
            Note::G,
            Note::C,
            Note::F,
        ],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_tuning_name() {
        assert_eq!(tuning_to_string(Tuning::OpenG6), "Open G (6 string)");
        assert_eq!(tuning_to_string(Tuning::DropA7), "Drop A (7 string)");
    }

    #[test]
    fn test_get_notes_by_tuning() {
        assert_eq!(
            get_notes_by_tuning(Tuning::StandardCSharp6),
            &[
                Note::CSharp,
                Note::FSharp,
                Note::CSharp,
                Note::E,
                Note::GSharp,
                Note::CSharp,
            ]
        );
        assert_eq!(
            get_notes_by_tuning(Tuning::DropA7),
            &[
                Note::A,
                Note::E,
                Note::A,
                Note::D,
                Note::G,
                Note::B,
                Note::E
            ]
        );
    }
}
