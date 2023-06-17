use ggez::graphics::Color;
use ggegui::egui;

pub const Background: Color = Color {
  r: 0.1,
  g: 0.1,
  b: 0.1,
  a: 1.0,
};

pub const Text: Color = Color {
  r: 1.0,
  g: 1.0,
  b: 1.0,
  a: 1.0,
};

pub fn into_color32(color: Color) -> egui::Color32 {
  let rgb = color.to_rgb();
  egui::Color32::from_rgb(rgb.0, rgb.1, rgb.2)
}
