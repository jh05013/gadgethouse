use egui::{RichText, vec2};

use super::HouseGadget;
use std::fmt::Write;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#0123456789";
const ALPHABET_CNT: usize = 26;
const LETTER_CNT: usize = 37;
const LARGE_FONT_SIZE: f32 = 24.0;
const LARGE_BUTTON_SIZE: f32 = 35.0;

const BINARY: [&str; ALPHABET_CNT] = [
    "00001", "00010", "00011", "00100", "00101", // ABCDE
    "00110", "00111", "01000", "01001", "01010", // FGHIJ
    "01011", "01100", "01101", "01110", "01111", // KLMNO
    "10000", "10001", "10010", "10011", "10100", // PQRST
    "10101", "10110", "10111", "11000", "11001", // UVWXY
    "11010", // Z
];
const BRAILLE: [&str; LETTER_CNT] = [
    "100000", "101000", "110000", "110100", "100100", // ABCDE
    "111000", "111100", "101100", "011000", "011100", // FGHIJ
    "100010", "101010", "110010", "110110", "100110", // KLMNO
    "111010", "111110", "101110", "011010", "011110", // PQRST
    "100011", "101011", "011101", "110011", "110111", // UVWXY
    "100111", "010111", // Z#
    "011100", "100000", "101000", "110000", "110100", // 01234
    "100100", "111000", "111100", "101100", "011000", // 56789
];
const MORSE: [&str; LETTER_CNT] = [
    "12000", "21110", "21210", "21100", "10000", // ABCDE
    "11210", "22100", "11110", "11000", "12220", // FGHIJ
    "21200", "12110", "22000", "21000", "22200", // KLMNO
    "12210", "22120", "12100", "11100", "20000", // PQRST
    "11200", "11120", "12200", "21120", "21220", // UVWXY
    "22110", "99999", // Z#
    "22222", "12222", "11222", "11122", "11112", // 01234
    "11111", "21111", "22111", "22211", "22221", // 56789
];
const SEMAPHORE: [&str; LETTER_CNT] = [
    "00000110", "00010010", "10000010", "01000010", // ABCD
    "00100010", "00001010", "00000011", "00010100", // EFGH
    "10000100", "01001000", "01000100", "00100100", // IJKL
    "00001100", "00000101", "10010000", "01010000", // MNOP
    "00110000", "00011000", "00010001", "11000000", // QRST
    "10100000", "01000001", "00101000", "00100001", // UVWX
    "10001000", "00001001", "01100000", // YZ#
    "01001000", "00000110", "00010010", "10000010", // 0123
    "01000010", "00100010", "00001010", "00000011", // 4567
    "00010100", "10000100", // 89
];
const TERNARY: [&str; ALPHABET_CNT] = [
    "001", "002", "010", "011", "012", // ABCDE
    "020", "021", "022", "100", "101", // FGHIJ
    "102", "110", "111", "112", "120", // KLMNO
    "121", "122", "200", "201", "202", // PQRST
    "210", "211", "212", "220", "221", // UVWXY
    "212", // Z
];

const INDEXING_WHITESPACE_ERR: &str = "Please separate the indices by whitespaces only";
const OOB_ERR: &str = "Out of bounds";

#[derive(Default)]
struct Indexing {
    input: String,
    indices: String,
    use_0_indexing: bool,
}

impl Indexing {
    /// Returns (input length, extracted string).
    fn get(&self) -> (usize, String) {
        let alphabets = self
            .input
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<_>>();
        let input_length = alphabets.len();

        let Ok(indices) = self
            .indices
            .split_whitespace()
            .map(|num| num.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
        else {
            return (input_length, INDEXING_WHITESPACE_ERR.to_string());
        };

        let Some(extracted) = indices
            .into_iter()
            .map(|index| {
                let index = if self.use_0_indexing {
                    Some(index)
                } else {
                    index.checked_sub(1)
                }?;
                alphabets.get(index).copied()
            })
            .collect::<Option<String>>()
        else {
            return (input_length, OOB_ERR.to_string());
        };

        (input_length, extracted.to_uppercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
enum CipherMode {
    A1Z26,
    Binary,
    Braille,
    Morse,
    Semaphore,
    Ternary,
}

impl CipherMode {
    fn name(&self) -> &str {
        match self {
            Self::A1Z26 => "🆎 A1Z26",
            Self::Binary => "🔟 Binary",
            Self::Braille => "✋ Braille",
            Self::Morse => "〰 Morse",
            Self::Semaphore => "🚩 Semaphore",
            Self::Ternary => "３ Ternary",
        }
    }

    fn answer_list(&self) -> &[&str] {
        match self {
            Self::Binary => &BINARY,
            Self::Braille => &BRAILLE,
            Self::Morse => &MORSE,
            Self::Semaphore => &SEMAPHORE,
            Self::Ternary => &TERNARY,
            _ => unreachable!("{self:?} doesn't support wildcards"),
        }
    }

    fn has_two(&self) -> bool {
        matches!(self, Self::Morse | Self::Ternary)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum CipherCriterion {
    Blank,
    Yes,
    No,
    Two, // 2 for Ternary, dash for Morse
}

impl CipherCriterion {
    fn label(&self, mode: CipherMode) -> char {
        match (self, mode) {
            (Self::Blank, _) => ' ',
            (Self::Yes, CipherMode::Binary) => '1',
            (Self::No, CipherMode::Binary) => '0',
            (Self::Two, CipherMode::Morse) => '—',
            (Self::Yes, CipherMode::Morse) => '⚫',
            (Self::Two, CipherMode::Ternary) => '2',
            (Self::Yes, CipherMode::Ternary) => '1',
            (Self::No, CipherMode::Ternary) => '0',
            (Self::No, _) => '✖',
            (_, CipherMode::Braille) => '⏺',
            (_, CipherMode::Semaphore) => '🚩',
            _ => unreachable!(),
        }
    }

    fn matches(&self, ans: char, exact: bool) -> bool {
        if ans == '9' {
            // e.g. in MORSE, # is "99999" since it doesn't exist
            return false;
        }

        let choice = if exact && matches!(self, Self::Blank) {
            Self::No
        } else {
            *self
        };
        matches!(
            (ans, choice),
            (_, Self::Blank) | ('0', Self::No) | ('1', Self::Yes) | ('2', Self::Two)
        )
    }

    fn left_click(&mut self, mode: CipherMode) {
        *self = match self {
            Self::Blank | Self::No => Self::Yes,
            Self::Yes => {
                if mode.has_two() {
                    Self::Two
                } else {
                    Self::Blank
                }
            }
            Self::Two => Self::Blank,
        };
    }

    fn right_click(&mut self) {
        *self = match self {
            Self::Blank | Self::Yes | Self::Two => Self::No,
            Self::No => Self::Blank,
        };
    }
}

pub struct Cipher {
    mode: CipherMode,
    use_numbers: bool,
    criteria: [CipherCriterion; 10],
    input: String,
}

impl Cipher {
    fn match_letter(&self, num: usize, exact: bool) -> bool {
        let answer = self.mode.answer_list()[num];
        answer
            .chars()
            .zip(&self.criteria)
            .all(|(ans, crit)| crit.matches(ans, exact))
    }

    fn match_result(&self, exact: bool) -> impl Iterator<Item = char> + '_ {
        LETTERS
            .char_indices()
            .take(if self.use_numbers { 37 } else { 26 })
            .filter(move |(i, _)| self.match_letter(*i, exact))
            .map(|(_, letter)| letter)
    }

    fn ui_with_wildcard(
        &mut self,
        ui: &mut egui::Ui,
        name: &str,
        newlines: Vec<usize>,
        special_labels: Vec<(usize, char)>,
    ) {
        let total = newlines.last().unwrap() + 1;
        egui::Grid::new(name).show(ui, |ui| {
            for (i, crit) in self.criteria[..total].iter_mut().enumerate() {
                let text = RichText::new(crit.label(self.mode)).size(LARGE_FONT_SIZE);
                let response = ui.add(
                    egui::Button::new(text).min_size(vec2(LARGE_BUTTON_SIZE, LARGE_BUTTON_SIZE)),
                );
                if response.clicked() {
                    crit.left_click(self.mode);
                }
                if response.secondary_clicked() {
                    crit.right_click();
                }

                if let Some((_, c)) = special_labels.iter().find(|(index, _)| *index == i) {
                    ui.label(RichText::new(*c).size(LARGE_FONT_SIZE));
                }
                if newlines.contains(&i) {
                    ui.end_row();
                }
            }
        });

        ui.separator();

        ui.heading("Exact match");
        let result = self
            .match_result(true)
            .fold(String::new(), |mut s, letter| {
                write!(&mut s, "{letter} ").unwrap();
                s
            });
        ui.label(result);

        ui.heading("Blank match");
        let result = self
            .match_result(false)
            .fold(String::new(), |mut s, letter| {
                write!(&mut s, "{letter} ").unwrap();
                s
            });
        ui.label(result);
    }

    fn ui_a1z26(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.input);
        let mut answer = String::new();
        let mut warn_oob = false;
        for word in self.input.split_whitespace() {
            let Ok(num) = word.parse::<usize>() else {
                write!(&mut answer, "{word} ").unwrap();
                continue;
            };
            if !(1..=ALPHABET_CNT).contains(&num) {
                write!(&mut answer, "✖ ").unwrap();
                warn_oob = true;
                continue;
            }
            let letter = LETTERS.as_bytes()[num - 1] as char;
            write!(&mut answer, "{letter} ").unwrap();
        }

        ui.label(answer);
        if warn_oob {
            ui.label(
                RichText::new("⚠ There is an out-of-bound number")
                    .color(ui.visuals().warn_fg_color),
            );
        }
    }

    fn ui_binary(&mut self, ui: &mut egui::Ui) {
        self.ui_with_wildcard(ui, "Binary", vec![4], vec![]);
    }

    fn ui_braille(&mut self, ui: &mut egui::Ui) {
        self.ui_with_wildcard(ui, "Braille", vec![1, 3, 5], vec![]);
        ui.checkbox(&mut self.use_numbers, "Use numbers");
    }

    fn ui_morse(&mut self, ui: &mut egui::Ui) {
        self.ui_with_wildcard(ui, "Morse", vec![4], vec![]);
        ui.checkbox(&mut self.use_numbers, "Use numbers");
    }

    fn ui_semaphore(&mut self, ui: &mut egui::Ui) {
        self.ui_with_wildcard(ui, "Semaphore", vec![2, 4, 7], vec![(3, '☃')]);
        ui.checkbox(&mut self.use_numbers, "Use numbers");
    }

    fn ui_ternary(&mut self, ui: &mut egui::Ui) {
        self.ui_with_wildcard(ui, "Ternary", vec![2], vec![]);
    }
}

#[derive(PartialEq)]
enum Tool {
    Links,
    Indexing,
    Cipher,
}

pub struct PuzzleHuntTools {
    tab: Tool,

    indexing: Indexing,
    cipher: Cipher,
}

impl PuzzleHuntTools {
    fn ui_indexing(&mut self, ui: &mut egui::Ui) {
        let (input_length, extracted) = self.indexing.get();

        ui.label(format!("Input ({input_length} chars)"));
        ui.add(egui::TextEdit::singleline(&mut self.indexing.input));
        ui.label("Indices");
        ui.add(egui::TextEdit::singleline(&mut self.indexing.indices));
        ui.checkbox(&mut self.indexing.use_0_indexing, "Use 0-indexing");
        ui.add_space(6.0);

        ui.heading("Result");
        ui.label(extracted);
    }

    fn ui_cipher(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for cipher_mode in CipherMode::iter() {
                let name = cipher_mode.name();
                let response = ui.selectable_value(&mut self.cipher.mode, cipher_mode, name);
                if response.clicked() {
                    self.cipher.criteria = [CipherCriterion::Blank; 10];
                    self.cipher.use_numbers = false;
                }
            }
        });
        ui.separator();

        match &self.cipher.mode {
            CipherMode::A1Z26 => self.cipher.ui_a1z26(ui),
            CipherMode::Binary => self.cipher.ui_binary(ui),
            CipherMode::Braille => self.cipher.ui_braille(ui),
            CipherMode::Morse => self.cipher.ui_morse(ui),
            CipherMode::Semaphore => self.cipher.ui_semaphore(ui),
            CipherMode::Ternary => self.cipher.ui_ternary(ui),
        };
    }
}

impl HouseGadget for PuzzleHuntTools {
    fn new() -> Self {
        Self {
            tab: Tool::Links,
            indexing: Default::default(),
            cipher: Cipher {
                mode: CipherMode::A1Z26,
                use_numbers: false,
                criteria: [CipherCriterion::Blank; 10],
                input: String::default(),
            },
        }
    }

    fn title(&self) -> String {
        "Puzzlehunt Tools".to_string()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tool::Links, "Useful links");
            ui.selectable_value(&mut self.tab, Tool::Indexing, "Indexing");
            ui.selectable_value(&mut self.tab, Tool::Cipher, "Cipher");
        });
        ui.separator();

        match self.tab {
            Tool::Links => useful_links(ui),
            Tool::Indexing => self.ui_indexing(ui),
            Tool::Cipher => self.ui_cipher(ui),
        };
    }
}

fn useful_links(ui: &mut egui::Ui) {
    ui.heading("Lists");
    ui.hyperlink_to(
        "MIT Hunt DB",
        "http://devjoe.appspot.com/huntindex/index/index.html",
    );
    ui.hyperlink_to(
        "Sets of Things",
        "https://phenomist.wordpress.com/storage/sets/",
    );

    ui.heading("Phrase finder");
    ui.hyperlink_to("Nutrimatic", "https://nutrimatic.org");
    ui.hyperlink_to("Qat (word finder)", "https://quinapalus.com/cgi-bin/match");

    ui.heading("Solver");
    ui.hyperlink_to("Noq (logic puzzle)", "https://www.noq.solutions");
    ui.hyperlink_to(
        "Wordplays (crossword)",
        "https://www.wordplays.com/crossword-solver/",
    );
    ui.hyperlink_to("qhex (multi tools)", "https://tools.qhex.org/");
    ui.hyperlink_to("quipquip (cryptogram)", "https://quipqiup.com/");
}
