fn main() {
    let css = rusc::generate_css(); // From your lib.rs
    std::fs::create_dir_all("dist").unwrap();
    std::fs::write("dist/output.css", css).expect("Failed to write dist/output.css");
    println!("✅ dist/output.css generated!");
}
