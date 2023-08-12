use std::{sync::mpsc, thread};

struct Job;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        println!("I'm doin' it!")
    }
}

impl Worker {
    pub fn new(id: usize) -> Self {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
