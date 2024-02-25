use droxyd::bloom_filter::bloom_filter::bloom_filter;
use droxyd::bloom_filter::bloom_filter::bloom_size_computing;
use droxyd::bloom_filter::fill_filter::fill_filter;
use droxyd::bloom_filter::hash_functions::sha256;
use droxyd::bloom_filter::hash_functions::hash_2;
use droxyd::bloom_filter::hash_functions::hash_3;
use droxyd::bloom_filter::is_present::is_present;

// use droxyd::bloom_filter::bloom_filter::bloom_filter;



fn main()
{
    let num_k = 5_i64;
    let err_rate = 0.1_f64;
    bloom_size_computing(num_k, err_rate);

    println!();
    println!("===============================================================");
    println!();

    sha256("fffffffffffffffffffffffffffffffffffffffffffffffffffffff");


}
