use droxyd::bloom_filter::bloom_filter::bloom_filter;
use droxyd::bloom_filter::bloom_filter::bloom_size_computing;
use droxyd::bloom_filter::hash_functions::*;
//use droxyd::bloom_filter::is_present::is_present;

// use droxyd::bloom_filter::bloom_filter::bloom_filter;



fn main()
{
    println!();
    println!("==============================================================");
    println!();

    let mut num_k: i64 = 10000;
    let mut err_rate: f64 = 0.5;
    dbg!(bloom_size_computing(num_k, err_rate));

    num_k = 10000;
    err_rate = 0.9;
    dbg!(bloom_size_computing(num_k, err_rate));

    num_k = 100000;
    err_rate = 0.9;
    dbg!(bloom_size_computing(num_k, err_rate));

    num_k = 100000;
    err_rate = 0.1;
    dbg!(bloom_size_computing(num_k, err_rate));

    num_k = 1000000;
    err_rate = 0.01;
    dbg!(bloom_size_computing(num_k, err_rate));


    println!();
    println!("===============================================================");
    println!();

    println!("1: Hello, World! = {}", sha256("Hello, World!"));
    println!("2: Hello, World! = {}", md5("Hello, World!"));
    println!("3: Hello, World! = {}", sha256(&sha256("Hello, World!")));
    println!("4: Hello, World! = {}", md5(&md5("Hello, World!")));

    println!();
    println!("===============================================================");
    println!();

    let words: Vec<String> = vec![
        String::from("Mickey"),
        String::from("Minnie"),
        String::from("Donald"),
        String::from("Daisy"),
        String::from("Dingo"),
        String::from("Pluto")
    ];

    bloom_filter(words, 0.1);

}
