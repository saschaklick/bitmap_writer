use bitmap_writer::{Writer, Bitmap, Frame, Style};

use std::{thread, time};
use std::io::BufWriter;

fn main() {
    let bitmap_0 = Bitmap::new(16, 8, &[
        0b00010000, 0b00001000,
        0b00111111, 0b11111100,
        0b01111111, 0b11111110,
        0b11111111, 0b11111111,
        0b01111111, 0b11111110,
        0b00000111, 0b11100000,
        0b11001111, 0b11110000,
        0b01111111, 0b11111000
    ]);

    let bitmap_1 = Bitmap::new(16, 8, &[
        0b00000000, 0b00000000,
        0b00010000, 0b00001000,
        0b00111111, 0b11111100,
        0b01111111, 0b11111110,
        0b11111111, 0b11111111,
        0b01111111, 0b11111110,
        0b11001111, 0b11110000,
        0b01111111, 0b11111000
    ]);

    let mut w = Writer::new();
    w
    .style(Style::UnicodeBlock2x2)
    .ansi_position_restore(true);
    
    let mut cnt: usize = 0;
    loop {
        w.frame(if cnt % 2 == 0 { Frame::UnicodeFrame } else { Frame::UnicodeDoubleUFrame } );
        
        w.print(if cnt % 2 == 0 { &bitmap_0 } else { &bitmap_1 });        

        cnt += 1;

        if cnt > 16 {
            break;
        }

        thread::sleep(time::Duration::from_millis(125));        
    }
        
}

