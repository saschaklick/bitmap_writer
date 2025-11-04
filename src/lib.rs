#![cfg_attr(not(feature = "std"), no_std)]

//! Convert monochrome pixel data to text for displaying in terminal or
//! transfer.


/// The style determines the character set used to convert the bitmap.
/// 
/// The style's name denotes how many pixels can be displayed per character.
/// `1x1` indicates that each character equals one pixel.
/// `2x3` encodes a total of six pixels in one character.
/// 
/// The quality of the output is highly depended on the font used in the
/// terminal that displays the result.
/// 
/// Most modern fonts are taller than they are wide. To achieve a natural
/// looking width-to-height ratio, use `1x2` or `1x3` fonts.
/// 
/// The `ASCII` style is the most compatible and easily transferable.
///
/// The `Unicode` styles can fail to display correctly if the font used in
/// the terminal has missing or misaligned characters.
/// 
/// The `2x3` style is the most space-efficient style but fails on most
/// Unicode fonts due to misaligned character shapes.
/// 
/// On the `ASCII1x1`, a style character must be provided which will be used
/// to display the set pixels, while an unset pixel is represented by a blank
/// space.
#[repr(usize)]
pub enum Style {
    ASCII1x1(char) = 0,
    UnicodeBlock1x1 = 1,
    UnicodeBlock1x2 = 2,
    UnicodeBlock2x2 = 3,
    UnicodeSextant1x3 = 4,
    UnicodeSextant2x3 = 5
}

/// A decorative frame can be drawn around the bitmap to better indicate
/// where the edges of the bitmap are located.
/// 
/// Much like with the styles, the results are depended on the font used.
/// The `ASCII` frame should always work. The `Unicode` frames are nicer
/// but support is more spotty.
#[repr(usize)]
pub enum Frame {
    NoFrame = 0,
    ASCIIFrame = 1,
    UnicodeFrame = 2,
    UnicodeBoldFrame = 3,
    UnicodeDoubleUFrame = 4,
    UnicodeBlockFrame = 5,
    UnicodeShadeFrame = 6
}


mod bitmap;
pub use self::bitmap::Bitmap;

mod writer;
pub use self::writer::Writer;