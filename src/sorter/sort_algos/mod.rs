mod span_sort;
pub use span_sort::*;

//T is the type that represents a pixel
//A represents how we want data to be returned.
pub trait SortMethod<P, R>{
    fn sort(&self, pixels: Vec<P>, line: usize) -> R;
}
