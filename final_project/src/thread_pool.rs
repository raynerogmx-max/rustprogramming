use std::sync::{Arc, Mutex, Condvar};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<(Mutex<Vec<Job>>, Condvar)>,
    shutdown: Arc<Mutex<bool>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let queue = Arc::new((
            Mutex::new(Vec::<Job>::new()),
            Condvar::new(),
        ));

        let shutdown = Arc::new(Mutex::new(false));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let queue = Arc::clone(&queue);
            let shutdown = Arc::clone(&shutdown);

            let handle = thread::spawn(move || loop {
                let job = {
                    let (lock, cvar) = &*queue;
                    let mut queue = lock.lock().unwrap();

                    while queue.is_empty() && !*shutdown.lock().unwrap() {
                        queue = cvar.wait(queue).unwrap();
                    }

                    // If shutdown AND no remaining jobs → exit
                    if *shutdown.lock().unwrap() && queue.is_empty() {
                        return;
                    }

                    queue.pop()
                };

                if let Some(job) = job {
                    job();
                }
            });

            workers.push(handle);
        }

        Self { workers, queue, shutdown }
    }

    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let (lock, cvar) = &*self.queue;
        let mut queue = lock.lock().unwrap();
        queue.push(Box::new(job));
        cvar.notify_one();
    }

    pub fn shutdown(self) {
        *self.shutdown.lock().unwrap() = true;
        self.queue.1.notify_all();

        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}
