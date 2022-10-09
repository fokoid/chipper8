use egui::widget_text::RichText;

pub fn monospace(s: &str) -> RichText {
    RichText::new(s).monospace().size(16.0)
}

