#[derive(Debug)]
pub struct PoolCreationError;

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cannot create ThreadPool with size 0")
    }
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
}

// Penggunaan di main.rs:
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Dengan build, kita bisa handle error dengan lebih graceful
    let pool = ThreadPool::build(4).unwrap_or_else(|err| {
        eprintln!("Failed to create thread pool: {}", err);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}