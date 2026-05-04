//! #Threadpools
//! this create only exist to create a multi-thread solution 
//! for our single thread server

use std::{sync::{Arc, Mutex, mpsc}, thread::{self, JoinHandle}};

struct Worker {
    id : usize ,
    thread : Option<JoinHandle<()>>
}

impl Worker {
    fn new(id : usize , receiver : Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(task) => {
                        println!("Worker {} running a task ..." , id);
                        task.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} is about to terminate ..." , id);
                        break;
                    }
                }
            }
        });
        Worker { id , thread : Some(thread) }
    }
}

trait FnBox {
    fn call_box(self : Box<Self>);
}

impl<F : FnOnce ()> FnBox for F {
    fn call_box(self : Box<Self>) {
        (*self)()
    }
}


type Job = Box<dyn FnBox + Send + 'static>;


enum Message {
    NewJob(Job),
    Terminate
}

/// struct to work with threads make all the server multithreaded 
/// can hold up number of workers of user choice 
/// 
/// using channel to send the tasks between threads and workers
pub struct ThreadPool {
    workers : Vec<Worker>,
    sender : mpsc::Sender<Message>
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panics if the size is zero 
    /// 
    pub fn new(size : usize) -> Self {
        assert!(size > 0);
        
        let mut workers = Vec::with_capacity(size);

        let (sender , receiver) = mpsc::channel();

        // Creating a ARC out of the receiver with shadowing the value and get the value of tuple from previous line 
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
           workers.push(Worker::new(id , Arc::clone(&receiver))); 
        }

        Self { workers : workers , sender : sender}
    }
    
    
    pub fn execution<F>(&self , f : F)
        where 
            F : FnOnce() + Send + 'static
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).expect("Could execute the task !");
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message for all workers ...");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers ...");

        for worker in &mut self.workers {
            
            println!("Shutting down worker {}" , worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}