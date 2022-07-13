
use tesseract::{Tesseract, TesseractError};

fn main() {
    let text = process_image("test/01.PNG").unwrap();
    println!("result: {}", text);
}


fn process_image(
    file_name: &str,
) -> Result<String, TesseractError> {
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