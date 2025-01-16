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
    #[inline]
    #[must_use]
    pub fn new(port: u16, threads: usize) -> Self {
        log::info!("Running the server on port :{port}");

        Self {
            receiver: Receiver::new(port),
            pool: ThreadPool::new(threads),
            router: Router::new(),
        }
    }

    #[inline]
    pub fn run(&mut self) -> ! {
        loop {
            let Some((mut stream, addr)) = self.receiver.next_request() else {
                log::error!("Cannot receive the next request");
                continue;
            };

            let req = match Request::new(&mut stream, addr) {
                Ok(req) => req,
                Err(e) => {
                    log::error!("Cannot parse the request: {e}");
                    continue;
                }
            };

            let Some(handler) = self.router.find_handler(req.path.clone(), req.rtype) else {
                log::error!("No handler found for this request");
                continue;
            };

            self.pool.execute(move || {
                let response = handler(req);
                let bytes = match response.build() {
                    Ok(b) => b,
                    Err(e) => {
                        log::error!("Failed building the response: {e}");
                        return;
                    }
                };

                if let Err(e) = stream.write(&bytes) {
                    log::error!("Cannot write to the stream: {e}");
                }
            });
        }
    }

    #[inline]
    pub fn add_route<R>(&mut self, route: R)
    where
        R: Route + 'static,
    {
        self.router.add_route(route);
    }

    #[inline]
    pub fn add_default_handler<R>(&mut self, route: R)
    where
        R: Route + 'static,
    {
        self.router.add_default_handler(route);
    }
}

impl Default for Server {
    #[inline]
    fn default() -> Self {
        Self::new(6060, 20)
    }
}
