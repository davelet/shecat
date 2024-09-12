use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use log::{info, warn};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
            sender: Some(sender),
        }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // let (sender, _) = mpsc::channel();
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.threads {
            warn!("to shut down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
            // drop(worker);
        }
    }
}

struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            info!("Worker {} is waiting for a job.", id);
            let lock = receiver.lock();
            let lock = lock.unwrap();
            let job = lock.recv();
            if let Ok(job) = job {
                info!("Worker {} got a job; executing.", id);
                job();
            } else {
                info!("Worker {} is shutting down.", id);
                break;
            };
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
