
#[cfg(feature = "myalloc")]
mod enable_myalloc {
    use rs_alloc::myalloc::MyAlloc;
    
    #[global_allocator]
    static ALLOCATOR: MyAlloc = MyAlloc;
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::boxed::Box;

    #[test]
    fn box_instanciation() {
        let x = 5u8;
        let _ = Box::new(x);
    }
}
