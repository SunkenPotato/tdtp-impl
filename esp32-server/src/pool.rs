use std::{
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    thread::{JoinHandle, spawn},
};

pub type Job = Box<dyn FnOnce() + Send + Sync + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(thread_n: usize) -> Self {
        assert!(thread_n > 0);

        let (sender, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        let sender = Some(sender);

        let mut workers = Vec::with_capacity(thread_n);

        for _ in 0..thread_n {
            workers.push(Worker::new(Arc::clone(&rx)));
        }

        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }

    pub fn close(&mut self) {
        drop(self.sender.take().unwrap());

        for worker in self.workers.drain(..) {
            worker.handle.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.close()
    }
}

pub struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = spawn(move || {
            loop {
                if let Ok(v) = receiver.lock().unwrap().recv() {
                    v();
                } else {
                    break;
                }
            }
        });

        Self { handle }
    }
}
