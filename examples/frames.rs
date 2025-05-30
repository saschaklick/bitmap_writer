use bitmap_writer::{Writer, Bitmap, Frame, Style};

fn main() {
    let bitmap = Bitmap::new(16, 8, &[
        0b00010000, 0b00001000,
        0b00111111, 0b11111100,
        0b01111111, 0b11111110,
        0b11111111, 0b11111111,
        0b01111111, 0b11111110,
        0b00000111, 0b11100000,
        0b11001111, 0b11110000,
        0b01111111, 0b11111000
    ]);

    let mut w = Writer::new();
    w.style(Style::UnicodeBlock2x2);

    println!("AsciiFrame");
    w.frame(Frame::ASCIIFrame).print(&bitmap);
    println!("UnicodeFrame");
    w.frame(Frame::UnicodeFrame).print(&bitmap);
    println!("UnicodeBoldFrame");
    w.frame(Frame::UnicodeBoldFrame).print(&bitmap);
    println!("UnicodeDoubleFrame");
    w.frame(Frame::UnicodeDoubleUFrame).print(&bitmap);
    println!("UnicodeBlockFrame");
    w.frame(Frame::UnicodeBlockFrame).print(&bitmap);
    println!("UnicodeShadeFrame");
    w.frame(Frame::UnicodeShadeFrame).print(&bitmap);
    
}

