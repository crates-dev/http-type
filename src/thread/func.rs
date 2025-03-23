use crate::*;

pub fn get_thread_count() -> usize {
    std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or_else(|_| num_cpus::get())
}
