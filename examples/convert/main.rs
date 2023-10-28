fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_font = genpdf::fonts::from_files("./fonts", "Inter", None)?;

    let mut doc = genpdf::Document::new(default_font);

    let test = std::fs::read_to_string("./test.json")?;
    let mut delta = quill_delta_pdf::DeltaPdf::new(test)?;
    delta.set_image_dir("./images".into());
    delta.write_to_pdf(&mut doc)?;

    doc.render_to_file("test.pdf")?;
    Ok(())
}
