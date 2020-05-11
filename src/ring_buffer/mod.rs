mod ring_buffer {
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

        // https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter
        // pub fn for_each(&self, f: fn(T) -> ()) { // fn pointer or non-capturing lambda
        // same as for_each(&self, f: F) where F: Fn(etc) {}
        // pub fn for_each(&self, f: impl Fn(&T) -> ()) {
        //     let last = self.idx % self.vec.len();
        //     let mut i = self.idx + 1;
        //     while (i % self.vec.len()) != last {
        //         i += 1;
        //         f(&self.vec[i % self.vec.len()]);
        //     }
        // }

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

    // impl<T> IntoIterator for RingBuffer<T> {
    //     type Item = T;
    //     type IntoIter = RingBufferIntoIterator<T>;
    //     fn into_iter(self) -> Self::IntoIter {
    //         let len = self.vec.len();
    //         let idx = self.idx;
    //         RingBufferIntoIterator {
    //             rb: self,
    //             idx: idx,
    //             last: idx % len,
    //         }
    //     }
    // }
}

pub use ring_buffer::*;
