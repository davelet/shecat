use std::{sync::{mpsc, Arc, Mutex}, thread};

use log::info;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: u32) -> Self {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let rec = Arc::new(Mutex::new(receiver));
        let mut threads = vec![];
        for id in 0..size {
            threads.push(Worker::new(id, rec.clone()));
        }
        Self {
            threads,
            sender,
        }
    }

    pub fn exec<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static {
        // let (sender, _) = mpsc::channel();
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            info!("Worker {} is waiting for a job.", id);
            let lock = receiver.lock();
            info!("Worker {} getting lock; trying.", id);
            let lock = lock.unwrap();
            info!("Worker {} got a lock; locking.", id);
            let job = lock.recv().unwrap();
            info!("Worker {} got a job; executing.", id);
            job();
        });
        Self {
            id,
            thread,
        }
    }
}