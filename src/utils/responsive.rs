pub fn wrap_with_responsive_variants(css: &str) -> String {
    let breakpoints = vec![
        ("sm", "640px"),
        ("md", "768px"),
        ("lg", "1024px"),
        ("xl", "1280px"),
    ];

    let mut responsive_css = String::new();

    for (prefix, min_width) in breakpoints {
        responsive_css.push_str(&format!(
            "@media (min-width: {}) {{\n",
            min_width
        ));

        // For each class, prefix it inside media query
        for line in css.lines() {
            if line.starts_with('.') {
                let updated_line = line.replacen('.', &format!(".{}:", prefix), 1);
                responsive_css.push_str(&format!("    {}\n", updated_line));
            }
        }

        responsive_css.push_str("}\n");
    }

    responsive_css
}
