pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_size: usize) -> ThreadPool {
        assert!(_size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}