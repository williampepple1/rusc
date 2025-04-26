pub fn generate_border_radius_utilities() -> String {
    let radii = vec![
        ("none", "0px"),
        ("sm", "0.125rem"),
        ("", "0.25rem"),        // default .rounded
        ("md", "0.375rem"),
        ("lg", "0.5rem"),
        ("xl", "0.75rem"),
        ("2xl", "1rem"),
        ("3xl", "1.5rem"),
        ("full", "9999px"),
    ];

    let mut css = String::new();

    for (name, value) in radii {
        if name.is_empty() {
            css.push_str(&format!(".rounded {{ border-radius: {}; }}\n", value));
        } else {
            css.push_str(&format!(".rounded-{} {{ border-radius: {}; }}\n", name, value));
        }
    }

    css
}

pub fn generate_border_corner_radius_utilities() -> String {
    let radii = vec![
        ("none", "0px"),
        ("sm", "0.125rem"),
        ("md", "0.375rem"),
        ("lg", "0.5rem"),
        ("xl", "0.75rem"),
        ("2xl", "1rem"),
        ("full", "9999px"),
    ];

    let mut css = String::new();

    for (name, value) in radii.iter() {
        css.push_str(&format!(".rounded-tl-{} {{ border-top-left-radius: {}; }}\n", name, value));
        css.push_str(&format!(".rounded-tr-{} {{ border-top-right-radius: {}; }}\n", name, value));
        css.push_str(&format!(".rounded-bl-{} {{ border-bottom-left-radius: {}; }}\n", name, value));
        css.push_str(&format!(".rounded-br-{} {{ border-bottom-right-radius: {}; }}\n", name, value));
    }

    css
}


pub fn generate_border_width_utilities() -> String {
    let widths = vec![
        ("", "1px"),      // default `.border`
        ("2", "2px"),
        ("4", "4px"),
        ("8", "8px"),
    ];

    let mut css = String::new();

    for (name, value) in widths {
        if name.is_empty() {
            css.push_str(&format!(".border {{ border-width: {}; }}\n", value));
        } else {
            css.push_str(&format!(".border-{} {{ border-width: {}; }}\n", name, value));
        }
    }

    css
}


use std::collections::HashMap;

pub fn generate_border_color_utilities() -> String {
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
            let shade = if hexes.len() > 1 { shades[i] } else { "" };
            if shade.is_empty() {
                css.push_str(&format!(".border-{} {{ border-color: {}; }}\n", color, hex));
            } else {
                css.push_str(&format!(".border-{}-{} {{ border-color: {}; }}\n", color, shade, hex));
            }
        }
    }

    css
}


pub fn generate_border_style_utilities() -> String {
    let styles = vec![
        ("solid", "solid"),
        ("dashed", "dashed"),
        ("dotted", "dotted"),
        ("double", "double"),
        ("none", "none"),
    ];

    let mut css = String::new();

    for (name, value) in styles {
        css.push_str(&format!(".border-{} {{ border-style: {}; }}\n", name, value));
    }

    css
}
