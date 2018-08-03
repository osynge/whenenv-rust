use std::collections::HashMap;

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum ThreadTask {
    Db,
    Run,
}

use futures::future::{lazy, Future};

use std::time::Duration;
use tokio_threadpool;

#[derive(Debug)]
pub struct threadInit {
    desired_state: HashMap<ThreadTask, u32>,
    threadpool: Option<tokio_threadpool::ThreadPool>,
}

impl threadInit {
    pub fn new() -> Result<threadInit, String> {
        let bill = threadInit {
            desired_state: HashMap::new(),
            threadpool: None,
        };
        Ok(bill)
    }
    pub fn desiredStateSet(&mut self, new_state: &HashMap<ThreadTask, u32>) -> () {
        
        for (key, val) in new_state.iter() {
            self.desired_state.insert(key.clone(), val.clone());
        }
    }
}

pub fn threading_init() -> threadInit {
    let mut desired_state = HashMap::new();
    desired_state.insert(ThreadTask::Db, 1);
    desired_state.insert(ThreadTask::Run, 4);
    let mut thread_pool2 = threadInit::new().unwrap();
    thread_pool2.desiredStateSet(&desired_state);
    let thread_pool = tokio_threadpool::Builder::new()
        .pool_size(4)
        .keep_alive(Some(Duration::from_secs(30)))
        .build();

    for (book, review) in &desired_state {
        match book {
            ThreadTask::Run => {}
            ThreadTask::Db => {
                // let _ = listener::henry();
            }
        }

        println!("{}: \"{}\"", "ddd", review);
    }
    let thread_pool = threadInit::new();
    thread_pool.unwrap()
}
pub fn threaing_run() {}
