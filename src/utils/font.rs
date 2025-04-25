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
