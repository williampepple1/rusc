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

pub fn generate_flex_direction_utilities() -> String {
    let directions = vec![
        ("row", "row"),
        ("col", "column"),
        ("row-reverse", "row-reverse"),
        ("col-reverse", "column-reverse"),
    ];

    let mut css = String::new();

    for (class, value) in directions {
        css.push_str(&format!(".flex-{} {{ flex-direction: {}; }}\n", class, value));
    }

    css
}

pub fn generate_justify_content_utilities() -> String {
    let alignments = vec![
        ("center", "center"),
        ("start", "flex-start"),
        ("end", "flex-end"),
        ("between", "space-between"),
        ("around", "space-around"),
        ("evenly", "space-evenly"),
    ];

    let mut css = String::new();

    for (class, value) in alignments {
        css.push_str(&format!(".justify-{} {{ justify-content: {}; }}\n", class, value));
    }

    css
}
