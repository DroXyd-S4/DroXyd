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
    let len = test.key_word.len();
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
static mut RESDB: [[&str; 7]; 100] = [[""; 7]; 100];
static mut cReq: std::string::String = String::new();

/** RENDER HOME PAGE **/
#[get("/")]
async fn root() -> Template {
    Template::render("root", context! { message: "Welcome to DroXyd !"})
}

/** RENDER HOME PAGE **/
#[get("/fr")]
async fn root_fr() -> Template {
    init_dic(1);
    Template::render("root", context! { message: "Welcome to DroXyd !"})
}

/** RENDER HOME PAGE **/
#[get("/en")]
async fn root_en() -> Template {
    init_dic(0);
    Template::render("root", context! { message: "Welcome to DroXyd !"})
}

/** RENDER HOME PAGE **/
#[get("/home")]
async fn root_home() -> Template {
    init_dic(2);
    Template::render("root", context! { message: "Welcome to DroXyd !"})
}

/** RENDER RESULTS PAGE **/
#[get("/hi?<request>")]
async fn hello(request: String, flash: Option<FlashMessage<'_>>) -> Template {

    /** PARSE REQUEST **/
    let message2 = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    let parts: Vec<_> = message2.split("#").collect();
    let bRequest = parts[0]; // Base Request
    let sRequest = parts[1]; // Suggestion of correction
    let qResults = parts[2]; // All results
    let mut nb = "10"; // Nb results
    if parts[3] != "" {nb = parts[3];}

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

    /** DEFAULT FILL **/
    unsafe {
    RESDB = [[""; 7]; 100];
    RESDB[0] =
    ["Wikipedia","https://fr.wikipedia.org/","info","wiki","data",
    "news","learn"];
    RESDB[1] =
    ["Youtube","https://m.youtube.com/","videos","discover","news",
    "shorts","followed"];
    RESDB[2] =
    ["Github","https://github.com/","projects","browse","develloper",
    "microsoft","login"];
    RESDB[3] =
    ["Instagram","https://instagram.com/","discover","post","foryou",
    "videos","followed"];
    RESDB[4] =
    ["Wikihow","https://fr.wikihow.org/","tutorials","wiki","learn",
    "news","trending"];
    RESDB[5] =
    ["testWebsite","https://test.com/","KeywordA","KeywordB","KeywordC",
    "KeywordD","KeywordE"];
    RESDB[6] =
    ["Wikipedia","https://fr.wikipedia.org/","info","wiki","data",
    "news","learn"];
    RESDB[7] =
    ["Wikipedia","https://fr.wikipedia.org/","info","wiki","data",
    "news","learn"];
    RESDB[8] =
    ["Wikipedia","https://fr.wikipedia.org/","info","wiki","data",
    "news","learn"];
    RESDB[9] =
    ["Wikipedia","https://fr.wikipedia.org/","info","wiki","data",
    "news","learn"];

    /** PARSE RESULTS/PAGE **/
    if nb.parse::<i32>().unwrap() > 0
    {R0 = getParsedRes(RESDB[0]);}
    if nb.parse::<i32>().unwrap() > 1
    {R1 = getParsedRes(RESDB[1]);}
    if nb.parse::<i32>().unwrap() > 2
    {R2 = getParsedRes(RESDB[2]);}
    if nb.parse::<i32>().unwrap() > 3
    {R3 = getParsedRes(RESDB[3]);}
    if nb.parse::<i32>().unwrap() > 4
    {R4 = getParsedRes(RESDB[4]);}
    if nb.parse::<i32>().unwrap() > 5
    {R5 = getParsedRes(RESDB[5]);}
    if nb.parse::<i32>().unwrap() > 6
    {R6 = getParsedRes(RESDB[6]);}
    if nb.parse::<i32>().unwrap() > 7
    {R7 = getParsedRes(RESDB[7]);}
    if nb.parse::<i32>().unwrap() > 8
    {R8 = getParsedRes(RESDB[8]);}
    if nb.parse::<i32>().unwrap() > 9
    {R9 = getParsedRes(RESDB[9]);}

    /** DISPLAY RESULTS INFO **/
    let mut f = 0;
    for i in 2..7
    {
        for j in 0..nb.parse::<i32>().unwrap() as usize
        {
             if !(bRequest.contains(RESDB[j][i]))
             {
                f += 1;
                SUGG += RESDB[j][i];
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

    let parts2: Vec<_> = SUGG.split("#").collect();
    SUGG_A = parts2[0].to_string();
    SUGG_B = parts2[1].to_string();
    SUGG_C = parts2[2].to_string();
    SUGG_D = parts2[3].to_string();
    SUGG_E = parts2[4].to_string();

    /** BROWSE LINKS **/
    unsafe{
        link0 += RESDB[0][1];
        link1 += RESDB[1][1];
        link2 += RESDB[2][1];
        link3 += RESDB[3][1];
        link4 += RESDB[4][1];
        link5 += RESDB[5][1];
        link6 += RESDB[6][1];
        link7 += RESDB[7][1];
        link8 += RESDB[8][1];
        link9 += RESDB[9][1];
    }

    /** RENDER **/
    Template::render("hello", context!
    {bRequest ,sRequest, qResults,
    SUGG_A,SUGG_B,SUGG_C,SUGG_D,SUGG_E,
    R0,R1,R2,R3,R4,R5,R6,R7,R8,R9,
    link0,link1,link2,link3,link4,link5,link6,link7,link8,link9})
}

/** PARSE RESULTS ON PAGE **/
fn getParsedRes(l: [&str;7]) -> String
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
     res += ",";
     res += &l[5];
     res += ",";
     res += &l[6];
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
       tot += loadDic("Dic-en.txt");
    }
    else if l == 1
    {
        tot += loadDic("Dic-fr.txt");
    }
    else
    {
        tot += loadDic("Dic-fr.txt");
        tot += loadDic("Dic-en.txt");
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
#[launch]
fn rocket() -> _ {
   unsafe{
    testSuite();
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
}}

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

/*
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
*/

