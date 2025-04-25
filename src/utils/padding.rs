
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