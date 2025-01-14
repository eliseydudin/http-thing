use std::io::Write;

use crate::{
    router::{Receiver, Request, Route, Router},
    thread_pool::ThreadPool,
};

pub struct Server {
    receiver: Receiver,
    pool: ThreadPool,
    router: Router,
}

impl Server {
    pub fn new(port: u16, threads: usize) -> Self {
        log::info!("Running the server on port :{port}");

        Self {
            receiver: Receiver::new(port),
            pool: ThreadPool::new(threads),
            router: Router::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let (mut stream, addr) = match self.receiver.next_request() {
                Some(n) => n,
                None => {
                    log::error!("Cannot receive the next request");
                    continue;
                }
            };

            let req = match Request::new(&mut stream, addr) {
                Some(req) => req,
                None => {
                    log::error!("Cannot parse the request");
                    continue;
                }
            };

            let handler = match self.router.find_handler(req.path.clone(), req.rtype) {
                Some(h) => h,
                None => {
                    log::error!("No handler found for this request");
                    continue;
                }
            };

            self.pool.execute(move || {
                let response = handler(req);
                let bytes = response.build();
                match stream.write(&bytes) {
                    Err(e) => log::error!("Cannot write to the stream: {e}"),
                    _ => (),
                }
            });
        }
    }

    pub fn add_route(&mut self, route: impl Route + 'static) {
        self.router.add_route(route);
    }

    pub fn add_default_handler(&mut self, route: impl Route + 'static) {
        self.router.add_default_handler(route);
    }
}
