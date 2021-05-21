use alloc::vec;

#[derive(Debug, Copy, Clone)]
pub struct Array<'lt, T> {
    raw_data : &'lt[T]
}

impl<'lt, T> Array<'lt, T> {
    pub fn new(capacity : usize) -> Array<'lt, T> {
        Array {
            raw_data : vec![0 ; capacity]
        }
    }

    pub fn from_raw(data : &'lt[T]) -> Array<'lt, T> {
        Array {
            raw_data: data
        }
    }

    pub fn get(&self, index : usize) -> &T {
        &self.raw_data[index]
    }

    pub fn get_mut(&mut self, index : usize) -> &mut T {
        &mut self.raw_data[index]
    }
}




