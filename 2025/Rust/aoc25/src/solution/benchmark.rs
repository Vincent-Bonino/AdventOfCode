#[macro_export]
macro_rules! time {
    ($call:expr) => {{
        let start = ::std::time::Instant::now();
        let result = $call;
        let duration = start.elapsed().as_micros();
        (result, duration)
    }};
}
