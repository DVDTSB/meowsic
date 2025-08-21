#![no_std]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use core::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Note {
    octave: usize,
    index: usize,
}

impl Note {
    pub fn from_midi(midi_note: u8) -> Self {
        let midi_note = midi_note as usize;
        let m = midi_note - 21;
        let index = m % 12;
        let octave = m / 12;
        Note { index, octave }
    }
    pub fn get_name(&self) -> String {
        let names = [
            "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
        ];
        names[self.index % 12].into()
    }
    pub fn get_name_idx(idx: usize) -> String {
        let names = [
            "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
        ];
        names[idx % 12].into()
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = self.octave * 12 + self.index;
        let other_val = other.octave * 12 + other.index;
        self_val.cmp(&other_val)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Chord {
    notes: Vec<Note>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Interval {
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectForth,
    Tritone,
    PerfectFifth,
    SharpFifth,
    Sixth,
    MinorSeventh,
    MajorSeventh,
}

impl Interval {
    fn get_third(self) -> String {
        match self {
            Interval::MinorSecond => "sus(b2)".into(),
            Interval::MajorSecond => "sus2".into(),
            Interval::MinorThird => "m".into(),
            Interval::MajorThird => "".into(),
            Interval::PerfectForth => "sus4".into(),
            _ => "".into(),
        }
    }
}

impl Chord {
    pub fn get_names(&self) -> Vec<String> {
        let mut names = Vec::new();

        let mut notes = self.notes.clone();
        notes.sort();

        let bass_note = notes[0].get_name();

        for &root in &notes {
            let relative_form: Vec<Note> = notes
                .iter()
                .map(|&x| Note {
                    index: (12 + x.index - root.index) % 12,
                    octave: x.octave,
                })
                .collect();
            let mut rel_sorted = relative_form.clone();
            rel_sorted.sort();

            if let Some(name) = Chord::get_name_from_relative_form(rel_sorted) {
                names.push(format!("{}{}", root.get_name(), name));
            }

            if root.get_name() == bass_note {
                continue;
            }

            let relative_form: Vec<Note> = notes
                .iter()
                .skip(1)
                .map(|&x| Note {
                    index: (12 + x.index - root.index) % 12,
                    octave: x.octave,
                })
                .collect();
            let mut rel_sorted = relative_form.clone();
            rel_sorted.sort();

            if let Some(name) = Chord::get_name_from_relative_form(rel_sorted) {
                names.push(format!("{}{}/{}", root.get_name(), name, bass_note));
            }
        }
        names
    }

    pub fn get_name_from_relative_form(relative_form: Vec<Note>) -> Option<String> {
        let size = relative_form.len();
        if size == 0 {
            return None;
        }
        if size == 1 {
            return Some("".into());
        }

        let mut has_interval = [false; 12];
        if size == 2 {
            if has_interval[7] {
                return Some("5".into());
            } else {
                return None;
            }
        }

        for note in &relative_form {
            has_interval[note.index] = true;
        }

        let third = if has_interval[4] {
            has_interval[4] = false;
            Some(Interval::MajorThird)
        } else if has_interval[3] {
            has_interval[3] = false;
            Some(Interval::MinorThird)
        } else if has_interval[5] {
            has_interval[5] = false;
            Some(Interval::PerfectForth)
        } else if has_interval[2] {
            has_interval[2] = false;
            Some(Interval::MajorSecond)
        } else if has_interval[1] {
            has_interval[1] = false;
            Some(Interval::MinorSecond)
        } else {
            None
        };

        let seventhish = if has_interval[11] {
            has_interval[11] = false;
            Some(Interval::MajorSeventh)
        } else if has_interval[10] {
            has_interval[10] = false;
            Some(Interval::MinorSeventh)
        } else if has_interval[9] {
            has_interval[9] = false;
            Some(Interval::Sixth)
        } else if has_interval[8] {
            has_interval[8] = false;
            Some(Interval::SharpFifth)
        } else {
            None
        };

        let ninth = if has_interval[1] {
            has_interval[1] = false;
            Some(Interval::MinorSecond) // b9
        } else if has_interval[2] {
            has_interval[2] = false;
            Some(Interval::MajorSecond) // 9
        } else if has_interval[3] {
            has_interval[3] = false;
            Some(Interval::MinorThird) // #9
        } else {
            None
        };

        let fifth = if has_interval[7] {
            has_interval[7] = false;
            Some(Interval::PerfectFifth)
        } else if has_interval[6] {
            has_interval[6] = false;
            Some(Interval::Tritone) // b5
        } else if has_interval[8] {
            has_interval[8] = false;
            Some(Interval::SharpFifth) // #5
        } else {
            None
        };

        let eleventh = if has_interval[5] {
            has_interval[5] = false;
            Some(Interval::PerfectForth) // 11
        } else if has_interval[6] {
            has_interval[6] = false;
            Some(Interval::Tritone) // #11
        } else {
            None
        };

        let thirteenth = if has_interval[9] {
            has_interval[9] = false;
            Some(Interval::Sixth) // 13 (enharmonic 6)
        } else if has_interval[8] {
            has_interval[8] = false;
            Some(Interval::SharpFifth) // b13
        } else {
            None
        };

        let quality = match (third, fifth) {
            (Some(Interval::MinorThird), Some(Interval::Tritone)) => "dim".into(),
            (Some(Interval::MajorThird), Some(Interval::SharpFifth)) => "aug".into(),
            (Some(t), _) => t.get_third(), // "m", "", "sus2", "sus4"
            (None, _) => "".into(),
        };

        let mut ext = match seventhish {
            Some(Interval::MajorSeventh) => "maj7".into(),
            Some(Interval::MinorSeventh) => "7".into(),
            Some(Interval::Sixth) => {
                if quality == "dim" {
                    "7".into()
                } else {
                    "6".into()
                }
            }
            Some(Interval::SharpFifth) => "b6".into(),

            _ => String::new(),
        };

        let mut alters: Vec<String> = Vec::new();
        let mut add: Vec<String> = Vec::new();

        if let Some(fv) = fifth {
            match (fv, third) {
                (Interval::Tritone, Some(Interval::MinorThird)) => {}
                (Interval::SharpFifth, Some(Interval::MajorThird)) => {}
                (Interval::Tritone, _) => alters.push("b5".into()),
                (Interval::SharpFifth, _) => alters.push("#5".into()),
                _ => {}
            }
        }

        let has_7 = ext == "7" || ext == "maj7";

        if let Some(n) = ninth {
            match n {
                Interval::MajorSecond => {
                    if ext == "7" {
                        ext = "9".into();
                    } else if ext == "maj7" {
                        ext = "maj9".into();
                    } else if ext == "6" || ext == "b6" {
                        add.push("/9".into());
                    } else if ext.is_empty() {
                        add.push("add9".into());
                    }
                }
                Interval::MinorSecond => {
                    if ext.is_empty() && !quality.contains("sus") {
                        add.push("addb9".into());
                    } else {
                        alters.push("b9".into());
                    }
                }
                Interval::MinorThird => {
                    if ext.is_empty() {
                        add.push("add#9".into());
                    } else {
                        alters.push("#9".into());
                    }
                }
                _ => {}
            }
        }

        if let Some(e) = eleventh {
            match e {
                Interval::PerfectForth => {
                    if ext == "9" || ext == "7" {
                        ext = "11".into();
                    } else if ext == "maj9" || ext == "maj7" {
                        ext = "maj11".into();
                    } else if ext == "6" || ext == "b6" {
                        add.push("/11".into());
                    } else if ext.is_empty() {
                        add.push("add11".into());
                    }
                }
                Interval::Tritone => {
                    if ext.is_empty() {
                        alters.push("add#11".into());
                    } else {
                        alters.push("#11".into());
                    }
                }
                _ => {}
            }
        }

        if let Some(t) = thirteenth {
            match t {
                Interval::Sixth => {
                    if has_7 || ext.contains("9") || ext.contains("11") || ext.contains("maj") {
                        ext = if ext.starts_with("maj") {
                            "maj13".into()
                        } else {
                            "13".into()
                        };
                    }
                }
                Interval::SharpFifth => {
                    if has_7 || ext.contains("9") || ext.contains("11") || ext.contains("maj") {
                        alters.push("b13".into());
                    } else {
                        alters.push("b13".into());
                    }
                }
                _ => {}
            }
        }

        let mut omissions: Vec<String> = Vec::new();
        if third.is_none() {
            omissions.push("no3".into());
        }
        /* if fifth.is_none() && !(ext.co) {
            omissions.push("no5".into());
        }*/
        if ninth.is_none() && (ext.contains("13") || ext.contains("11")) {
            omissions.push("no9".into());
        }

        /*if eleventh.is_none() && ext.contains("13") {
            omissions.push("no11".into());
        }*/

        // combine

        let mut res = String::new();
        if quality.starts_with("sus") {
            res.push_str(&ext);
            res.push_str(&quality);
        } else {
            res.push_str(&quality);
            res.push_str(&ext);
        }

        res.push_str(&add.join(""));

        let mut modifiers = Vec::new();
        modifiers.extend(alters);
        modifiers.extend(omissions);

        if !modifiers.is_empty() {
            res.push_str("(");
            res.push_str(&modifiers.join(","));
            res.push_str(")");
        }

        Some(res)
    }
}

pub fn get_chords(notes: &Vec<u8>) -> Vec<String> {
    let notes = notes.iter().map(|&x| Note::from_midi(x)).collect();

    Chord { notes }.get_names()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_cmaj7() {
        let chord = Chord {
            notes: vec![
                Note::from_midi(60), // C
                Note::from_midi(64), // E
                Note::from_midi(67), // G
                Note::from_midi(71), // B
            ],
        };
        assert!(chord.get_names().contains(&"Cmaj7".into()));
    }

    #[test]
    fn test_c9() {
        let chord = Chord {
            notes: vec![
                Note::from_midi(60), // C
                Note::from_midi(64), // E
                Note::from_midi(67), // G
                Note::from_midi(71), // B
                Note::from_midi(74), // D
            ],
        };
        assert!(chord.get_names().contains(&"Cmaj9".into()));
    }

    #[test]
    fn test_c13() {
        let chord = Chord {
            notes: vec![
                Note::from_midi(60), // C
                Note::from_midi(64), // E
                Note::from_midi(67), // G
                Note::from_midi(71), // B
                Note::from_midi(74), // D
                Note::from_midi(81), // A
            ],
        };
        assert!(chord.get_names().contains(&"Cmaj13".into()));
    }

    #[test]
    fn test_c_sus4() {
        let chord = Chord {
            notes: vec![
                Note::from_midi(60), // C
                Note::from_midi(65), // F
                Note::from_midi(67), // G
            ],
        };
        assert!(chord.get_names().contains(&"Csus4".into()));
    }
}
