/// Demonstrates the practical use of displaying a digital clock with a custom
/// bitmap font.
/// 
/// Example output:
/// 
/// ```
/// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
/// ┃  ▄███  ██████▄   ▄▄▄      ▄███ ███████   ▄▄▄   ███████ ██████▄ ┃
/// ┃  ▀███  ▀▀▀███▀   ███    ▄██▀██ ▀▀▀▀███   ███   ▀▀▀▀███ ▀▀▀███▀ ┃
/// ┃   ███   ▄██▀     ▄▄▄   ███▄▄██▄   ▀▀█▄   ▄▄▄      ▀▀█▄  ▄██▀   ┃
/// ┃   ▀▀▀  ▀▀▀▀      ▀▀▀       ▀▀▀ ▀▀▀▀▀▀    ▀▀▀   ▀▀▀▀▀▀  ▀▀▀▀    ┃
/// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
/// ```

use bitmap_writer::{Writer, Bitmap, Frame, Style};

use chrono::{Local, Timelike};

fn main() {    
    // Font source: https://damieng.com/typography/zx-origins/zx-baveuse/ by Raymond Larabie 2021
    const CHAR_AMOUNT: usize = 11;
    const CHARS: &[u8; CHAR_AMOUNT * 8] = &[
        0b01111100, 0b00011100, 0b11111100, 0b11111110, 0b00001110, 0b11111110, 0b01111110, 0b11111100, 0b01111100, 0b01111100,  0b00000000,
        0b11111110, 0b00111100, 0b11111110, 0b11111110, 0b00011110, 0b11111110, 0b11111110, 0b11111110, 0b11111110, 0b11111110,  0b00111000,
        0b11111110, 0b00111100, 0b11111110, 0b11111110, 0b00111110, 0b11111110, 0b11111110, 0b11111110, 0b11111110, 0b11111110,  0b00111000,
        0b11100110, 0b00011100, 0b00001110, 0b00001110, 0b01110110, 0b11000000, 0b11100000, 0b00011100, 0b11100110, 0b11000110,  0b00111000,
        0b11100110, 0b00011100, 0b01111100, 0b00011100, 0b11100110, 0b11111100, 0b11111110, 0b00111000, 0b01111100, 0b01111110,  0b00000000,
        0b11100110, 0b00011100, 0b11000000, 0b00000110, 0b11111111, 0b00000010, 0b11000010, 0b01110000, 0b11001110, 0b00001110,  0b00111000,
        0b01111100, 0b00011100, 0b11111110, 0b11111100, 0b00001110, 0b11111100, 0b01111100, 0b11110000, 0b01111100, 0b11111100,  0b00111000,
        0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  0b00000000
    ];
    
    let now = Local::now();
    
    let digits: &[u8] = &[
        (now.hour() / 10) as u8 % 10, (now.hour() as u8) % 10,
        (now.minute() / 10) as u8 % 10, (now.minute() as u8) % 10,
        (now.second() / 10) as u8 % 10, (now.second() as u8) % 10,
    ];

    let mut pixels: [u8; 8 * 8] = [0; 8 * 8];
    for i in 0..8 {
        let char = 
            if i == 2 || i == 5 { 10 } else
            if i < 2 { digits[i] } else
            if i > 2 && i < 5 { digits[i - 1] } else
            if i > 5 { digits[i - 2] } else
            { 0 };         
        for ii in 0..8 {
            pixels[(i + ii * 8) as usize] = CHARS[(char as usize + ii * CHAR_AMOUNT) as usize];
        }        
    }

    let bitmap = Bitmap::new(8 * 8, 8, &pixels);

    let mut w = Writer::new();
    w.style(Style::UnicodeBlock1x2).frame(Frame::UnicodeBoldFrame).print(&bitmap);
    
}

