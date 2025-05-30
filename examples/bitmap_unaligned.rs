use bitmap_writer::{Writer, Bitmap, Frame, Style};

fn main() {
    let bitmap = Bitmap::new(19, 8, &[
        0b00010000, 0b00001000, 0b00100000,
        0b00111111, 0b11111100, 0b01000000,
        0b01111111, 0b11111110, 0b00100000,
        0b11111111, 0b11111111, 0b01000000,
        0b01111111, 0b11111110, 0b00100000,       
        0b00000111, 0b11100000, 0b01000000,
        0b11001111, 0b11110000, 0b00100000,
        0b01111111, 0b11111000, 0b01000000,
    ]);

    let mut w = Writer::new();
    w.frame(Frame::UnicodeDoubleUFrame).byte_aligned(true);

    println!("ASCII1x1");
    w.style(Style::ASCII1x1('@')).print(&bitmap);    
    println!("UnicodeBlock1x1");
    w.style(Style::UnicodeBlock1x1).print(&bitmap);    
    println!("UnicodeBlock1x2");
    w.style(Style::UnicodeBlock1x2).print(&bitmap);
    println!("UnicodeSextant1x3");
    w.style(Style::UnicodeSextant1x3).print(&bitmap);    
    println!("UnicodeBlock2x2");
    w.style(Style::UnicodeBlock2x2).print(&bitmap);    
    println!("UnicodeSextant2x3");
    w.style(Style::UnicodeSectant2x3).print(&bitmap); 
}

