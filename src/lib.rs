// src/lib.rs
use std::collections::HashMap;

pub fn generate_bg_color_utilities() -> String {
    let mut colors = HashMap::new();
    colors.insert("red", vec!["#fee2e2", "#fca5a5", "#f87171", "#ef4444", "#dc2626"]);
    colors.insert("blue", vec!["#dbeafe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb"]);
    colors.insert("green", vec!["#d1fae5", "#6ee7b7", "#34d399", "#10b981", "#059669"]);
    colors.insert("yellow", vec!["#fef9c3", "#fef08a", "#fde047", "#facc15", "#eab308"]);
    colors.insert("white", vec!["#ffffff", "#f9fafb", "#f3f4f6", "#e5e7eb", "#d1d5db"]);
    colors.insert("black", vec!["#6b7280", "#4b5563", "#374151", "#1f2937", "#000000"]);

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
pub fn generate_text_color_utilities() -> String {
    let mut colors = HashMap::new();

    colors.insert("red", vec!["#fee2e2", "#fca5a5", "#f87171", "#ef4444", "#dc2626"]);
    colors.insert("blue", vec!["#dbeafe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb"]);
    colors.insert("green", vec!["#d1fae5", "#6ee7b7", "#34d399", "#10b981", "#059669"]);
    colors.insert("yellow", vec!["#fef9c3", "#fef08a", "#fde047", "#facc15", "#eab308"]);
    colors.insert("white", vec!["#ffffff", "#f9fafb", "#f3f4f6", "#e5e7eb", "#d1d5db"]);
    colors.insert("black", vec!["#6b7280", "#4b5563", "#374151", "#1f2937", "#000000"]);

    let shades = vec!["100", "200", "300", "400", "500"];
    let mut css = String::new();

    for (color, hexes) in colors.iter() {
        for (i, hex) in hexes.iter().enumerate() {
            let shade = shades[i];
            css.push_str(&format!(".text-{}-{} {{ color: {}; }}\n", color, shade, hex));
        }
    }

    css
}

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
pub fn generate_margin_utilities() -> String {
    let spacings = vec![0, 1, 2, 3, 4, 5];
    let mut css = String::new();

    for val in spacings {
        let px = val * 4;
        css.push_str(&format!(".m-{} {{ margin: {}px; }}\n", px, px));
        css.push_str(&format!(".mt-{} {{ margin-top: {}px; }}\n", px, px));
        css.push_str(&format!(".mr-{} {{ margin-right: {}px; }}\n", px, px));
        css.push_str(&format!(".mb-{} {{ margin-bottom: {}px; }}\n", px, px));
        css.push_str(&format!(".ml-{} {{ margin-left: {}px; }}\n", px, px));
        css.push_str(&format!(".mx-{} {{ margin-left: {}px; margin-right: {}px; }}\n", px, px, px));
        css.push_str(&format!(".my-{} {{ margin-top: {}px; margin-bottom: {}px; }}\n", px, px, px));
    }

    css
}


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

pub fn generate_font_weight_utilities() -> String {
    let weights = vec![
        ("thin", "100"),
        ("extralight", "200"),
        ("light", "300"),
        ("normal", "400"),
        ("medium", "500"),
        ("semibold", "600"),
        ("bold", "700"),
        ("extrabold", "800"),
        ("black", "900"),
    ];

    let mut css = String::new();

    for (name, value) in weights {
        css.push_str(&format!(".font-{} {{ font-weight: {}; }}\n", name, value));
    }

    css
}

pub fn generate_font_style_utilities() -> String {
    let mut css = String::new();

    css.push_str(".underline { text-decoration: underline; }\n");
    css.push_str(".line-through { text-decoration: line-through; }\n");
    css.push_str(".no-underline { text-decoration: none; }\n");

    css.push_str(".italic { font-style: italic; }\n");
    css.push_str(".not-italic { font-style: normal; }\n");

    css
}



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
