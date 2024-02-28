/** Imports **/
use std::fs::read_to_string;

/** MAIN **/

fn main() {
    // ID, int value of letter, bool endWord, pointers (l,m,r)
    let mut Dic: &mut [[u32; 6]; 100000] = &mut [[0u32; 6]; 100000];
    Dic[0] = [0, 2, 0, 0, 0, 0]; // Default
    Dic[1] = [1, 97, 0, 0, 0, 0];
    testSuite(Dic);
    loadDic(Dic);
    debug(Dic);
}

fn loadDic(mut Dic: &mut [[u32; 6]; 100000]) {
    for line in read_to_string("Dic.txt").unwrap().lines() {
        addWord(Dic, line);
    }
}

fn debug(mut Dic: &mut [[u32; 6]; 100000]) {
    for line in read_to_string("Debug.txt").unwrap().lines() {
        println!("Debug for word: {}", line);
        checkWord(Dic, line, false);
    }
}

/** TOOLS **/

/** Add a word in dictionnary **/
fn addWord(mut Dic: &mut [[u32; 6]; 100000], s: &str) {
    insert(Dic, 1, s.to_string(), 0, 0);
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
fn suggest(mut Dic: &mut [[u32; 6]; 100000], s: &str) {
    let a = getNodeID(Dic, s.to_string());
    if a == 0 {
        println!(">> no suggestions found !");
    } else {
        traverse(
            Dic,
            Dic[a][4].try_into().unwrap(),
            s.to_string().clone(),
            s.to_string().chars().count().try_into().unwrap(),
            s.to_string().clone(),
        );
    }
}

/** Check if a word exists, or try a suggestion or correction **/
fn checkWord(mut Dic: &mut [[u32; 6]; 100000], s: &str, debug: bool) {
    if findWord(Dic, s) {
        println!(">> {} is a word", s);
    }
    let a = getNodeID(Dic, s.to_string());
    if a != 0 {
        suggest(Dic, s);
    } else {
        correct(Dic, s, debug)
    }
}

/** Try if a word exists in dictionnary as a correction **/
fn tryInDic(mut Dic: &mut [[u32; 6]; 100000], s: String, debug: bool) -> u32 {
    let a = findWord(Dic, &s.trim());
    if debug {
        println!("testing if {} is in Dic => {}", s, a);
        if a {
            return 1;
        }
    } else if a {
        println!(">> {} is a possible correction !", s);
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

    println!("\n#SUGGESTIONS#\n");
    println!("Suggestions for : /c/");
    suggest(Dic, "c");
    println!("Suggestions for : /ca/");
    suggest(Dic, "ca");
    println!("Suggestions for : /b/");
    suggest(Dic, "b");
    println!("Suggestions for : /bat/");
    suggest(Dic, "bat");

    println!("\n#CORRECTION#\n");
    println!("Corrections for : /caat/");
    correct(Dic, "caat", false);
    println!("Debug corrections for : /bu/");
    correct(Dic, "bu", true);

    println!("\n#CHECK#\n");
    println!("WordCheck for : /cat/");
    checkWord(Dic, "cat", false);
    println!("WordCheck for : /ca/");
    checkWord(Dic, "ca", false);
    println!("WordCheck for : /ug/");
    checkWord(Dic, "ug", false);
    println!("WordCheck for : /cazt/");
    checkWord(Dic, "cazt", false);
    println!("WordCheck for : /op/");
    checkWord(Dic, "op", false);
    println!("WordCheck for : /caaat/");
    checkWord(Dic, "caaat", false);

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
) -> u32 {
    if node == 0 && f == 0 {
        node = Dic[0][1] as usize;
        Dic[0][1] += 1;
        Dic[node][0] = node as u32;
        Dic[node][1] = word.chars().nth(index.try_into().unwrap()).unwrap() as u32;
    }
    if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) < Dic[node][1] {
        Dic[node][3] = insert(Dic, Dic[node][3] as usize, word, index, 0);
    } else if (word.chars().nth(index.try_into().unwrap()).unwrap() as u32) > Dic[node][1] {
        Dic[node][5] = insert(Dic, Dic[node][5] as usize, word, index, 0);
    } else {
        if index < (word.chars().count() as u32) - 1 {
            Dic[node][4] = insert(Dic, Dic[node][4] as usize, word, index + 1, 0);
        } else {
            Dic[node][2] = 1;
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
            return Dic[node][2] == 1;
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
    if Dic[node][2] == 1 {
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

/** Print all autocompletion suggestions **/
fn traverse(
    mut Dic: &mut [[u32; 6]; 100000],
    mut node: usize,
    prefix: String,
    l: u32,
    base: String,
) {
    if node != 0 {
        traverse(
            Dic,
            Dic[node][3].try_into().unwrap(),
            prefix.clone(),
            l,
            base.clone(),
        );
        let mut prefix2 = prefix.clone();
        prefix2.push(char::from_u32(Dic[node][1]).unwrap());
        if Dic[node][2] == 1 {
            println!(">> suggestion: {}", prefix2);
        }
        traverse(
            Dic,
            Dic[node][4].try_into().unwrap(),
            prefix2,
            l + 1,
            base.clone(),
        );
        traverse(
            Dic,
            Dic[node][5].try_into().unwrap(),
            prefix,
            l,
            base.clone(),
        );
    }
}

/** Find all possible corrections for a word **/
fn correct(mut Dic: &mut [[u32; 6]; 100000], s: &str, debug: bool) {
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
        c += tryInDic(Dic, test, debug);
    }

    /** Deletion **/
    for i in 0..base.chars().count() {
        let mut test = String::new();
        for j in 0..base.chars().count() {
            if i != j {
                test.push(base.chars().nth(j).unwrap());
            }
        }
        c += tryInDic(Dic, test, debug);
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
            c += tryInDic(Dic, test, debug);
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
            c += tryInDic(Dic, test, debug);
        }
    }
    if c == 0 {
        println!(">> no corrections found =/");
    }
}
