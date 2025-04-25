fn main() {
    let css = russ::generate_css(); // From your lib.rs
    std::fs::write("output.css", css).expect("Unable to write CSS file");
    println!("✅ CSS generated at output.css");
}
