/* To add 
use droxyd::bloom_filter::bloom_filter::*;
use droxyd::bloom_filter::hash_functions::*;
use droxyd::bloom_filter::is_present::*;
use droxyd::crawl_web::get_content::*;
*/

use droxyd::crawl_web::crawler::*;

use crate::processing::{
    cleansing::*,
    extraction::*,
    language::*,
    IO_helper::*,
};

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
    //crawler(2000);
}

/* TESTS DE JUSTINE */

mod processing;
use crate::processing::demo::demo;


fn scraper_tests()
{
    let url = "https://cglab.ca/~abeinges/blah/rust-btree-case/";
    demo(&url, false);
}

/* Add in data base Justine */

use self::models::{NewPost1, Post1};

pub fn create_post1(url: &str, langue: &str, name: &str, date: &str, word1: &str, word2: &str, word3: &str) {
    use crate::schema::posts1;

    let conn = &mut establish_connection();
    let new_post = NewPost1 { url, langue, name, date, word1, word2, word3};

    diesel::insert_into(posts1::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}

use self::models::{NewPost2, Post2};

pub fn create_post2( key: &str, idofsite: &i32) {
    use crate::schema::posts2;
    let conn = &mut establish_connection();

    let new_post = NewPost2 { key, idofsite };

    diesel::insert_into(posts2::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}
pub fn search_id(s: &str) -> i32
{
    use self::schema::posts1::dsl::*;

    let connection = &mut establish_connection();
    let results = posts1
        .filter(
                url.eq(s)
                )
        .select(Post1::as_select())
        .load(connection)
        .expect("Error loading posts");
    if results.len() == 0
    {
        return -1;
    }
    return results[0].id;
}

pub fn add_in_data_base(u: &str)
{
    let text = URL_to_String(&u).unwrap();
    let lang = get_lang(&text);
    let title = get_title(&text);
    let (keywords, w1, w2, w3) = keywords2(TF_IDF(text), 25);
    create_post1(&u, &lang, &title, &String::new(), &(w1.to_lowercase()), &(w2.to_lowercase()), &(w3.to_lowercase()));
    let id = search_id(&u);
    if id == -1 { return }
    for i in keywords {
        create_post2(&(i.to_lowercase()), &id);
    }

    //create_post1("https://www.youtube.com/watch?v=oQaHPZ4c1QE&ab_channel=Kolanii","USA","bob","29/02/-5000","chat","poils","teste");
    //create_post2("minecraft",&11);
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

use std::collections::HashMap;

pub fn search_all_word() -> Vec<(String,i32)>
{
    use self::schema::posts2::dsl::*;

    let connection = &mut establish_connection();
    let results = posts2
        .select(Post2::as_select())
        .load(connection)
        .expect("Error loading posts");
    let mut h = HashMap::new();
    for post in results
    {
        h.insert(post.key.clone(), 1 + if h.contains_key(&(post.key.clone())) { h[&(post.key.clone())] } else { 1 });
        /*if h.contains_key(&post.key)
        {
            h[post.key] += 1;
        }
        else
        {
            h.insert(post.key,1);
        }*/
    }
    let mut v = vec![];
    for (la, le) in &h
    {
        v.push(((*la).clone(),*le));
    }
    return v;
}
//<==3

pub fn search_in_database(s: &str) -> Vec<Post1>
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
        let tmp = search_in_database(&w);
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
    let mut len = test.key_word.len();
    for w in test.key_word.clone()
    {
        let tmp = search_in_database(&w);
        if tmp.len() == 0
        {
            len -= 1;
        }
    }
    for w in test.key_word
    {
        let tmp = search_in_database(&w);
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
                if oc == -1
                {
                    sitetmp.push(p.clone());
                    site.push(p);
                    /*occ[oc as usize] += 1;
                    if occ[oc as usize] == len
                    {
                        site.push(p);
                    }*/
                }
                /*else
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
                }*/
            }
        }
    }
    //dbg!(test.key_word.len());
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
    println!("Input: the cat");
    dbg!(query("the cat"));
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

/** IMPORTS **/
use std::time::{ SystemTime, SystemTimeError, UNIX_EPOCH };
use std::fs::read_to_string;
//mod interface;

/** ROCKET **/
//mod models;
use rocket::{get, launch, post, routes, uri};
use rocket::form::{Contextual, Form};
use rocket::fs::{FileServer, Options, relative};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
use crate::postquery::models::SearchRequest;

/** VARIABLES **/

static mut Dic: &mut [[u32; 6]; 1000000] = &mut [[0u32; 6]; 1000000];//dictionnary
static mut cReq: std::string::String = String::new();
static mut DicDb : Vec<(String,i32)> = vec![];
static mut VecOfSearch: Vec<Post1> = vec![];
static mut QueryLimit: i32 = 0;
static mut Historique: String = String::new();


/** RENDER DEFAULT HOME PAGE **/
#[get("/")]
async fn root() -> Template {
    Template::render("root", context! { message: "Welcome to DroXyd Premium !"})
}

/** RENDER FR HOME PAGE **/
#[get("/fr")]
async fn root_fr() -> Template {
    init_dic(1);
    Template::render("root", context! { message: "Welcome to DroXyd France !"})
}

/** RENDER EN HOME PAGE **/
#[get("/en")]
async fn root_en() -> Template {
    init_dic(0);
    Template::render("root", context! { message: "Welcome to DroXyd English !"})
}

/** RENDER PREMIUM HOME PAGE **/
#[get("/home")]
async fn root_home() -> Template {
    init_dic(2);
    Template::render("root", context! { message: "Welcome to DroXyd Premium !"})
}

/** RENDER RESULTS PAGE **/
#[get("/hi?<request>")]
async fn hello(request: String, flash: Option<FlashMessage<'_>>) -> Template {

    /** PARSE REQUEST **/
    let message2 = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    let parts: Vec<_> = message2.split("#").collect();
    let bRequest = parts[0].to_lowercase(); // Base Request
    let sRequest = parts[1].to_lowercase(); // Suggestion of correction
    let qResults = parts[2].to_lowercase(); // All results
    let mut nb = "0".to_string(); // N°page
    let mut currentPage = "0".to_string();
    let mut currentPagePlus = "1".to_string();
    let mut currentPageMinus = "0".to_string();
    if parts[3] != "" {nb = parts[3].to_string();}
    let mut nbResults = 0;

    /** PRINTABLE RESULTS **/
    let mut R0 = "".to_string();
    let mut R1 = "".to_string();
    let mut R2 = "".to_string();
    let mut R3 = "".to_string();
    let mut R4 = "".to_string();
    let mut R5 = "".to_string();
    let mut R6 = "".to_string();
    let mut R7 = "".to_string();
    let mut R8 = "".to_string();
    let mut R9 = "".to_string();
    let mut link0 = String::new();
    let mut link1 = String::new();
    let mut link2 = String::new();
    let mut link3 = String::new();
    let mut link4 = String::new();
    let mut link5 = String::new();
    let mut link6 = String::new();
    let mut link7 = String::new();
    let mut link8 = String::new();
    let mut link9 = String::new();
    let mut SUGG = String::new();
    let mut SUGG_A = String::new();
    let mut SUGG_B = String::new();
    let mut SUGG_C = String::new();
    let mut SUGG_D = String::new();
    let mut SUGG_E = String::new();

    /** LAUNCH QUERY **/
    let v = query(&bRequest);
    nbResults = v.len() as i32;
    if nbResults == 0
    {
        open::that("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    }
    if nbResults > 10000
    {
        nbResults = 10000;
    }
    let mut resdb = [[""; 5]; 10000];
    for i in 0..(nbResults as usize)
    {
        resdb[i] = [v[i].name.as_str(),v[i].url.as_str(),v[i].word1.as_str(),v[i].word2.as_str(),v[i].word3.as_str()];
    }



    /** PARSE RESULTS/PAGE **/
    unsafe {
    let mut tmpV: Vec<(i32,[&str;5])> = vec![];
    for i in 0..(resdb.len())
    {
        let mut note = 0;
        for j in 0..5
        {
            if (j != 1) & (resdb[i][j] != "")
            {
                if Historique.contains(resdb[i][j])
                {
                    note += 1;
                }
                if bRequest.contains(resdb[i][j])
                {
                    note += 3;
                }
            }
        }
        tmpV.push((note,resdb[i].clone()));
    }
    tmpV.sort();
    let tmpV2 : Vec<(i32,[&str;5])> = tmpV.into_iter().rev().collect();
    for i in 0..(nbResults as usize)
    {
        resdb[i] = tmpV2[i].1.clone();
    }

    if (nb.parse::<i32>().unwrap()*10 >= nbResults) & (nbResults != 0)
    {
        nb = (nb.parse::<i32>().unwrap() as usize - 1).to_string();
    }
    if nb.parse::<i32>().unwrap() >= 10
    {
        nb = "9".to_string();
    }
    if nb.parse::<i32>().unwrap() < 0
    {
        nb = "0".to_string();
    }
    R0 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 0]);
    R1 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 1]);
    R2 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 2]);
    R3 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 3]);
    R4 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 4]);
    R5 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 5]);
    R6 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 6]);
    R7 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 7]);
    R8 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 8]);
    R9 = getParsedRes(resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 9]);

    /** DISPLAY RESULTS INFO **/
    let mut f = 0;
    for i in 2..5
    {
        for j in 0..10 as usize
        {
             if !(bRequest.contains(resdb[nb.parse::<i32>().unwrap() as usize * 10 + j][i]))
             {
                f += 1;
                SUGG += resdb[nb.parse::<i32>().unwrap() as usize * 10 + j][i];
                SUGG += "#";
             }
             if f>=5
             {
                break;
             }
        }
        if f>=5
        {
             break;
        }
    }
    }

    /** LINKED WORDS RENDER **/
    let parts2: Vec<_> = SUGG.split("#").collect();
    if parts2.len() > 0
    {
        SUGG_A = parts2[0].to_string();
    }
    if parts2.len() > 1
    {
        SUGG_B = parts2[1].to_string();
    }
    if parts2.len() > 2
    {
        SUGG_C = parts2[2].to_string();
    }
    if parts2.len() > 3
    {
        SUGG_D = parts2[3].to_string();
    }
    if parts2.len() > 4
    {
        SUGG_E = parts2[4].to_string();
    }
    unsafe{
    Historique.push_str(&SUGG_A);
    Historique.push(' ');
    Historique.push_str(&SUGG_B);
    Historique.push(' ');
    Historique.push_str(&SUGG_C);
    Historique.push(' ');
    Historique.push_str(&SUGG_D);
    Historique.push(' ');
    Historique.push_str(&SUGG_E);
    Historique.push(' ');
    Historique.push_str(&bRequest);
    Historique.push(' ');
    }
    /** BROWSE LINKS **/
    unsafe{
        link0 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 0][1];
        link1 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 1][1];
        link2 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 2][1];
        link3 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 3][1];
        link4 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 4][1];
        link5 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 5][1];
        link6 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 6][1];
        link7 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 7][1];
        link8 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 8][1];
        link9 += resdb[ nb.parse::<i32>().unwrap() as usize * 10 + 9][1];
    }
    
    
    currentPage = nb.clone();
    currentPagePlus = (nb.parse::<i32>().unwrap()  as usize + 1).to_string();
    if nb.parse::<i32>().unwrap() != 0
    {
        currentPageMinus = (nb.parse::<i32>().unwrap() as usize - 1).to_string();
    }
    let nbResultsToString = nbResults.to_string();
    let nbPage = (nbResults / 10 + 1).to_string();

    /** RENDER **/
    Template::render("hello", context!
    {bRequest ,sRequest, qResults,
    SUGG_A,SUGG_B,SUGG_C,SUGG_D,SUGG_E,
    R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,
    link0,link1,link2,link3,link4,link5,link6,link7,link8,link9,
    currentPage,currentPagePlus,currentPageMinus,
    nbResultsToString,nbPage})
}

/** PARSE RESULTS ON PAGE **/
fn getParsedRes(l: [&str;5]) -> String
{
     let mut res = String::new();
     res += &l[0];
     res += " | ";
     res += &l[1];
     res += " | ";
     res += &l[2];
     res += ",";
     res += &l[3];
     res += ",";
     res += &l[4];
     return res;
}

/** PARSE REQUEST AND REDIRECT **/
#[post("/", data = "<form>")]
async fn create(mut form: Form<Contextual<'_, SearchRequest>>) -> Result<Flash<Redirect>, Template> {
    if let Some(ref mut search) = form.value {
        let name = format!("{} {}", search.get_request(), search.get_results());
        let mut qRes = String::new();
        unsafe{
            qRes += &search.get_request();
            qRes += "#";
            for i in (&search.get_request()).split_whitespace()
            {
                let mut res = Vec::new();
                qRes += &manageQuery(i,&mut res,false,1);
                qRes += " ";
            }
            qRes += "#";
            for i in (&search.get_request()).split_whitespace()
            {
                let mut res = Vec::new();
                qRes += "[word: ";
                qRes += i;
                qRes += ",results: {";
                qRes += &manageQuery(i,&mut res,false,5);
                qRes += "}]";
            }
            //qRes = manageQuery(&search.request,&mut res,false,1);
        }
        qRes += "#";
        qRes += &search.get_results();
        let message = Flash::success(Redirect::to(uri!(hello(name))),
        "".to_owned()+&qRes);
        return Ok(message);
    }
    let error_messages: Vec<String> = form.context.errors().map(|error| {
        let name = error.name.as_ref().unwrap().to_string();
        let description = error.to_string();
        format!("'{}' {}", name, description)
    }).collect();
    Err(Template::render("root", context! {
        request : form.context.field_value("request"),
        number_results : form.context.field_value("number_results"),
        request_error : form.context.field_errors("request").count() > 0,
        number_results_error : form.context.field_errors("number_results")
        .count() >= 0,
        errors: error_messages
    }))
}

fn init_dic(l:u32)
{
    let mut tot = 0;
    let milliseconds_timestamp: u128 = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis();
    // ID, int value of letter, bool endWord, pointers (l,m,r)
    //let mut Dic: &mut [[u32; 6]; 1000000] = &mut [[0u32; 6]; 100000];
    unsafe {
       let m = Dic[0][1];
       for i in 0..m
       {
          Dic[i as usize] = [0, 0, 0, 0, 0, 0];
       }
       Dic[0] = [0, 2, 0, 0, 0, 0]; // Default
       Dic[1] = [1, 97, 0, 0, 0, 0];
    }
    if l == 0
    {
       tot += loadDic("src/postquery/Dic-en.txt");
    }
    else if l == 1
    {
        tot += loadDic("src/postquery/Dic-fr.txt");
    }
    else
    {
        tot += loadDic("src/postquery/Dic-fr.txt");
        tot += loadDic("src/postquery/Dic-en.txt");
        unsafe{
        let h = DicDb.clone();
        for (key,value) in h
        {
            tot += 1;
            addWordDatabase(&key,value);
        }
        }
    }
    printMemory(2);
    let milliseconds_timestamp2: u128 = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_millis();
    println!("=============== Dictionnary Built ===============");
    println!("words: {}",tot);
    println!("time: {}ms",(milliseconds_timestamp2-milliseconds_timestamp));
    unsafe {
       let saved = ((9*tot*4*2*2) -  21*Dic[0][1] ) as i32;
       println!("size: {} bytes ({} ko)",21*Dic[0][1],(21*Dic[0][1]/8000));
       println!("saved: {} bytes ({} ko)",saved,saved/8000);
    }
    println!("==================================================");
}

/** ON START FUNCTION **/

/*
#[launch]
fn rocket() -> _ {   
   unsafe{
    testSuite();
    DicDb = search_all_word();
    init_dic(2);
    insert(1, "ferrari".to_string(), 0, 0,8);
    insert(1, "ferraille".to_string(), 0, 0,3);
    insert(1, "ferry".to_string(), 0, 0,5);
    //debug(Dic);
    open::that("localhost:8000");
    rocket::build()
    // add templating system
    .attach(Template::fairing())
    // serve content from disk
    .mount("/public", FileServer::new(relative!("/public"),
    Options::Missing | Options::NormalizeDirs))
    // register routes
    .mount("/", routes![root, create, hello,root_fr,root_en,root_home])
   }
}
*/
/*
pub fn main()
{
    add_in_data_base("https://login.rosettastone.com/#/login");
}
*/

/** GENERATE RESULTS FOR A SINGLE WORD QUERY **/
fn manageQuery(s : &str,mut results : &mut Vec<(String,u32)>,debug:bool,mut nb:u32 ) -> String
{ unsafe{
    let mut qRes = String::new();
    if s != ""{

    // check if special char
    for i in s.chars()
    {
        if !( (i >= 'A' && i <= 'Z') || (i >= 'a' && i <= 'z') )
        {
           qRes += s;
           return qRes;
        }
    }
    let a = checkWord(&s,false,&mut results);
    println!("[WORDISVALID]: {}",a==0);
    if a == 0
    {
        qRes += s;
        qRes += ",";
        nb -= 1;
    }
    if a != 2
    {
       let mut s = results.iter().count();
       let mut l = s;
       if l > nb as usize
       {l=nb as usize;}
       for i in 0..l
       {
         let mut index = 0;
         let mut max = results[0].1;
         for j in 1..s
         {
            if results[j].1>max
            {
                max = results[j].1;
                index = j;
            }
         }
         println!("[AUTOCOMPLETION_{}]: {}",i,results[index].0);
         qRes += &results[index].0;
         qRes += ",";
         results.remove(index);
         s -= 1;
       }
    }
    else
    {
       let s = results.iter().count();
       if s == 0
       {
          println!("[CORRECTION]: No correction found !");
       }
       else
       {
       let mut l = s;
       let mut index = 0;
       let mut max = results[0].1;
       for j in 1..s
       {
            if results[j].1>max
            {
                max = results[j].1;
                index = j;
            }
       }
       println!("[CORRECTION]: {}",results[index].0);
       qRes += &results[index].0;
       qRes += ",";
       }
    }
    }
    qRes.pop();
    return qRes;
}
}

/** LOAD DICTIONNARY FROM FILE **/
fn loadDic(s:&str) -> u32 {
   println!("loading Dictionnary");

    let mut a = 0;
    for line in read_to_string(s).unwrap().lines() {
        unsafe{
           addWord(line);
           a += 1;
        }
    }
    println!("Dictionnary loaded");
    return a;
}

/** PRINT DEBUG RESULTS FROM A FILE **/
fn debug() { unsafe{
    for line in read_to_string("Debug.txt").unwrap().lines() {
        println!("Debug for word: {}", line);
        let mut res = Vec::new();
        checkWord(line, false,&mut res);
    }
}}

/** TOOLS **/

/** Add a word in dictionnary **/
fn addWord(s: &str) {
let mut a = 0;
for i in s.to_string().to_lowercase().chars()
{
    if i<'a' || i>'z'
    {
       a = 1;
    }
}
if s == ""
{
    a = 1;
}
unsafe{
    if a == 0 {
       insert(1, s.to_string().to_lowercase(), 0, 0,1);
    }
}}

/** Add a word in dictionnary with database tf-idf value **/
fn addWordDatabase(s: &str, i: i32) {
let mut a = 0;
for i in s.to_string().to_lowercase().chars()
{
    if i<'a' || i>'z'
    {
       a = 1;
    }
}
if s == ""
{
    a = 1;
}
unsafe{
    if a == 0 {
       insert(1, s.to_string().to_lowercase(), 0, 0,i as u32);
    }
}}

/** Find a word in dictionnary **/
fn findWord(s: &str) -> bool { unsafe{
    return search(1, s.to_string(), 0);
}}

/** List the entire dictionnary **/
fn listAllWords() { unsafe{
    listAll( 1, "".to_string(), 0);
}}

/** Suggest words from a given prefix **/
fn suggest( s: &str, mut results : &mut Vec<(String,u32)>,debug:bool) { unsafe{
    let a = getNodeID( s.to_string());
    if a == 0 {
        if debug {println!(">> no suggestions found !");}
    } else {
        traverse(
            Dic[a][4].try_into().unwrap(),
            s.to_string().clone(),
            s.to_string().chars().count().try_into().unwrap(),
            s.to_string().clone(),results,debug
        );
    }
}}

/** Check if a word exists, or try a suggestion or correction
0: word found (vec contains autocompletion)
1: word not found, but suggestions are given
2: word not found, but corrections are given
**/
fn checkWord( s: &str, debug: bool,
mut results : &mut Vec<(String,u32)>) -> usize { unsafe{
    let mut r = 1;
    if findWord(&s.to_lowercase()) {
        if debug {println!(">> {} is a word", s);}
        r = 0;
    }
    let a = getNodeID(s.to_string());
    if a != 0 {
        suggest(&s.to_lowercase(), results,debug);
        return r;
    } else {
      correct(&s.to_lowercase(), debug,results);
      return 2;
    }
}}

/** Try if a word exists in dictionnary as a correction **/
fn tryInDic( s: String, debug: bool,
mut results : &mut Vec<(String,u32)>) -> u32 { unsafe{
    let a = findWord(&s.trim());
    if debug {
        println!("testing if {} is in Dic => {}", s, a);
        if a {
            return 1;
        }
    } else if a {
        if debug {println!(">> {} is a possible correction !", s);}
        results.push((s.clone(),Dic[getNodeID(s) as usize][2]));
        return 1;
    }
    return 0;
}}

/** DEBUG **/

/** Display all memory info and values**/
fn printMemory(size: usize) { unsafe{
    println!();
    println!("================== MEMORY  STATUS ==================");
    println!("ID status: {}", Dic[0][0]);
    println!("Nodes: {}", Dic[0][1]);
    println!("Root: {}", Dic[0][2] == 0);
    println!("OutputL: {}", Dic[0][3]);
    println!("OutputE: {}", Dic[0][4]);
    println!("OutputR: {}", Dic[0][5]);
    println!("================== MEMORY DISPLAY ==================");
    println!("NodeID | Value | isEnd | Left | Equal | Right");
    println!("================== MEMORY  VALUES ==================");
    for i in 1..size {
        if Dic[i][1] == 0 {
            break;
        }
        for j in 0..6 {
            print!("{}  ", Dic[i][j]);
        }
        println!();
    }
    println!("====================================================");
}}

fn testSuite() { unsafe{
    Dic[0] = [0, 2, 0, 0, 0, 0]; // Default
    Dic[1] = [1, 97, 0, 0, 0, 0];
    println!("==================   TEST SUITE   ==================");

    println!("\n#ADDING WORDS#\n");
    addWord("cat");
    println!("Added: /cat/");
    addWord("cats");
    println!("Added: /cats/");
    addWord("bug");
    println!("Added: /bug/");
    addWord("up");
    println!("Added: /up/");

    println!("\n#LISTING WORDS#\n");
    listAllWords();

    println!("\n#SEARCHING WORDS#\n");

    println!("Contains: /cat/  {}", findWord("cat"));
    println!("Contains: /cats/ {}", findWord("cats"));
    println!("Contains: /bug/  {}", findWord("bug"));
    println!("Contains: /up/   {}", findWord("up"));
    println!("Contains: /uwu/  {}", findWord("uwu"));
    println!("Contains: /bu/   {}", findWord("bu"));

    let mut res = Vec::new();

    println!("\n#SUGGESTIONS#\n");
    println!("Suggestions for : /c/");
    suggest("c",&mut res,true);
    println!("Suggestions for : /ca/");
    suggest("ca",&mut res,true);
    println!("Suggestions for : /b/");
    suggest("b",&mut res,true);
    println!("Suggestions for : /bat/");
    suggest("bat",&mut res,true);

    println!("\n#CORRECTION#\n");
    println!("Corrections for : /caat/");
    correct("caat", false,&mut res);
    println!("Debug corrections for : /bu/");
    correct("bu", true,&mut res);

    println!("\n#CHECK#\n");
    println!("WordCheck for : /cat/");
    checkWord("cat", false,&mut res);
    println!("WordCheck for : /ca/");
    checkWord("ca", false,&mut res);
    println!("WordCheck for : /ug/");
    checkWord("ug", false,&mut res);
    println!("WordCheck for : /cazt/");
    checkWord("cazt", false,&mut res);
    println!("WordCheck for : /op/");
    checkWord("op", false,&mut res);
    println!("WordCheck for : /caaat/");
    checkWord("caaat", false,&mut res);

    println!("\n#PRITING MEMORY#\n");
    printMemory(100);
}}

/** CORE **/

/** Insert a word node by node in a ternary search tree **/
fn insert(
    mut node: usize,
    word: String,
    index: u32,
    mut f: usize,
    occ: u32
) -> u32 { unsafe{
    if node == 0 && f == 0 {
        node = Dic[0][1] as usize;
        Dic[0][1] += 1;
        Dic[node][0] = node as u32;
        Dic[node][1] = word.chars().nth(index.try_into().unwrap()).unwrap()
        as u32;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32)
    < Dic[node][1] {
        Dic[node][3] = insert( Dic[node][3] as usize, word, index, 0,occ);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32)
    > Dic[node][1] {
        Dic[node][5] = insert( Dic[node][5] as usize, word, index, 0,occ);
    } else {
        if index < (word.chars().count() as u32) - 1 {
            Dic[node][4] = insert( Dic[node][4] as usize, word, index + 1, 0,occ);
        } else {
            Dic[node][2] = occ;
        }
    }
    return node.try_into().unwrap();
}}

/**  Find a word in a ternary search tree  **/
fn search( mut node: usize, word: String, index: u32) -> bool { unsafe{
    if node == 0 {
        return false;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32)
    < Dic[node][1] {
        return search( Dic[node][3].try_into().unwrap(), word, index);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32)
    > Dic[node][1] {
        return search( Dic[node][5].try_into().unwrap(), word, index);
    } else {
        if index == (word.chars().count() as u32) - 1 {
            return Dic[node][2] >= 1;
        } else {
            return search( Dic[node][4].try_into().unwrap(), word, index + 1);
        }
    }
}}

/**  List all words from node **/
fn listAll( mut node: usize, mut word: String, mut l: usize) { unsafe{
    if node == 0 {
        return;
    }
    if Dic[node][3] as usize !=node{
    listAll( Dic[node][3].try_into().unwrap(), word.clone(), l);}
    word.push(char::from_u32(Dic[node][1]).unwrap());
    l += 1;
    if Dic[node][2] >= 1 {
        println!("{}", word);
    }
    let mut word2 = String::new();
    for i in 0..l - 1 {
        word2.push(word.chars().nth(i).unwrap());
    }
    l -= 1;
    let mut word3 = word2.clone();
    let l2 = l + 1;
    word3.push(char::from_u32(Dic[node][1]).unwrap());
    listAll( Dic[node][4].try_into().unwrap(), word3, l2);
    listAll( Dic[node][5].try_into().unwrap(), word2, l);
}}

/**  Find node id given a prefix  **/
fn getNodeID( prefix: String) -> usize { unsafe{
    let mut node = 1;
    let mut index = 0;
    while node != 0 && index < prefix.chars().count() {
        if (prefix.chars().nth(index.try_into().unwrap()).unwrap() as u32)
        < Dic[node][1] {
            node = Dic[node][3] as usize;
        }
        else if (prefix.chars().nth(index.try_into().unwrap()).unwrap() as u32)
        > Dic[node][1] {
            node = Dic[node][5] as usize;
        } else {
            index += 1;
            if index < prefix.chars().count() {
                node = Dic[node][4] as usize;
            }
        }
    }
    return node;
}}

/** Return all autocompletion suggestions **/
fn traverse(
    mut node: usize,
    prefix: String,
    l: u32,
    base: String, mut results : &mut Vec<(String,u32)>,
    debug : bool
) { unsafe{
    if node != 0 {
        traverse(
            Dic[node][3].try_into().unwrap(),
            prefix.clone(),
            l,
            base.clone(),results,debug
        );
        let mut prefix2 = prefix.clone();
        prefix2.push(char::from_u32(Dic[node][1]).unwrap());
        if Dic[node][2] >= 1 {
            if debug {println!(">> suggestion: {}", prefix2);}
            results.push((prefix2.clone(),Dic[node][2]));
        }
        traverse(
            Dic[node][4].try_into().unwrap(),
            prefix2,
            l + 1,
            base.clone(),results,debug
        );
        traverse(
            Dic[node][5].try_into().unwrap(),
            prefix,
            l,
            base.clone(),results,debug
        );
    }
}}

/** Find all possible corrections for a word **/
fn correct(s: &str, debug: bool,
mut results : &mut Vec<(String,u32)>) { unsafe{

    let mut c = 0;
    /** Swap **/
    let mut base = s.to_string();
    for i in 0..base.chars().count() - 1 {
        let mut test = String::new();
        for j in 0..i {
            test.push(base.chars().nth(j).unwrap());
        }
        test.push(base.chars().nth(i + 1).unwrap());
        test.push(base.chars().nth(i).unwrap());
        for j in i + 2..base.chars().count() {
            test.push(base.chars().nth(j).unwrap());
        }
        c += tryInDic( test, debug,results);
    }

    /** Deletion **/
    for i in 0..base.chars().count() {
        let mut test = String::new();
        for j in 0..base.chars().count() {
            if i != j {
                test.push(base.chars().nth(j).unwrap());
            }
        }
        c += tryInDic(test, debug,results);
    }

    /** Replacement **/
    for i in 0..base.chars().count() {
        for k in 0..26 {
            let mut test = String::new();
            for j in 0..base.chars().count() {
                if i != j {
                    test.push(base.chars().nth(j).unwrap());
                } else {
                    test.push(char::from_u32('a' as u32 + k).unwrap());
                }
            }
            c += tryInDic( test, debug,results);
        }
    }

    /** Insertion **/
    base.push(' ');
    for i in 0..base.chars().count() {
        for k in 0..26 {
            let mut test = String::new();
            for j in 0..base.chars().count() {
                if i != j {
                    test.push(base.chars().nth(j).unwrap());
                } else {
                    test.push(char::from_u32('a' as u32 + k).unwrap());
                    test.push(base.chars().nth(j).unwrap());
                }
            }
            c += tryInDic( test, debug,results);
        }
    }
    if c == 0 && debug{
        println!(">> no corrections found =/");
    }
}}


fn main()
{
    //add_in_data_base("https://login.rosettastone.com/#/login");
    println!("Crawler's Tests");
    crawler_tests();
    let list = crawler(30000);
    for elt in list
    {
        add_in_data_base(&elt);
    }
    //println!();
    //println!("Scraper's Tests");
    //scraper_tests();
    //println!();
    //println!("Parser's Tests");
    //parser_tests();
    //println!();

    /*
    println!("Queries's Tests");
    query_tests();
    println!();*/
}


