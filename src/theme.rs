use ggez::graphics::Color;
use ggegui::egui::{self, Style, Visuals, TextStyle, FontId};

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

pub fn egui_style() -> Style {
  let mut visuals = Visuals::default();
  visuals.override_text_color = Some(into_color32(TEXT));
  visuals.panel_fill = into_color32(BACKGROUND);

  let mut style = Style::default();
  style.visuals = visuals;
  style.text_styles = [
    (TextStyle::Small, FontId::proportional(9.0)),
    (TextStyle::Body, FontId::proportional(12.5)),
    (TextStyle::Button, FontId::proportional(12.5)),
    (TextStyle::Heading, FontId::proportional(18.0)),
    (TextStyle::Monospace, FontId::monospace(12.0)),
  ].into();
  style.spacing.button_padding = [40.0, 40.0].into();
  
  style
}
