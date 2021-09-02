use qrcode::QrCode;
use image::Luma;

// fn output() {
//     let mut line = String::new();
//     println!("Enter Output: \n");
//     let b1 = std::io::stdin().read_line(&mut line).unwrap();
//     println!("\nHello, here is your output: {}", line);
//     println!("Number of Bytes Read: {}", b1);
// }

fn qr_data() {
    // Encode some data into bits.
    let code = QrCode::new(" ").unwrap();

    
    
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("img/qrcode3.png").unwrap();
}

fn main() {
    qr_data();
}
