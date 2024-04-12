/*use droxyd::bloom_filter::bloom_filter::*;
  use droxyd::bloom_filter::hash_functions::*;
  use droxyd::bloom_filter::is_present::*;
  */

//use droxyd::crawl_web::get_content::*;
use droxyd::crawl_web::crawler::*;

fn crawler_tests()
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
    println!("Now, it's time to explore the web...");
    println!();

    //println!("{}", get_content(String::from("https://example.com/")));
    crawler(5);
}

/* TESTS DE JUSTINE */

mod processing;
use crate::processing::demo::demo;


fn scraper_tests()
{
    let url = "https://cglab.ca/~abeinges/blah/rust-btree-case/";
    demo(&url, false);
}

fn parser_tests() {
    //let s = "-aa inurl:bernard.com \" t e s t \" aaa OR:\"char a\"/\"heli helo\" unsite:chat.com intext:fer argent";
    println!("Teste suite:");
    println!("");

    let s1 = "crevette";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "-voiture";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "site:pizzahut.com";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "OR:voiture/moto";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "inurl:pizza/commande";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "unsite:pizza.com";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "\"pizza au 4 fromage\"";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");

    let s1 = "site:pizzahut.com -ananas \"fromage a raclette\"";
    let p = parser::parser::parser(&s1);
    println!("{:?}",s1);
    println!("{:?}",p);
    println!("");
}


use self::models::*;
use diesel::prelude::*;
use droxyd::*;


pub fn search(s: &str) -> Vec<Post1>
{
    use self::schema::posts1::dsl::*;
    use self::schema::posts2::dsl::*;

    let connection = &mut establish_connection();
    let results = posts1
        .inner_join(posts2.on(
                key.eq(s).and(idofsite.eq(id))
                ))
        .select(Post1::as_select())
        .load(connection)
        .expect("Error loading posts");
    return results;
}

mod parser
{
    pub mod parser;
}
use parser::parser::parser;

pub fn query(s: &str) -> Vec<Post1>
{
    let test = parser(s);
    let mut sitetmp:Vec<Post1> = vec![];
    let mut occ:Vec<usize> = vec![];
    let mut nositetmp:Vec<Post1> = vec![];
    let mut site:Vec<Post1> = vec![];
    for w in test.no_word
    {
        let tmp = search(&w);
        for p in tmp
        {
            let mut t = true;
            for i in &nositetmp
            {
                if i.id == p.id
                {
                    t = false;
                    break;
                }
            }
            if t
            {
                nositetmp.push(p);
            }
        }
    }
    let len = test.key_word.len();
    for w in test.key_word
    {
        let tmp = search(&w);
        for p in tmp
        {
            let mut t = true;
            for i in &nositetmp
            {
                if i.id == p.id
                {
                    t = false;
                    break;
                }
            }
            if t
            {
                let mut oc:i32 = -1;
                let mut n = 0;
                for i in &sitetmp
                {
                    if i.id == p.id
                    {
                        oc = n;
                        break;
                    }
                    n +=1;
                }
                if oc != -1
                {
                    occ[oc as usize] += 1;
                    if occ[oc as usize] == len
                    {
                        site.push(p);
                    }
                }
                else
                {
                    if len != 1
                    {
                        occ.push(1);
                        sitetmp.push(p);
                    }
                    else
                    {
                        site.push(p);
                    }
                }
            }
        }
    }
    return site;
    /*use testing::src::main::test;
    test();*/
    /*let words = ["pizza","ananas","hut"];
    let mut site:Vec<Post1> = vec![];
    for i in words
    {
        let c = search(i);
        for j in c
        {
            site.push(j);
        }
    }
    for i in site
    {
        dbg!(i);
    }*/
}

fn query_tests()
{
    println!("Test suite:");
    println!("============================================================");
    println!("Input: cat");
    dbg!(query("cat"));
    println!("============================================================");
    println!("Input: pizza");
    dbg!(query("pizza"));
    println!("============================================================");
    println!("Input: pizza fromage");
    dbg!(query("pizza fromage"));
    println!("============================================================");
    println!("Input: pizza -fromage");
    dbg!(query("pizza -fromage"));
}



fn main()
{
    println!("Crawler's Tests");
    crawler_tests();
    println!();
    println!("Scraper's Tests");
    scraper_tests();
    println!();
    //println!("Parser's Tests");
    //parser_tests();
    //println!();
    println!("Queries's Tests");
    query_tests();
    println!();
}


