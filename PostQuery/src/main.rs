/** Imports **/
use std::fs::read_to_string;
mod interface;

/** Rocket **/
mod models;
use rocket::{get, launch, post, routes, uri};
use rocket::form::{Contextual, Form};
use rocket::fs::{FileServer, Options, relative};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{context, Template};
use crate::models::SearchRequest;


/** MAIN **/
#[get("/")]
async fn root() -> Template {
    Template::render("root", context! { message: "Welcome to DroXyd !"})
}

#[get("/hi?<request>")]
async fn hello(request: String, flash: Option<FlashMessage<'_>>) -> Template {
    let message2 = flash.map_or_else(|| String::default(), |msg| msg.message().to_string());
    println!("{} {}",request,message2);
    let parts: Vec<_> = message2.split("#").collect();
    let bRequest = parts[0]; // Base Request
    let sRequest = parts[1]; // Suggestion of correction
    let qResults = parts[2]; // All results
    let nb = parts[3]; // Nb results
    let mut R0 = "Wikipedia |  https://fr.wikipedia.org/ | keyword1 | keyword2";
    if nb.parse::<i32>().unwrap() == 0
    {
        R0 = "";
    }
    //println!("render {} {} {} ",name,*message,qResults);
    Template::render("hello", context! {bRequest ,sRequest, qResults,R0})
    
}

#[post("/", data = "<form>")]
async fn create(form: Form<Contextual<'_, SearchRequest>>) -> Result<Flash<Redirect>, Template> {
    if let Some(ref search) = form.value {
        let name = format!("{} {}", search.request, search.results_number);
        let mut qRes = String::new();
        unsafe{
            qRes += &search.request;
            qRes += "#";
            for i in (&search.request).split_whitespace()
            {
                let mut res = Vec::new();
                qRes += &manageQuery(i,&mut res,false,1);
                qRes += " ";
            }
            qRes += "#";
            for i in (&search.request).split_whitespace()
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
        qRes += &search.results_number;
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
        number_results_error : form.context.field_errors("number_results").count() >= 0,
        errors: error_messages
    }))
}

static mut Dic: &mut [[u32; 6]; 100000] = &mut [[0u32; 6]; 100000];

#[launch]
fn rocket() -> _ {
   unsafe{
   
    // ID, int value of letter, bool endWord, pointers (l,m,r)
    //let mut Dic: &mut [[u32; 6]; 100000] = &mut [[0u32; 6]; 100000];
    Dic[0] = [0, 2, 0, 0, 0, 0]; // Default
    Dic[1] = [1, 97, 0, 0, 0, 0];
    //testSuite(Dic);
    loadDic();

    insert(1, "ferrari".to_string(), 0, 0,8);
    insert(1, "ferraille".to_string(), 0, 0,3);
    insert(1, "ferry".to_string(), 0, 0,5);


    //debug(Dic);

        rocket::build()
        // add templating system
        .attach(Template::fairing())
        // serve content from disk
        .mount("/public", FileServer::new(relative!("/public"), Options::Missing | Options::NormalizeDirs))
        // register routes
        .mount("/", routes![root, create, hello])
}}

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

fn loadDic() { unsafe{
    for line in read_to_string("Dic.txt").unwrap().lines() {
        addWord(line);
    }
}}

fn debug() { unsafe{
    for line in read_to_string("Debug.txt").unwrap().lines() {
        println!("Debug for word: {}", line);
        let mut res = Vec::new();
        checkWord(line, false,&mut res);
    }
}}

/** TOOLS **/

/** Add a word in dictionnary **/
fn addWord(s: &str) { unsafe{
    insert(1, s.to_string(), 0, 0,1);
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
    if findWord(s) {
        if debug {println!(">> {} is a word", s);}
        r = 0;
    }
    let a = getNodeID(s.to_string());
    if a != 0 {
        suggest(s, results,debug);
        return r;
    } else {
      correct(s, debug,results);
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
        Dic[node][1] = word.chars().nth(index.try_into().unwrap()).unwrap() as u32;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
        Dic[node][3] = insert( Dic[node][3] as usize, word, index, 0,occ);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
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
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
        return search( Dic[node][3].try_into().unwrap(), word, index);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
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
        if (prefix.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
            node = Dic[node][3] as usize;
        } else if (prefix.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
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
