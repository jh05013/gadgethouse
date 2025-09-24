use egui::RichText;
use emojis::Emoji;

use crate::house::HouseGadget;

const MEDIUM_FONT_SIZE: f32 = 16.0;

pub struct EmojiPicker {
    search_string: String,
    copied_emoji: String,
}

impl HouseGadget for EmojiPicker {
    fn new() -> Self {
        Self {
            search_string: String::default(),
            copied_emoji: String::default(),
        }
    }

    fn title(&self) -> String {
        "Emoji".to_string()
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.search_string);
        });
        let copy_info_text = format!("Click on the text to copy an emoji {}", self.copied_emoji);
        ui.label(copy_info_text);

        let matched_emojis = emojis::iter()
            .filter(|emoji| self.matches(emoji))
            .collect::<Vec<_>>();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for emoji in matched_emojis {
                self.show_emoji(ui, emoji);
            }
        });
    }
}

impl EmojiPicker {
    fn matches(&self, emoji: &Emoji) -> bool {
        emoji.name().contains(&self.search_string)
            || emoji
                .shortcodes()
                .any(|shortcode| shortcode.contains(&self.search_string))
    }

    fn show_emoji(&mut self, ui: &mut egui::Ui, emoji: &Emoji) {
        let emoji_string = format!("{} {}", emoji.as_str(), emoji.name(),);
        let emoji_text = RichText::new(emoji_string).size(MEDIUM_FONT_SIZE);
        if ui.label(emoji_text).clicked() {
            self.copied_emoji = emoji.to_string();
            ui.ctx().copy_text(emoji.to_string());
        }
    }
}
