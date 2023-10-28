//! Parse and convert Quill's Deltas to PDF documents.
//!
//! Calling `DeltaPdf::new()` will parse the data according to the
//! [Quill Delta specification](https://quilljs.com/docs/delta/) and return an error if the delta
//! is invalid or has unsupported attributes.
//!
//! The following attributes are supported:
//! - bold
//! - italic
//! - header
//! - image
//!
//! Only inserts are rendered. Deletes and retains are parsed but ignored.
//!
//! ## Example Usage
//!
//! ```
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let default_font = genpdf::fonts::from_files("./fonts", "Inter", None)?;
//!
//!     let mut doc = genpdf::Document::new(default_font);
//!
//!     let test = std::fs::read_to_string("./test.json")?;
//!     let mut delta = quill_delta_pdf::DeltaPdf::new(test)?;
//!     delta.set_image_dir("./images".into());
//!     delta.write_to_pdf(&mut doc)?;
//!
//!     doc.render_to_file("test.pdf")?;
//!     Ok(())
//! }
//! ```
//!
//! This library makes use of genpdf. If you want to customize the look of the PDF file feel free
//! to take a look at their [documentation](https://docs.rs/genpdf/latest/genpdf/index.html)

mod delta;

use std::path::PathBuf;

use delta::{Attribute, Change, Delta, DeltaType};
use genpdf::{
    elements::{Image, Paragraph},
    style::{Style, StyledString},
    Document, Element,
};

#[derive(Debug)]
/// Error type for DeltaPdf
pub enum DeltaPdfError {
    ImageUrlError,
    ImagePathNotSet,
    PdfError(genpdf::error::Error),
}

impl std::error::Error for DeltaPdfError {}

impl std::fmt::Display for DeltaPdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeltaPdfError::ImageUrlError => write!(f, "The image url could not be parsed"),
            DeltaPdfError::ImagePathNotSet => write!(
                f,
                "Parsed Delta had an image but the image directory is not set."
            ),
            DeltaPdfError::PdfError(e) => write!(f, "{}", e),
        }
    }
}

impl From<genpdf::error::Error> for DeltaPdfError {
    fn from(err: genpdf::error::Error) -> Self {
        DeltaPdfError::PdfError(err)
    }
}

enum PdfElement {
    String(StyledString),
    Image(Image),
}

/// Struct that holds the parsed Delta.
pub struct DeltaPdf {
    delta: Delta,
    images_path: Option<PathBuf>,
}

impl DeltaPdf {
    /// Parse a Quill Delta.
    pub fn new(delta: String) -> serde_json::Result<DeltaPdf> {
        let delta_serialized: Delta = serde_json::from_str(&delta)?;
        Ok(Self {
            delta: delta_serialized,
            images_path: None,
        })
    }

    /// Set the location of where images are located.
    /// The last segment of the image url will be used as the image name.
    /// If the URL is: `https://example.com/image.png` then
    /// the library will try to get `image.png` from the image directory.
    pub fn set_image_dir(&mut self, path: PathBuf) {
        self.images_path = Some(path);
    }

    /// Set the heading font size for the previous string
    fn set_heading(strings: &mut [PdfElement], font_size: u8) {
        // For some reason the heading is applied to the newline character that follows the heading
        // So we need to get the previous string to test the font size
        if let Some(PdfElement::String(last)) = strings.last_mut() {
            last.style.set_font_size(font_size);
        }
    }

    /// Write the parsed Delta to a PDF document
    pub fn write_to_pdf(&self, document: &mut Document) -> Result<(), DeltaPdfError> {
        let mut pdf_elements: Vec<PdfElement> = Vec::new();

        for op in &self.delta.ops {
            let delta_type = match &op.change {
                Change::Insert(x) | Change::Delete(x) | Change::Retain(x) => x,
            };

            match delta_type {
                DeltaType::String(text) => {
                    let mut style = Style::new();
                    if let Some(attributes) = &op.attributes {
                        for attribute in attributes {
                            match attribute {
                                Attribute::Bold(true) => style.set_bold(),
                                Attribute::Italic(true) => style.set_italic(),
                                Attribute::Header(1) => Self::set_heading(&mut pdf_elements, 18),
                                Attribute::Header(2) => Self::set_heading(&mut pdf_elements, 16),
                                _ => (),
                            }
                        }
                    }
                    pdf_elements.push(PdfElement::String(StyledString::new(text, style)));
                }
                DeltaType::Image(image) => {
                    let image_name = image
                        .image
                        .path_segments()
                        .ok_or(DeltaPdfError::ImageUrlError)?
                        .last()
                        .ok_or(DeltaPdfError::ImageUrlError)?;
                    let full_path = self
                        .images_path
                        .as_ref()
                        .ok_or(DeltaPdfError::ImagePathNotSet)?
                        .join(image_name);
                    let image = Image::from_path(full_path)?;
                    pdf_elements.push(PdfElement::Image(image));
                }
            }
        }

        let mut paragraph = Paragraph::default();
        for element in pdf_elements {
            match element {
                PdfElement::String(string) => {
                    let mut lines = string.s.split('\n');

                    // The first line will be pushed to an existing paragraph
                    let line = lines.next().unwrap_or_default();
                    paragraph.push_styled(line, string.style);

                    // If we have more than one line then create a new paragraph for each
                    for line in lines {
                        // Push the current paragraph before we override it
                        document.push(paragraph);
                        paragraph = Paragraph::default();
                        paragraph.push_styled(line, string.style);
                    }
                }
                PdfElement::Image(image) => {
                    document.push(image.padded(1));
                }
            }
        }
        document.push(paragraph);
        Ok(())
    }
}
