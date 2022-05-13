/// Iterator which buffers last result
pub trait Scannable<T> {
    fn curr(&self) -> T;
    fn pop(&mut self) -> bool;
}
