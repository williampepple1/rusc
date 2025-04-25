use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn generate_bg_color_utilities() -> String {
    let mut colors = HashMap::new();
    colors.insert("red", vec!["#fee2e2", "#fca5a5", "#f87171", "#ef4444", "#dc2626"]);
    colors.insert("blue", vec!["#dbeafe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb"]);
    colors.insert("green", vec!["#d1fae5", "#6ee7b7", "#34d399", "#10b981", "#059669"]);
    colors.insert("yellow", vec!["#fef9c3", "#fef08a", "#fde047", "#facc15", "#eab308"]);

    let shades = vec!["100", "200", "300", "400", "500"];
    let mut css = String::new();

    for (color, hexes) in colors.iter() {
        for (i, hex) in hexes.iter().enumerate() {
            let shade = shades[i];
            css.push_str(&format!(".bg-{}-{} {{ background-color: {}; }}\n", color, shade, hex));
        }
    }

    css
}

#[wasm_bindgen]
pub fn generate_padding_utilities() -> String {
    let spacings = vec![0, 1, 2, 3, 4, 5];
    let mut css = String::new();

    for val in spacings {
        let px = val * 4;
        css.push_str(&format!(".p-{} {{ padding: {}px; }}\n", px, px));
        css.push_str(&format!(".pt-{} {{ padding-top: {}px; }}\n", px, px));
        css.push_str(&format!(".pr-{} {{ padding-right: {}px; }}\n", px, px));
        css.push_str(&format!(".pb-{} {{ padding-bottom: {}px; }}\n", px, px));
        css.push_str(&format!(".pl-{} {{ padding-left: {}px; }}\n", px, px));
        css.push_str(&format!(".px-{} {{ padding-left: {}px; padding-right: {}px; }}\n", px, px, px));
        css.push_str(&format!(".py-{} {{ padding-top: {}px; padding-bottom: {}px; }}\n", px, px, px));
    }

    css
}

#[wasm_bindgen]
pub fn generate_font_size_utilities() -> String {
    let sizes = vec![
        ("xs", "0.75rem"),
        ("sm", "0.875rem"),
        ("base", "1rem"),
        ("lg", "1.125rem"),
        ("xl", "1.25rem"),
        ("2xl", "1.5rem"),
        ("3xl", "1.875rem"),
        ("4xl", "2.25rem"),
        ("5xl", "3rem"),
    ];

    let mut css = String::new();

    for (name, value) in sizes {
        css.push_str(&format!(".text-{} {{ font-size: {}; }}\n", name, value));
    }

    css
}

#[wasm_bindgen]
pub fn generate_css() -> String {
    format!(
        "{}{}",
        generate_bg_color_utilities(),
        generate_padding_utilities(),
        generate_font_size_utilities()
    )
}
