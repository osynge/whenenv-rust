use cfg;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

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
    pub fn desiredStateStep(&mut self, shared_config_referance: &Arc<Mutex<cfg::Config>>) {
        match (&self.threadpool) {
            Some(_) => {}
            None => {
                self.threadpool = Option();
                return;
            }
        }

        let thread_pool = self.threadpool.unwrap();

        thread_pool.spawn(lazy(move || {
            let data = Arc::clone(&shared_config_referance);
            let mut bill = thread_init {
                uuid: data.deref().clone(),
                wake: rx,
            };
            println!("called from a worker thread");
            worker_thread(&mut bill, job_db)
        }));
    }
}

pub fn threading_init(shared_config_referance: &Arc<Mutex<cfg::Config>>) -> threadInit {
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
