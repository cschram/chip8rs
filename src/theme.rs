use ggez::graphics::Color;
use ggegui::egui;

const BACKGROUND: Color = Color {
  r: 0.1,
  g: 0.1,
  b: 0.1,
  a: 1.0,
};

const TEXT: Color = Color {
  r: 1.0,
  g: 1.0,
  b: 1.0,
  a: 1.0,
};

fn into_color32(color: Color) -> egui::Color32 {
  let rgb = color.to_rgb();
  egui::Color32::from_rgb(rgb.0, rgb.1, rgb.2)
}

pub fn egui_style() -> egui::Style {
  let mut visuals = egui::Visuals::default();
  visuals.override_text_color = Some(into_color32(TEXT));
  visuals.panel_fill = into_color32(BACKGROUND);

  let mut style = egui::Style::default();
  style.visuals = visuals;
  style.text_styles = [
    (egui::TextStyle::Small, egui::FontId::new(9.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Body, egui::FontId::new(12.5, egui::FontFamily::Proportional)),
    (egui::TextStyle::Button, egui::FontId::new(12.5, egui::FontFamily::Proportional)),
    (egui::TextStyle::Heading, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
    (egui::TextStyle::Monospace, egui::FontId::new(12.0, egui::FontFamily::Monospace)),
  ].into();
  style.spacing.button_padding = [10.0, 10.0].into();
  
  style
}
