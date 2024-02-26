use droxyd::bloom_filter::bloom_filter::bloom_filter;
use droxyd::bloom_filter::bloom_filter::bloom_size_computing;
use droxyd::bloom_filter::fill_filter::fill_filter;
use droxyd::bloom_filter::hash_functions::*;
use droxyd::bloom_filter::is_present::is_present;

// use droxyd::bloom_filter::bloom_filter::bloom_filter;



fn main()
{
    let num_k: i64 = 10000;
    let err_rate: f64 = 0.1;
    bloom_size_computing(num_k, err_rate);

    println!();
    println!("===============================================================");
    println!();

    println!("1: Hello, World! = {}", sha256("Hello, World!"));
    println!("2: Hello, World! = {}", md5("Hello, World!"));

}
