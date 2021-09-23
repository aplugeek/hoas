use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Hybrid Open Api Service", about = "Hybrid Open Api Service")]
pub struct App {
    /// Activate debug mode
    #[structopt(short = "l", long = "level", default_value = "INFO")]
    pub level: String,
    /// Parse rule json file path
    #[structopt(short = "f", long = "parse-file", default_value = "conf/api.yaml")]
    pub parse_path: String,
    /// runtime work thread
    #[structopt(short = "w", long = "worker", default_value = "128")]
    pub worker_thread: usize,
    /// server bind port
    #[structopt(short = "p", long = "port", default_value = "80")]
    pub port: usize,
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    use backtrace::Backtrace;
    use std::panic;
    panic::set_hook(Box::new(|e| {
        let bt = Backtrace::new();
        error!("Panic occurs,error:{:?},stack:{:?}", e, bt);
    }));
}
