use self::models::*;
use diesel::prelude::*;
use sqlquery::*;


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

#[derive(Debug)]
pub struct Parse
{
    pub  key_word : Vec<String>,
    pub  no_word : Vec<String>,
    pub  link : Vec<String>,
    pub  no_link : Vec<String>,
    pub  word_or : Vec<String>,
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

fn main()
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
