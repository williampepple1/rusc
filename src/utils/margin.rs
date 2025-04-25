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
