
use tesseract::{Tesseract, plumbing::TessBaseApiGetUtf8TextError};

fn main() {
    let text = process_image("test/01.PNG").unwrap();
    println!("result: {}", text);
}


fn process_image(
    file_name: &str,
) -> Result<String, TessBaseApiGetUtf8TextError> {
    let lang = "rus";

    let tesseract = Tesseract::new(None, Some(&lang))?;

    let text = tesseract
        .set_image(file_name)
        // .set_image_from_mem(&buf.as_slice())
        .unwrap()
        .recognize()
        .unwrap()
        .get_text()?;
        Ok(text)

}