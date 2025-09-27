use crate::{Frame, Style};
use crate::bitmap::Bitmap;

#[cfg(feature = "std")]
use std::io::Write;
#[cfg(not(feature = "std"))]
use core::fmt::Write;

struct AnsiPosition {
    line: usize,
    column: usize
}

/// Write a `Bitmap` conversion either to a buffer or terminal.
pub struct Writer {
    _style: Style,
    _frame: Frame,
    _ansi_position: Option<AnsiPosition>,
    _ansi_position_restore: bool,
    _use_be: bool,
    _byte_aligned: bool
}

/// ```no_std``` compatible unless otherwise noted.
impl Writer {
    /// Construct a writer to write or print bitmaps later with defaults settings.
    /// 
    /// The resulting writer instance can be chained for further configurations.
    /// 
    /// ```
    /// let p = bitmap_writer::Writer::new()
    ///     .style(bitmap_writer::Style::UnicodeBlock2x2)
    ///     /* ... */
    ///    .be(true);
    /// ```
    ///    
    /// # Returns
    /// - Writer instance
    pub fn new() -> Writer {
        return Writer {
            _style: Style::UnicodeBlock1x2,
            _frame: Frame::NoFrame,
            _ansi_position: None,
            _ansi_position_restore: false,
            _use_be: false,
            _byte_aligned: false
        }
    }
    
    /// The style determines which characters are used to write the bitmap itself.
    /// 
    /// Some styles work better or worse depending on the Unicode-font used to
    /// view the result.
    /// 
    /// # Arguments
    /// - `style`: See `bitmap_writer::Style`
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn style(&mut self, style: Style) -> &mut Self {
        self._style = style;        
        return self;
    }

    /// The frame determines which characters are used to create the frame around
    /// the bitmap.
    /// 
    /// Some frame work better or worse depending on the Unicode-font used to
    /// view the result.
    /// 
    /// # Arguments
    /// - `style`: See `bitmap_writer::Frame`    
    ///    
    /// # Returns
    /// - Reference to writer.
    pub fn frame(&mut self, frame: Frame) -> &mut Self {
        self._frame = frame;        
        return self;
    }

    /// Set a specific line and column the bitmap will be written to.
    /// 
    /// It uses ANSI commands which might not be supported by all terminals
    /// used to view the result.
    /// 
    /// The ANSI commands will not written instantly but are included with
    /// the resulting character stream at the end of bitmap conversion.
    ///
    /// # Arguments
    /// - `line`: Line to set the terminal cursor to before writing the bitmap
    /// - `column`: Column to set the terminal cursor to before writing the bitmap
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn ansi_position(&mut self, line: usize, column: usize) -> &mut Self {
        self._ansi_position = Some(AnsiPosition { line: line, column: column });
        return self;
    }

    /// Removes the line and column previously set by `ansi_position(..)`,
    /// making the writer reuseable for non-ANSI use cases.
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn clear_ansi_position(&mut self) -> &mut Self {
        self._ansi_position = None;
        return self;
    }

    /// Determines whether ANSI save-and-restore-cursor commands are used
    /// to reset the terminal cursor back to the position before the bitmap
    /// was written.
    /// 
    /// This allows to overwrite the bitmap in place, updating the same screen
    /// area with a bitmap of equal size.
    /// 
    /// When activated `ansi_position(..)` is ignored.
    /// 
    /// # Arguments
    /// - `state`: Set to `true` to use ANSI save-and-restore.
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn ansi_position_restore(&mut self, state: bool) -> &mut Self {
        self._ansi_position_restore = state;        
        return self;
    }

    /// Whether to use big-endian or little-endian encoding when decoding
    /// the bytes of the bitmap.
    /// 
    /// By default the writer used little-endian.
    /// 
    /// Little-endian encoding is used when defining bitmaps in source code:
    /// 
    /// ```
    /// let pixels: &[u8] = &[
    ///     0b10000000,
    ///     0b11000000,
    ///     0b11100000,
    /// ];
    /// ```
    /// 
    /// Hardware buffers such as DAC framebuffers in display driver chips
    /// or textures in VRAM mostly use big-endian.
    /// 
    /// Choosing the wrong endianess will result in 8-pixel wide mirrored
    /// columns in the writer output.
    /// 
    /// # Arguments
    /// - `state`: Set to `true` to use big-endian byte encoding.
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn be(&mut self, state: bool) -> &mut Self {
        self._use_be = state;        
        return self;
    }

    /// Whether to bitmap is has byte-aligned pixel rows. This is only relevant
    /// when the bitmap's width is not a multiple of 8.
    /// 
    /// When set to `true` the bitmap is byte-aligned, meaning that the remaining
    /// bits of each line that are outside of the bitmap's width are ignored.
    /// The next row starts by using the first bit of the next byte in the bitmap.
    /// 
    /// When set to `false` the bitmap is expected to be non-byte-aligned. The first
    /// pixel of the next line uses the first remaining pixel of the last byte of
    /// the last line.
    ///     
    /// 
    /// # Arguments
    /// - `state`: Sets the byte-alignment of the bitmap.
    /// 
    /// # Returns
    /// - Reference to writer.
    pub fn byte_aligned(&mut self, state: bool) -> &mut Self {
        self._byte_aligned = state;        
        return self;
    }

    /// After setting up the writer, finally convert the bitmap into a string of
    /// characters that can be printed out to terminal or transfered to a display
    /// device.
    /// 
    /// # Arguments
    /// - `writer`: A writer implementing the `Write` trait. It will contain the result of the bitmap conversion.
    /// - `bitmap`: The bitmap to write into the writer. It contains both size and pixel contents.    
    pub fn write(&self, writer: &mut impl Write, bitmap: &Bitmap){                    
        self._write_bitmap(writer, bitmap._pixels, bitmap._w, bitmap._h, &self._style, &[], &self._ansi_position, self._ansi_position_restore, &self._frame);        
    }

    fn _write_bitmap(&self, w: &mut impl Write, bitmap: &[u8], width: usize, height: usize, style_ref: &Style, blocks: &[char], ansi_position: &Option<AnsiPosition>, ansi_position_reset: bool, frame_ref: &Frame){    
        let mut line: usize = 0;
        
        let nine_patch: [char;9];
        match frame_ref {        
            Frame::ASCIIFrame =>  { nine_patch = ['.', '-', '.', '|', ' ', '|', '\'', '-', '\'']; }
            Frame::UnicodeBoldFrame =>   { nine_patch = ['┏', '━', '┓', '┃', ' ', '┃', '┗', '━', '┛']; }
            Frame::UnicodeDoubleUFrame => { nine_patch = ['╔', '═', '╗', '║', ' ', '║', '╚', '═', '╝']; }
            Frame::UnicodeBlockFrame =>  { nine_patch = ['▞', '▀', '▚', '▌', ' ', '▐', '▚', '▄', '▞']; }
            Frame::UnicodeShadeFrame =>  { nine_patch = ['🮞', '🮐', '🮟', '🮐', ' ', '🮐', '🮝', '🮐', '🮜']; }
            _ =>                  { nine_patch = ['┌', '─', '┐', '│', ' ', '│', '└', '─', '┘']; }
        }             
        
        if ansi_position_reset {
            let Ok(_v) = self._ansi_store_position(w) else { return };
        }else
        if ansi_position.is_some() {
            let Ok(_v) = self._ansi_set_position(w, ansi_position.as_ref().unwrap(), line) else { return };  
        }      
        
        struct StyleSettings <'a>{
            blocks: &'a[char],
            w: usize,
            h: usize,
        }
        let style: StyleSettings = match style_ref {            
            Style::ASCII1x1(char) => { StyleSettings { w: 1, h: 1, blocks: &[' ', *char] } }
            Style::UnicodeBlock1x1 => { StyleSettings { w: 1, h: 1, blocks: &[' ', '█'] } }
            Style::UnicodeBlock1x2 => { StyleSettings { w: 1, h: 2, blocks: &[' ', '▀', '▄', '█'] } }
            Style::UnicodeBlock2x2 => { StyleSettings { w: 2, h: 2, blocks: &[
                ' ',  '▘', '▝', '▀', '▖', '▋', '▞', '▛',
                '▗', '▚', '▐' , '▜', '▄', '▙', '▟', '█'
            ] } }
            Style::UnicodeSextant1x3 => { StyleSettings { w: 1, h: 3, blocks: &[' ',  '🬀', '🬃', '🬄', '🬏', '🬐', '🬓', '▋' ] } }
            Style::UnicodeSectant2x3 => { StyleSettings { w: 2, h: 3, blocks: &[
                ' ',  '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆',
                '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎',
                '🬏', '🬐', '🬑', '🬒', '🬓', '▋', '🬔', '🬕',
                '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝',
                '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥',
                '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬',
                '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴',
                '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '🮋'
            ] } }
        };

        match frame_ref {
            Frame::NoFrame => { }
            _ => {
                let Ok(_v) = write!(w, "{}", nine_patch[0]) else { return };
                for _i in 0..((width + style.w - 1) / style.w) { let Ok(_v) = write!(w, "{}", nine_patch[1]) else { return } ; }
                let Ok(_v) = writeln!(w, "{}", nine_patch[2]) else { return };
                
                line += 1;
            }
        }
        
        for y in (0..height).step_by(style.h) {
            
            if ansi_position_reset == false && ansi_position.is_some() {
               let Ok(_v) = self._ansi_set_position(w, ansi_position.as_ref().unwrap(), line) else { return };
            }
            
            match frame_ref {
                Frame::NoFrame => { }
                _ => { let Ok(_v) = write!(w, "{}", nine_patch[3]) else { return }; }            
            }
            
            for x in (0..width).step_by(style.w) {
                let mut block: usize = 0;
                for yy in 0..style.h {
                    for xx in 0..style.w {
                        let w = if self._byte_aligned { (width + 7) / 8 * 8 } else { width };
                        let byte: usize = ((y + yy) * w + x + xx) / 8;
                        let mask: u8 = 1 << ((if self._use_be { (x + xx) % 8 } else { 7 - (x + xx) % 8 } ));
                        if byte < bitmap.len() && bitmap[byte] & mask != 0 { block |= 1 << (yy * style.w + xx); }                                
                    }
                }                                                     
               let Ok(_v) = write!(w, "{}", if block < blocks.len() { blocks[block] } else if block < style.blocks.len() { style.blocks[block] } else { ' ' }) else { return };                       
            }
            
            match frame_ref {
                Frame::NoFrame => { if y < height { let Ok(_v) = writeln!(w, "") else { return }; } }
                _ => { let Ok(_v) = writeln!(w, "{}", nine_patch[5]) else { return }; }
            }

            line += 1;
        }   

        if ansi_position_reset == false && ansi_position.is_some() {
            let Ok(_v) = self._ansi_set_position(w, ansi_position.as_ref().unwrap(), line) else { return };
        }

        match frame_ref {
            Frame::NoFrame => { }
            _ => {
                let Ok(_v) = write!(w, "{}", nine_patch[6]) else { return };
                for _i in 0..((width + style.w - 1) / style.w) { let Ok(_v) =  write!(w, "{}", nine_patch[7]) else { return }; }
                let Ok(_v) = writeln!(w, "{}", nine_patch[8]) else { return };                
            }                    
        }

        if ansi_position_reset {
           let Ok(_v) = self._ansi_restore_position(w) else { return  };
        }

        #[cfg(not(feature = "std"))]
        return;
        #[cfg(feature = "std")]
        let Ok(_v) = w.flush() else { return; };        
    }

    fn _ansi_set_position(&self, w: &mut impl Write, pos: &AnsiPosition, line: usize) -> Result<(), impl core::error::Error> {
       return write!(w, "\x1b[{};{}H", pos.line + line, pos.column);
    }

    fn _ansi_store_position(&self, w: &mut impl Write) -> Result<(), impl core::error::Error> {
        return write!(w, "\x1b[s");
    }

    fn _ansi_restore_position(&self, w: &mut impl Write) -> Result<(), impl core::error::Error> {
        return write!(w, "\x1b[u");
    }
}

#[cfg(feature = "std")]
use io_streams::StreamWriter;

/// Requires `features=["std"]` in ```cargo.toml```.
#[cfg(feature = "std")]
impl Writer {
    /// After setting up the writer, print the bitmap out to terminal.
    /// 
    /// **stdout** or it's non-Linux equivalent is used for output.    
    /// 
    /// # Argument
    /// - `bitmap`: The bitmap to write into the writer. It contains both size and pixel contents.    
    pub fn print(&self, image: &Bitmap){                        
        self._write_bitmap(&mut StreamWriter::stdout().unwrap(), image._pixels, image._w, image._h, &self._style, &[], &self._ansi_position, self._ansi_position_restore, &self._frame);        
    }
}