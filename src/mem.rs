use core::{alloc::{Layout, GlobalAlloc}, ptr::null_mut};

struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator{
    unsafe fn alloc(mut self: &DummyAllocator , _layout: Layout) -> *mut u8{
        null_mut()
    }
    unsafe fn dealloc(mut self: &DummyAllocator, _ptr: *mut u8, _layout: Layout){
        panic!("dealloc called on DummyAllocator")
    }
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;