/*use droxyd::bloom_filter::bloom_filter::*;
use droxyd::bloom_filter::hash_functions::*;
use droxyd::bloom_filter::is_present::*;
*/

//use droxyd::crawl_web::get_content::*;
use droxyd::crawl_web::crawler::*;

fn leo()
{
    /*
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
    */

    println!();
    println!("===============================================================");
    println!();
    println!("Now, it's time to get the content of a webpage");
    println!();

    //println!("{}", get_content(String::from("https://example.com/")));
    crawler(10, String::from("https://en.wikipedia.org/wiki/42_(number)"));





}




















/* TESTS DE JUSTINE */

mod processing;
use crate::processing::demo::demo;


fn justine() {
    println!("\n ////////////////////////////////////////////////////////////////////// \n");
    let header =  "
  ∧,,,∧
 (• v •) ☆
|￣U U￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣|
|          DATA PROCESSING            |
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header);
    let path = "processing/src3.html";
    demo(&path, false);



    let header_2 = "        ∧,,,∧
                           (- _ -) ☆
|￣￣￣￣￣￣￣￣￣￣￣￣￣￣U U￣￣￣|
|              TF - IDF               |
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header_2);

}

fn main()
{
    leo();
    //justine();
}
