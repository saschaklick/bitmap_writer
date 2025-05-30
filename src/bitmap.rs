/// Define a bitmap by width, height and an `[u8]` array on pixel data.
pub struct Bitmap <'a> {
    pub(crate) _w: usize,
    pub(crate) _h: usize,
    pub(crate) _pixels: &'a [u8]
}

impl Bitmap <'_> {
    /// Construct a new bitmap instance.
    ///    
    /// # Arguments
    /// - `width`: Pixel width.
    /// - `height`: Pixel height.
    /// - `pixels`: Array of bytes that contains the bitmap data.
    ///
    /// # Returns
    /// - Bitmap instance.
    pub fn new(width: usize, height: usize, pixels: &[u8]) -> Bitmap {
        return Bitmap { _w: width, _h: height, _pixels: pixels };
    }

    /// Get the bitmap's width.
    ///    
    /// # Returns
    /// - Pixel width of the bitmap.
    pub fn width(self) -> usize {
        return self._w;
    }

    /// Get the bitmap's height.
    ///    
    /// # Returns
    /// - Pixel height of the bitmap.
    pub fn height(self) -> usize {
        return self._h;
    }
}