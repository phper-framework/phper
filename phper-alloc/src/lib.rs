use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        eprintln!("GlobalAlloc::alloc");
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        eprintln!("GlobalAlloc::dealloc");
        System.dealloc(ptr, layout)
    }
}

//#[global_allocator]
//static GLOBAL: MyAllocator = MyAllocator;
