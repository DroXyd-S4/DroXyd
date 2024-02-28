
use droxyd::*;

fn main() {
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
