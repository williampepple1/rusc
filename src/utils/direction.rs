pub fn generate_display_utilities() -> String {
    let display_types = vec![
        ("block", "block"),
        ("inline-block", "inline-block"),
        ("flex", "flex"),
        ("grid", "grid"),
        ("inline-flex", "inline-flex"),
        ("inline-grid", "inline-grid"),
    ];

    let mut css = String::new();

    for (class, value) in display_types {
        css.push_str(&format!(".{} {{ display: {}; }}\n", class, value));
    }

    css
}
