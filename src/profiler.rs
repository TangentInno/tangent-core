lazy_static! {
    static ref PROFILER_MESSAGE_QUEUE: std::sync::Mutex<std::collections::VecDeque<String>> = std::sync::Mutex::new(std::collections::VecDeque::new());
}

pub struct Profiler {
    pub name: std::string::String,
    start: std::time::Instant,
}

impl Profiler {
    /* if you recieve none from this function it's because the compilation unit didn't have the flag set correctly. */
    pub fn invoke(name: &str) -> Profiler {
        return Profiler {name: std::string::String::from(name), start: std::time::Instant::now()};
    }
}

impl Drop for Profiler {
    fn drop(&mut self) {
        let profiler_queue_clone_locked = PROFILER_MESSAGE_QUEUE.lock().unwrap();

        println!("{:?} took {:#?} to occur.", self.name, self.start.elapsed());

        drop(profiler_queue_clone_locked);
    }
}