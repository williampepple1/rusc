pub fn generate_hover_focus_utilities() -> String {
    let mut css = String::new();

    // Example hover states
    css.push_str(".hover:underline:hover { text-decoration: underline; }\n");
    css.push_str(".hover:italic:hover { font-style: italic; }\n");
    css.push_str(".hover:bold:hover { font-weight: bold; }\n");

    // Example focus states
    css.push_str(".focus:underline:focus { text-decoration: underline; }\n");
    css.push_str(".focus:italic:focus { font-style: italic; }\n");

    css
}

pub fn generate_hover_focus_for_utilities() -> String {
    let utilities = vec![
        "bg", "text", "p", "m", "font", "border", "shadow", "w", "h", "max-w", "max-h", "font-size", "font-weight"
    ];

    let mut css = String::new();

    for utility in utilities {
        css.push_str(&format!(".hover:{} {{ :hover; }}\n", utility));
        css.push_str(&format!(".focus:{} {{ :focus; }}\n", utility));
    }

    css
}

pub fn generate_dark_mode_utilities() -> String {
    let utilities = vec![
        ("bg", "black"),
        ("text", "white"),
        ("border", "gray"),
        // Add any other utilities as needed.
    ];

    let mut css = String::new();

    for (utility, color) in utilities {
        css.push_str(&format!(".dark:{}-{} {{ background-color: {}; }}\n", utility, color, color));
        css.push_str(&format!(".dark:{}-{} {{ color: {}; }}\n", utility, color, color));
    }

    css
}

pub fn generate_active_state_utilities() -> String {
    let utilities = vec![
        ("bg", "red-500"),
        ("text", "gray-700"),
        ("border", "blue-500"),
        // Add more utilities if needed
    ];

    let mut css = String::new();

    for (utility, color) in utilities {
        css.push_str(&format!(".active:{}-{} {{ background-color: {}; }}\n", utility, color, color));
        css.push_str(&format!(".active:{}-{} {{ color: {}; }}\n", utility, color, color));
    }

    css
}
