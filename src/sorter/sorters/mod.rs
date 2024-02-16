mod scanline;
pub use scanline::*;
use crate::sorter::SortMethod;

//T represents our pixels, A represents the image.
// S represents the sorter that we'll use.
// R represents the return we want.
pub trait Sorter<PixelType, ImageType, ReturnType, SortReturnType>
{

    fn sort_image(&self, image: ImageType, sorter: Box<dyn SortMethod<PixelType, SortReturnType>>) -> ReturnType;
}


