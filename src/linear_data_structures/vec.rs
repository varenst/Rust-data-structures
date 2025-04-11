// mission implement minimal working standart library vector, to understand inner working of it and how they can grow
// GetIntoGameDev https://www.youtube.com/watch?v=VxSiTKdQrqQ helped me start this class

use core::fmt;
use std::fmt::Debug;


///////////// MACROS ///////////// 
macro_rules! layout_for {
    ($t:ty, $cap:expr) => {
        std::alloc::Layout::from_size_align(
            $cap * std::mem::size_of::<$t>(),
            std::mem::align_of::<$t>()
        ).unwrap()
    };
}


// T: Debug means that the type parameter must have a debug implementation

#[derive(Clone)]
pub struct Vector<T: Debug> {
    data: *mut T, // raw pointer to heap-allocated memoory
    size: usize, // number of elements in vector
    capacity: usize // physical amount of memory
}

impl<T: Debug + Copy + PartialEq> Vector<T> {
    pub fn new() -> Self {
        let size: usize = 0;
        let capacity: usize = 1; // Must be 1 since otherwise when resized 2 * 0 = 0

        // let layout = std::alloc::Layout::from_size_align(16, 4).unwrap();
        // allocate 16 bytes, and they must be divisible by 4
        // Layout descriptor allows for effecient CPU usage
        let layout = layout_for!(T, capacity);

        let data: *mut T;

        unsafe  {
            data = std::alloc::alloc(layout) as *mut T; //Allocates uninitialized memory and casts to *mut T
        }

        Self {data, size, capacity}
    }

    fn resize(&mut self) {
        let byte_size = self.capacity * std::mem::size_of::<T>();
        let alignment = std::mem::align_of::<T>();
        let layout = std::alloc::Layout::from_size_align(byte_size, alignment).unwrap();

        unsafe  {
           let new_data = std::alloc::realloc(self.data as *mut u8, layout, 2 * byte_size) as *mut T;
           
            // Can happen when:
            // - device is out of memory
            // - requesting more then system allows
            // - doubling 1GB, memory might not be able to do that in place
           if new_data.is_null() {
            std::alloc::handle_alloc_error(layout);
           }

           self.data = new_data;
        }

        self.capacity = self.capacity * 2;
    }
    

    pub unsafe fn get(&self, index: usize) -> T {
        *self.data.add(index)
    }

    pub fn insert(&mut self, value: T) {
        while self.size >= self.capacity {
            self.resize();
        }

        //self.data.add(self.size) give me self.size elements past the start
        unsafe  {
            *self.data.add(self.size) = value
        }

        self.size += 1;
    }

    pub unsafe fn remove(&mut self, index: usize) {
        std::ptr::drop_in_place(self.data.add(index));   

        for i in index + 1..self.size {
            let index_value = self.get(i);

            self.set(i - 1, index_value);
        }

        self.size -= 1
    }

    pub unsafe fn find(&mut self, value: T) -> Option<usize> {
        for i in 0..self.size {
            if self.get(i) == value {
                return Some(i);
            }
        }

        None
    }
    pub unsafe fn set(&mut self, index: usize, value: T) {
        *self.data.add(index) = value
    }
}

impl<T: Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec_contents = f.debug_list();

        for i in 0..self.size {
            unsafe {
                vec_contents.entry(&*self.data.add(i));
            }
        }

        vec_contents.finish()
    }
}

impl <T: Debug> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl <T: Debug> Drop for Vector<T> {
    fn drop(&mut self) {
        for i in 0..self.size {
            unsafe {
                std::ptr::drop_in_place(self.data.add(i));
            }
        }

        let layout = layout_for!(T, self.capacity);

        unsafe  {
            std::alloc::dealloc(self.data as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basics() {
        let mut my_vec: Vector<i32> = Vector::new();
        println!("{}", my_vec);
    
        for i in 0..10 {
            my_vec.insert(i);
        }
        
        unsafe {
            my_vec.set(5, 100);
            println!("{}", my_vec);
            my_vec.remove(6);
            println!("{}", my_vec.get(5));
            println!("{}", my_vec);

            match my_vec.find(9) {
                Some(index) => println!("Found at index {}", index),
                None => println!("Not found"),
            }
            
        }
    }
}