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
