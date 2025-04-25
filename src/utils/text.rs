use std::collections::HashMap;

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