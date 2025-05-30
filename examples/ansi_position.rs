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

    println!("\x1b[2J");

    let mut w = Writer::new();
    w.style(Style::UnicodeBlock2x2).frame(Frame::UnicodeFrame);
    
    w.ansi_position(6, 12).print(&bitmap);    
    w.ansi_position(12, 24).print(&bitmap);    
    w.ansi_position(6, 36).print(&bitmap);
    w.ansi_position(12, 48).print(&bitmap);
    w.clear_ansi_position().print(&bitmap);
}

