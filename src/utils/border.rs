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
