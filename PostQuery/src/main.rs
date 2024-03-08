/** Imports **/
use console::Term;
use std::fs::read_to_string;

/** MAIN **/

fn main() {
    // ID, int value of letter, bool endWord, pointers (l,m,r)
    let mut Dic: &mut [[u32; 6]; 100000] = &mut [[0u32; 6]; 100000];
    Dic[0] = [0, 2, 0, 0, 0, 0]; // Default
    Dic[1] = [1, 97, 0, 0, 0, 0];
    //testSuite(Dic);
    loadDic(Dic);

    insert(Dic, 1, "lea".to_string(), 0, 0,2);
    insert(Dic, 1, "leo".to_string(), 0, 0,3);
    
    //debug(Dic);
    println!("====================================================");
    //println!("start typing: ");
    let mut s = String::new();
    let mut p = String::new();
    let stdout = Term::buffered_stdout();
    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                '\n' => break 'game_loop,
                ' ' => {s.push(' ');p+=&s;s=String::new();},
                '-' => {s.pop();},
                '+' => {p.pop();},
                _ => {
                // println!("{} {}",character,character as u32);
                s.push(character);
                },
            }
        }
        println!("your sentence: {} ",p);
        println!("your word: {} ",s);
        let mut res = Vec::new();
        manageQuery(Dic,&s,&mut res,false);
        println!("====================================================");
    }
}

fn manageQuery(mut Dic: &mut [[u32; 6]; 100000], s : &str,mut results : &mut Vec<(String,u32)>,debug:bool )
{
    if s != ""{
    let a = checkWord(Dic,&s,false,&mut results);
    println!("[WORDISVALID]: {}",a==0);
    if a != 2
    {
       let mut s = results.iter().count();
       let mut l = s;
       if l > 5
       {l=5;}
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
       }
    }
    }
}

fn loadDic(mut Dic: &mut [[u32; 6]; 100000]) {
    for line in read_to_string("Dic.txt").unwrap().lines() {
        addWord(Dic, line);
    }
}

fn debug(mut Dic: &mut [[u32; 6]; 100000]) {
    for line in read_to_string("Debug.txt").unwrap().lines() {
        println!("Debug for word: {}", line);
        let mut res = Vec::new();
        checkWord(Dic, line, false,&mut res);
    }
}

/** TOOLS **/

/** Add a word in dictionnary **/
fn addWord(mut Dic: &mut [[u32; 6]; 100000], s: &str) {
    insert(Dic, 1, s.to_string(), 0, 0,1);
}

/** Find a word in dictionnary **/
fn findWord(mut Dic: &mut [[u32; 6]; 100000], s: &str) -> bool {
    return search(Dic, 1, s.to_string(), 0);
}

/** List the entire dictionnary **/
fn listAllWords(mut Dic: &mut [[u32; 6]; 100000]) {
    listAll(Dic, 1, "".to_string(), 0);
}

/** Suggest words from a given prefix **/
fn suggest(mut Dic: &mut [[u32; 6]; 100000], s: &str, mut results : &mut Vec<(String,u32)>,debug:bool) {
    let a = getNodeID(Dic, s.to_string());
    if a == 0 {
        if debug {println!(">> no suggestions found !");}
    } else {
        traverse(
            Dic,
            Dic[a][4].try_into().unwrap(),
            s.to_string().clone(),
            s.to_string().chars().count().try_into().unwrap(),
            s.to_string().clone(),results,debug
        );
    }
}

/** Check if a word exists, or try a suggestion or correction
0: word found (vec contains autocompletion)
1: word not found, but suggestions are given
2: word not found, but corrections are given
**/
fn checkWord(mut Dic: &mut [[u32; 6]; 100000], s: &str, debug: bool,
mut results : &mut Vec<(String,u32)> ) -> usize {
    let mut r = 1;
    if findWord(Dic, s) {
        if debug {println!(">> {} is a word", s);}
        r = 0;
    }
    let a = getNodeID(Dic, s.to_string());
    if a != 0 {
        suggest(Dic, s, results,debug);
        return r;
    } else {
      correct(Dic, s, debug,results);
      return 2;
    }
}

/** Try if a word exists in dictionnary as a correction **/
fn tryInDic(mut Dic: &mut [[u32; 6]; 100000], s: String, debug: bool,
mut results : &mut Vec<(String,u32)>) -> u32 {
    let a = findWord(Dic, &s.trim());
    if debug {
        println!("testing if {} is in Dic => {}", s, a);
        if a {
            return 1;
        }
    } else if a {
        if debug {println!(">> {} is a possible correction !", s);}
        results.push((s.clone(),Dic[getNodeID(Dic, s) as usize][2]));
        return 1;
    }
    return 0;
}

/** DEBUG **/

/** Display all memory info and values**/
fn printMemory(mut Dic: &mut [[u32; 6]; 100000], size: usize) {
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
}

fn testSuite(mut Dic: &mut [[u32; 6]; 100000]) {
    println!("==================   TEST SUITE   ==================");

    println!("\n#ADDING WORDS#\n");
    addWord(Dic, "cat");
    println!("Added: /cat/");
    addWord(Dic, "cats");
    println!("Added: /cats/");
    addWord(Dic, "bug");
    println!("Added: /bug/");
    addWord(Dic, "up");
    println!("Added: /up/");

    println!("\n#LISTING WORDS#\n");
    listAllWords(Dic);

    println!("\n#SEARCHING WORDS#\n");

    println!("Contains: /cat/  {}", findWord(Dic, "cat"));
    println!("Contains: /cats/ {}", findWord(Dic, "cats"));
    println!("Contains: /bug/  {}", findWord(Dic, "bug"));
    println!("Contains: /up/   {}", findWord(Dic, "up"));
    println!("Contains: /uwu/  {}", findWord(Dic, "uwu"));
    println!("Contains: /bu/   {}", findWord(Dic, "bu"));

    let mut res = Vec::new();
    
    println!("\n#SUGGESTIONS#\n");
    println!("Suggestions for : /c/");
    suggest(Dic, "c",&mut res,true);
    println!("Suggestions for : /ca/");
    suggest(Dic, "ca",&mut res,true);
    println!("Suggestions for : /b/");
    suggest(Dic, "b",&mut res,true);
    println!("Suggestions for : /bat/");
    suggest(Dic, "bat",&mut res,true);

    println!("\n#CORRECTION#\n");
    println!("Corrections for : /caat/");
    correct(Dic, "caat", false,&mut res);
    println!("Debug corrections for : /bu/");
    correct(Dic, "bu", true,&mut res);

    println!("\n#CHECK#\n");
    println!("WordCheck for : /cat/");
    checkWord(Dic, "cat", false,&mut res);
    println!("WordCheck for : /ca/");
    checkWord(Dic, "ca", false,&mut res);
    println!("WordCheck for : /ug/");
    checkWord(Dic, "ug", false,&mut res);
    println!("WordCheck for : /cazt/");
    checkWord(Dic, "cazt", false,&mut res);
    println!("WordCheck for : /op/");
    checkWord(Dic, "op", false,&mut res);
    println!("WordCheck for : /caaat/");
    checkWord(Dic, "caaat", false,&mut res);

    println!("\n#PRITING MEMORY#\n");
    printMemory(Dic, 100);
}

/** CORE **/

/** Insert a word node by node in a ternary search tree **/
fn insert(
    mut Dic: &mut [[u32; 6]; 100000],
    mut node: usize,
    word: String,
    index: u32,
    mut f: usize,
    occ: u32
) -> u32 {
    if node == 0 && f == 0 {
        node = Dic[0][1] as usize;
        Dic[0][1] += 1;
        Dic[node][0] = node as u32;
        Dic[node][1] = word.chars().nth(index.try_into().unwrap()).unwrap() as u32;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
        Dic[node][3] = insert(Dic, Dic[node][3] as usize, word, index, 0,occ);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
        Dic[node][5] = insert(Dic, Dic[node][5] as usize, word, index, 0,occ);
    } else {
        if index < (word.chars().count() as u32) - 1 {
            Dic[node][4] = insert(Dic, Dic[node][4] as usize, word, index + 1, 0,occ);
        } else {
            Dic[node][2] = occ;
        }
    }
    return node.try_into().unwrap();
}

/**  Find a word in a ternary search tree  **/
fn search(mut Dic: &mut [[u32; 6]; 100000], mut node: usize, word: String, index: u32) -> bool {
    if node == 0 {
        return false;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
        return search(Dic, Dic[node][3].try_into().unwrap(), word, index);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
        return search(Dic, Dic[node][5].try_into().unwrap(), word, index);
    } else {
        if index == (word.chars().count() as u32) - 1 {
            return Dic[node][2] >= 1;
        } else {
            return search(Dic, Dic[node][4].try_into().unwrap(), word, index + 1);
        }
    }
}

/**  List all words from node **/
fn listAll(mut Dic: &mut [[u32; 6]; 100000], mut node: usize, mut word: String, mut l: usize) {
    if node == 0 {
        return;
    }
    if Dic[node][3] as usize !=node{
    listAll(Dic, Dic[node][3].try_into().unwrap(), word.clone(), l);}
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
    listAll(Dic, Dic[node][4].try_into().unwrap(), word3, l2);
    listAll(Dic, Dic[node][5].try_into().unwrap(), word2, l);
}

/**  Find node id given a prefix  **/
fn getNodeID(mut Dic: &mut [[u32; 6]; 100000], prefix: String) -> usize {
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
}

/** Return all autocompletion suggestions **/
fn traverse(
    mut Dic: &mut [[u32; 6]; 100000],
    mut node: usize,
    prefix: String,
    l: u32,
    base: String, mut results : &mut Vec<(String,u32)>,
    debug : bool
) {
    if node != 0 {
        traverse(
            Dic,
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
            Dic,
            Dic[node][4].try_into().unwrap(),
            prefix2,
            l + 1,
            base.clone(),results,debug
        );
        traverse(
            Dic,
            Dic[node][5].try_into().unwrap(),
            prefix,
            l,
            base.clone(),results,debug
        );
    }
}

/** Find all possible corrections for a word **/
fn correct(mut Dic: &mut [[u32; 6]; 100000], s: &str, debug: bool,
mut results : &mut Vec<(String,u32)>) {
    
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
        c += tryInDic(Dic, test, debug,results);
    }

    /** Deletion **/
    for i in 0..base.chars().count() {
        let mut test = String::new();
        for j in 0..base.chars().count() {
            if i != j {
                test.push(base.chars().nth(j).unwrap());
            }
        }
        c += tryInDic(Dic, test, debug,results);
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
            c += tryInDic(Dic, test, debug,results);
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
            c += tryInDic(Dic, test, debug,results);
        }
    }
    if c == 0 && debug{
        println!(">> no corrections found =/");
    }
}
