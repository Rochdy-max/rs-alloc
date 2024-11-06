// set no_std attribute only if std is not set as feature
#![cfg_attr(not(feature = "std"), no_std)]

#![allow(unused_variables)]
#![allow(dead_code)]


#[cfg(feature = "myalloc")]
pub mod myalloc {
    use core::alloc::GlobalAlloc;
    
    pub struct MyAlloc;
    
    unsafe impl GlobalAlloc for MyAlloc {
        unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
            unimplemented!()
        }
        
        unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
            unimplemented!()
        }
    }
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
