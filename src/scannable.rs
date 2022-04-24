/// Iterator which buffers last result
pub trait Scannable<T> {
    fn peek(&self) -> T;
    fn pop(&mut self);
}
