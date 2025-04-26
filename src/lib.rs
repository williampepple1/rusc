// src/lib.rs
mod utils;
use utils::bg::generate_bg_color_utilities;
use utils::text::generate_text_color_utilities;
use utils::font::{generate_font_size_utilities, generate_font_style_utilities, generate_font_weight_utilities};
use utils::layout::{generate_padding_utilities, generate_margin_utilities, generate_height_utilities, generate_max_height_utilities, generate_width_utilities, generate_max_width_utilities, generate_min_height_utilities, generate_min_width_utilities};
use utils::border::generate_border_radius_utilities;

pub fn generate_css() -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        generate_bg_color_utilities(),
        generate_padding_utilities(),
        generate_font_size_utilities(),
        generate_font_weight_utilities(),
        generate_font_style_utilities(),
        generate_margin_utilities(),
        generate_text_color_utilities(),
        generate_height_utilities(),
        generate_max_height_utilities(),
        generate_width_utilities(),
        generate_max_width_utilities(),
        generate_min_height_utilities(),
        generate_min_width_utilities(),
        generate_border_radius_utilities()

    )
}
