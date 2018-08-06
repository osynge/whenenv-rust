use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Deref;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tokio_threadpool;
use futures::future::{lazy, Future};


use cfg;
use uuid;
extern crate serde;
extern crate serde_json;

#[macro_use]
use serde_derive;

use listener;
use serde_json::Error;
use std::time::Duration;
use thread_common;

static NTHREADS: i32 = 3;

pub(crate) fn redirect_fd() -> Option<u32> {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
    return None;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ThreadTask {
    Exit,
    Rdbms,
    Executor,
}

#[derive(Debug)]
pub struct ThreadComunicationRaw {
    pub uuid: uuid::Uuid,
    pub outbound: Option<Sender<ThreadComunication>>, // Child ID wanted
    pub inbound: Option<Receiver<ThreadComunication>>, // Pairent ID to mirror
}

#[derive(Debug)]
pub struct ThreadComunicationManager {
    pub inbound: HashMap<uuid::Uuid, ThreadComunicationRaw>, // Pairent ID to mirror
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadComunication {
    pub sender: String,
    pub task: ThreadTask,
}

pub(crate) fn start_mpsc_usize(size: u32) -> Vec<ThreadComunicationRaw> {
    let mut ids: Vec<ThreadComunicationRaw> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let (tx, rx): (Sender<ThreadComunication>, Receiver<ThreadComunication>) = mpsc::channel();
        let bill = ThreadComunicationRaw {
            uuid: uuid::Uuid::new_v4(),
            outbound: Some(tx),
            inbound: Some(rx),
        };
        ids.push(bill);
    }
    return ids;
}

#[derive(Debug)]
pub struct thread_init {
    uuid: String,
    wake: Receiver<ThreadComunication>,
}

fn worker_thread(input: &mut thread_init, task: thread_common::ThreadTask) -> Result<(), ()> {
    match task {
        thread_common::ThreadTask::Run => {
            let _ = listener::henry();
        }
        thread_common::ThreadTask::Db => {
            let _ = listener::henry();
        }
    }

    drop(input);

    Ok(())
}

pub(crate) fn wibble(shared_config_referance: &Arc<Mutex<cfg::Config>>) -> u32 {
    //let mut books = HashSet::new();
    let data_inc = Arc::new(start_mpsc_usize(4));

    use futures::future::{lazy, Future};

    // Create a thread pool with default configuration values
    let thread_pool = tokio_threadpool::Builder::new()
        .pool_size(4)
        .keep_alive(Some(Duration::from_secs(30)))
        .build();
    let (tx, rx): (Sender<ThreadComunication>, Receiver<ThreadComunication>) = mpsc::channel();

    let data = Arc::new(uuid::Uuid::new_v4().to_string());

    thread_pool.spawn(lazy(move || {
        let data = Arc::clone(&data);
        let job_db = thread_common::ThreadTask::Db;
        let mut bill = thread_init {
            uuid: data.deref().clone(),
            wake: rx,
        };
        println!("called from a worker thread");
        worker_thread(&mut bill, job_db)
    }));

    // Gracefully shutdown the threadpool
    thread_pool.shutdown().wait().unwrap();
    0
}
