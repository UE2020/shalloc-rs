use shalloc::Shalloc;

#[global_allocator]
static ALLOCATOR: Shalloc = Shalloc;

use std::{
    alloc::Layout,
    time::{Duration, Instant},
};

fn main() {
    // Set the duration for each cycle (e.g., 1 second).
    let cycle_duration = Duration::from_secs(1);

    loop {
        let start = Instant::now();
        let mut alloc_count = 0;

        // Allocate as many Vec<u8> objects as possible within the time limit
        while Instant::now() - start < cycle_duration {
            // Allocate a vector with 1024 bytes
            let data = unsafe { std::alloc::alloc(Layout::array::<u8>(1024).unwrap()) };
            unsafe { std::alloc::dealloc(data, Layout::array::<u8>(1024).unwrap()) };
            alloc_count += 1;
        }

        println!("Completed cycle with {} allocations", alloc_count);

        // Sleep for a bit before starting the next cycle (optional)
        std::thread::sleep(Duration::from_millis(100));
    }
}
