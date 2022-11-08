use qrcode::QrCode;
use image::Luma;

fn main() {
println!("Testing qrcode...");


// Encode some data into bits.
let code = QrCode::new(b"01234567").unwrap();

// Render the bits into an image.
let image = code.render::<Luma<u8>>().build();

// Save the image.
image.save("/tmp/qrcode.png").unwrap();

// You can also render it into a string.
let string = code.render()
    .light_color(' ')
    .dark_color('#')
    .build();
println!("{}", string);

}
