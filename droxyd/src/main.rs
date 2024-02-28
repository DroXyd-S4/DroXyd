use droxyd::bloom_filter::bloom_filter::*;
use droxyd::bloom_filter::hash_functions::*;
use droxyd::bloom_filter::is_present::is_present;

fn main()
{
    /*
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
       */

    println!();
    println!("===============================================================");
    println!();

    println!("SHA-256 Tests");
    println!("DroXyd is the best project made by 2027 students ! = {}",
             sha256("DroXyd is the best project made by 2027 students !"));
    println!("Droxyd is the best project made by 2027 students ! = {}",
             sha256("Droxyd is the best project made by 2027 students !"));
    println!("DroXyd is the best project made by 2028 students ! = {}",
             sha256("DroXyd is the best project made by 2028 students !"));
    println!("droXyd is the best project made by 2027 students ! = {}",
             sha256("droXyd is the best project made by 2027 students !"));
    println!("droxyd is the best project made by 2027 students ! = {}",
             sha256("droxyd is the best project made by 2027 students !"));
    println!("DroXyd is the best project made by 2028 students !! = {}",
             sha256("DroXyd is the best project made by 2028 students !!"));

    println!();
    println!("===============================================================");
    println!();

    println!("MD5 Tests");
    println!("DroXyd is the best project made by 2027 students ! = {}",
             md5("DroXyd is the best project made by 2027 students !"));
    println!("Droxyd is the best project made by 2027 students ! = {}",
             md5("Droxyd is the best project made by 2027 students !"));
    println!("DroXyd is the best project made by 2028 students ! = {}",
             md5("DroXyd is the best project made by 2028 students !"));
    println!("droXyd is the best project made by 2027 students ! = {}",
             md5("droXyd is the best project made by 2027 students !"));
    println!("droxyd is the best project made by 2027 students ! = {}",
             md5("droxyd is the best project made by 2027 students !"));
    println!("DroXyd is the best project made by 2028 students !! = {}",
             md5("DroXyd is the best project made by 2028 students !!"));


    println!();
    println!("===============================================================");
    println!();

    let hash_functions = vec![sha256, md5, double_sha256];

    let mut words: Vec<String> = vec![
        String::from("Mickey"),
        String::from("Minnie"),
        String::from("Donald"),
        String::from("Daisy"),
        String::from("Dingo"),
        String::from("Pluto")
    ];

    println!("List of words : ");
    let mut a = 0;
    for word in &words
    {
        println!("md5     {}: {}", word, md5(word));
        println!("sha     {}: {}", word, sha256(word));
        println!("shasha  {}: {}", word, double_sha256(word));
        println!();
    }
    println!();
    println!();

    let filter = bloom_filter(&words, 0.1);
    print_filter(&filter);

    println!();
    println!("Added Uncle Scrooge, Riri, Fifi and Loulou to words");
    println!();

    words.push(String::from("Uncle Scrooge"));
    words.push(String::from("Riri"));
    words.push(String::from("Fifi"));
    words.push(String::from("Loulou"));

    for word in words
    {
        println!("{} in filter ? -> {}", word, is_present(&filter, &hash_functions, &word));
    }

}
