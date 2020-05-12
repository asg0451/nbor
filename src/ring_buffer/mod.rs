#[derive(Debug)]
pub struct RingBuffer<T> {
    max: usize,
    vec: Vec<T>,
    idx: usize,
}

impl<T> RingBuffer<T>
where
    T: std::fmt::Debug,
{
    pub fn new(size: usize) -> RingBuffer<T> {
        let mut vec = Vec::new();
        vec.reserve_exact(size);
        RingBuffer {
            max: size,
            idx: 0,
            vec,
        }
    }

    pub fn push(&mut self, el: T) {
        if self.vec.len() < self.max {
            self.vec.push(el);
        } else {
            self.vec[self.idx] = el;
        }
        self.idx = (self.idx + 1) % self.max;
    }

    pub fn iter<'a>(&'a self) -> RingBufferIterator<'a, T> {
        // println!("making iterator for {:#?}", *self);

        RingBufferIterator {
            rb: self,
            idx: self.idx % self.vec.len(),
            len: self.vec.len(),
            counter: 0,
        }
    }
}

#[derive(Debug)]
pub struct RingBufferIterator<'a, T> {
    rb: &'a RingBuffer<T>,
    idx: usize,
    len: usize,
    counter: usize,
}
impl<'a, T> Iterator for RingBufferIterator<'a, T>
where
    T: std::fmt::Debug,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let res = if self.counter < self.len {
            // println!("iterating {:#?}", self);

            Some(&self.rb.vec[self.idx])
        } else {
            None
        };
        self.idx = (self.idx + 1) % self.len;
        self.counter += 1;
        res
    }
}
