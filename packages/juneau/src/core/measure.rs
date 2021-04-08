use num_format::{Locale, ToFormattedString};
use time::Instant;
use tsc_time;


pub fn measure<T>(f:impl FnOnce() -> T) -> (T, i128, i128) {

    let start = Instant::now();
    let cycles_start = tsc_time::Start::now();
    let result = f();
    let cycles_stop = tsc_time::Stop::now();
    let stop = Instant::now();
    let duration = stop - start;
    let cycles = (cycles_stop - cycles_start).cycles() as i128;
    
    let duration = start.elapsed().whole_nanoseconds();
    (result, duration, cycles)
}

pub fn measure_and_print<T>(n:u64, f:impl FnOnce() -> T) -> T {
    let n = n as i128;
    let (result, duration, cycles) = measure(f);
    print!("items = {}, duration = {}ns, ns/item = {}, cycles/item = {}, items/s = {}, cycles/ms = {}, \n",
        n.to_formatted_string(&Locale::en),
        duration.to_formatted_string(&Locale::en),
        (duration / n).to_formatted_string(&Locale::en),
        (cycles / n).to_formatted_string(&Locale::en),
        ((n * 1_000_000_000) / duration).to_formatted_string(&Locale::en),
        (cycles * 1000 / duration).to_formatted_string(&Locale::en));
    
        result
}
