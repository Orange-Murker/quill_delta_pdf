use quill_delta_pdf::DeltaPdf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_font = genpdf::fonts::from_files("./fonts", "Inter", None)
        .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);

    let test = fs::read_to_string("./test.json")?;
    let mut delta = DeltaPdf::new(test)?;
    delta.set_image_dir("./images".into());
    delta.write_to_pdf(&mut doc)?;

    doc.render_to_file("test.pdf")?;
    Ok(())
}
