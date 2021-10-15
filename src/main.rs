use qrcode::QrCode;
use image::Luma;
use colored::*;

// fn output() {
//     let mut line = String::new();
//     println!("Enter Output: \n");
//     let b1 = std::io::stdin().read_line(&mut line).unwrap();
//     println!("\nYou Entered The Output: {}", line.green());
//     println!("Number Of Bytes Read: {}", b1);
// }

// fn qr_data() {
//     let code = QrCode::new(b"H3LL0").unwrap();
//     let string = code.render::<char>().quiet_zone(false).module_dimensions(2, 1).build();
//     println!("{}", string);
// }

fn qr_save() {
    // Encode some data into bits.
    let code = QrCode::new(b"HELLO").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("./QR/QRcode.png").unwrap();
}

fn main() {
    // qr_data();
    // qr_save();
}
