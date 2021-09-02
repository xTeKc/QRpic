use qrcode::QrCode;
use image::Luma;

#[macro_use]
extern crate fstrings;

fn output() {
    let mut line = String::new();
    println!("Enter Output: \n");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("\nHello, here is your output: {}", line);
    println!("Number of Bytes Read: {}", b1);
}

fn main() {
    let output_data = output();

    // Encode some data into bits.
    let code = QrCode::new(" ").unwrap();

    
    
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("img/qrcode3.png").unwrap();
}