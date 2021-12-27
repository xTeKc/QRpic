use qrcode::QrCode;

// create a function that takes a string and returns a qrcode image as a png file with the string as the content
fn create_qr_image(content: String) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, qrcode::QrCodeError> {
    let code = QrCode::new(content)?;
    let mut image = code.to_image(10)?;
    image.save("qr.png")?;
    Ok(image)
}

// create a function that takes a string and returns a qrcode image as a png file with the string as the content
fn create_qr_image_from_path(path: String) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, qrcode::QrCodeError> {
    let code = QrCode::new(path)?;
    let mut image = code.to_image(10)?;
    image.save("qr.png")?;
    Ok(image)
}

// create a function that takes a string and returns a qrcode image as a png file with the string as the content
fn create_qr_image_from_file(path: &str) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, qrcode::QrCodeError> {
    let code = QrCode::new(path)?;
    let mut image = code.to_image(10)?;
    image.save("qr.png")?;
    Ok(image)
}

fn main() {
    let path = "qr.png";
    let content = ""
        .to_string();
}