// src/lib.rs
mod utils;
use utils::bg::generate_bg_color_utilities;
use utils::text::generate_text_color_utilities;
use utils::padding::generate_padding_utilities;
use utils::margin::generate_margin_utilities;
use utils::font::{generate_font_size_utilities, generate_font_style_utilities, generate_font_weight_utilities};


pub fn generate_css() -> String {
    format!(
        "{}{}{}{}{}{}{}",
        generate_bg_color_utilities(),
        generate_padding_utilities(),
        generate_font_size_utilities(),
        generate_font_weight_utilities(),
        generate_font_style_utilities(),
        generate_margin_utilities(),
        generate_text_color_utilities()

    )
}
