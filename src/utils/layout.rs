pub fn generate_height_utilities() -> String {
    let heights = vec![0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64];
    let mut css = String::new();

    for val in heights {
        let px = val * 4;
        css.push_str(&format!(".h-{} {{ height: {}px; }}\n", px, px));
    }

    css
}

pub fn generate_max_height_utilities() -> String {
    let max_heights = vec![0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64];
    let mut css = String::new();

    for val in max_heights {
        let px = val * 4;
        css.push_str(&format!(".max-h-{} {{ max-height: {}px; }}\n", px, px));
    }

    css
}


pub fn generate_width_utilities() -> String {
    let sizes = vec![0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 20, 24, 32, 40, 48, 56, 64];
    let mut css = String::new();

    for size in sizes {
        let px = size * 4;
        css.push_str(&format!(".w-{} {{ width: {}px; }}\n", px, px));
    }

    css
}

pub fn generate_max_width_utilities() -> String {
    let sizes = vec![
        ("xs", "20rem"),
        ("sm", "24rem"),
        ("md", "28rem"),
        ("lg", "32rem"),
        ("xl", "36rem"),
        ("2xl", "42rem"),
        ("3xl", "48rem"),
        ("4xl", "56rem"),
        ("5xl", "64rem"),
        ("full", "100%"),
    ];

    let mut css = String::new();

    for (name, value) in sizes {
        css.push_str(&format!(".max-w-{} {{ max-width: {}; }}\n", name, value));
    }

    css
}
