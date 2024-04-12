#[derive(Debug)]
pub struct Parse
{
    pub  key_word : Vec<String>,
    pub  no_word : Vec<String>,
    pub  link : Vec<String>,
    pub  no_link : Vec<String>,
    pub  word_or : Vec<String>,
}


fn site(s: &str, x : usize) -> (usize, Parse)
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut n = x;
    let mut t = 0;
    let mut key = String::new();
    let mut l = String::new();
    let mut g = false;
    let mut j = 0;
    for i in s.chars()
    {
        if n == 0
        {
            //dbg!(i);
            j += 1;
            if i == ':'
            {  
                t += 1;
            }
            else if (i == ' ') & (g == false)
            {    
                t += 1;
                break;
            }
            else if i == '"'
            {
                if g
                {
                    t += 1;
                }
                g = true;
            }                
            else if t == 1 
            { 
                l.push(i);
            }
            else if t == 2                
            {
                key.push(i)
            }
               
            else if t == 3
            {  
                if key != ""
                {              
                    p.key_word.push(key.clone()); 
                }
                p.link.push(l.clone());
                t += 1;
                break;
            }
        }
        else
        {
            n -= 1;
        }
        
    }
    //dbg!(l.clone());
    //dbg!(t);
    if t == 1
    {
        p.link.push(l.clone());
        return (j-1,p);
    }
    if (t == 3) | (t == 2) 
    {
        if key != ""
        {
            //dbg!(l.clone());
            p.key_word.push(key.clone());
        }
        //dbg!(l.clone());
        p.link.push(l.clone());
    }
    return (j-1,p);
}

fn unsite(s: &str, x : usize) -> (usize, Parse)
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut n = x;
    let mut t = 0;
    let mut key = String::new();
    let mut l = String::new();
    let mut g = false;
    let mut j = 0;
    for i in s.chars()
    {
        if n == 0
        {
            //dbg!(i);
            j += 1;
            if i == ':'
            {  
                t += 1;
            }
            else if (i == ' ') & (g == false)
            {    
                t += 1;
                break;
            }
            else if i == '"'
            {
                if g
                {
                    t += 1;
                }
                g = true;
            }                
            else if t == 1 
            { 
                l.push(i);
            }
            else if t == 2                
            {
                key.push(i)
            }
               
            else if t == 3
            {  
                if key != ""
                {              
                    p.key_word.push(key.clone()); 
                }
                p.no_link.push(l.clone());
                t += 1;
                break;
            }
        }
        else
        {
            n -= 1;
        }
        
    }
    //dbg!(l.clone());
    //dbg!(t);
    if t == 1
    {
        p.no_link.push(l.clone());
        return (j-1,p);
    }
    if (t == 3) | (t == 2) 
    {
        if key != ""
        {
            //dbg!(l.clone());
            p.key_word.push(key.clone());
        }
        //dbg!(l.clone());
        p.no_link.push(l.clone());
    }
    return (j-1,p);
}


fn exclure(s: &str, x : usize) -> (usize, Parse)
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut n = x+1;
    let mut key = String::new();
    let mut j = 1;
    let mut t = false;
    for i in s.chars()
    {
        if n == 0
        {
            j += 1;
            if (i == '"') & (t == false)
            {
                t = true;
            }
            else if i == '"'
            {
                p.no_word.push(key);
                return (j, p);
            }
            else if (i == ' ') & (t == false)
            {
                p.no_word.push(key);
                return (j-1, p);
            }
            else
            {
                key.push(i);
            }
        }
        else
        {
            n -= 1;
        }
    }
    if key != ""
    {
        p.no_word.push(key);
    }
    return (j-1, p);
}

fn guillemets(s: &str, x: usize) -> (usize, Parse)
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut n = x;
    let mut key = String::new();
    let mut j = 0;
    let mut t = false;
    for i in s.chars()
    {
        if n == 0
        {
            j += 1;
            if (i == '"') & (t == false)
            {
                t = true;
            }
            else if i == '"'
            {
                p.key_word.push(key);
                break;
            }
            else
            {
                key.push(i);
            }
        }
        else
        {
            n -= 1;
        }
    }
    return (j, p);
}


fn or(s: &str, x: usize) -> (usize, Parse)
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut n = x;
    let mut key1 = String::new();
    let mut key2 = String::new();
    let mut j = 0;
    let mut t = false;
    let mut nb = false;
    let mut b = false;
    for i in s.chars()
    {
        if n == 0
        {
            j += 1;
            if (i == '"') & (t == false)
            {
                t = true;
            }
            else if (i == '"') & (nb == false)
            {
                t = false;
            }
            else if i == '"'
            {
                p.word_or.push(key2.clone());
                b = true;
                break;
            }
            else if i == '/'
            {
                nb = true;
                if t == false
                {
                    p.word_or.push(key1.clone());
                }
            }
            else if (i == ' ') & (t == false)
            {
                p.word_or.push(key2.clone());
                b = true;
                break;
            }
            else if nb == false
            {
                if i != ':'
                {
                    key1.push(i);
                }
            }
            else
            {
                key2.push(i);
            }
        }
        else
        {
            n -= 1;
        }
    }
    if b == false
    {
        p.word_or.push(key2.clone());
    }
    return (j-1, p);
}

fn union(mut p1: Parse, p2: Parse) -> Parse
{
    let mut n = 0;
    for _i in p2.key_word.iter()
    {
        p1.key_word.push(p2.key_word[n].clone());
        n+=1;
    }
    n = 0;
    for _i in p2.no_word.iter()
    {
        p1.no_word.push(p2.no_word[n].clone());
        n+=1;
    }
    n = 0;
    for _i in p2.link.iter()
    {
        p1.link.push(p2.link[n].clone());
        n+=1;
    }
    n = 0;
    for _i in p2.no_link.iter()
    {
        p1.no_link.push(p2.no_link[n].clone());
        n+=1;
    }
    n = 0;
    for _i in p2.word_or.iter()
    {
        p1.word_or.push(p2.word_or[n].clone());
        n+=1;
    }
    return p1;
}


pub fn parser(s : &str) -> Parse
{
    let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
        word_or: vec![],
    };
    let mut j = 0;
    let mut key = String::new();
    let mut x = 0;
    for i in s.chars()
    {
        if j != 0
        {
            j -= 1;
            x += 1;
            continue;
        }
        if i == ':'
        {
            if (key == "site") | (key == "inurl")
            {
                //dbg!(j);
                //dbg!(x);
                let (k, p1) = site(s,x);
                key = String::new();
                j = k;
                let tmp = union(p,p1);
                p = tmp;
                //dbg!(j);
            }
            else if key == "unsite"
            {
                //dbg!(j);
                //dbg!(x);
                let (k, p1) = unsite(s,x);
                key = String::new();
                j = k;
                let tmp = union(p,p1);
                p = tmp;
                //dbg!(j);
            }
            else if (key == "intext") | (key == "allintext")
            {
                key = String::new();
                x += 1;
                continue;
            }
            else if key == "OR"
            {
                //dbg!(j);
                //dbg!(x);
                let (k, p1) = or(s,x);
                key = String::new();
                j = k;
                let tmp = union(p,p1);
                p = tmp;
                //dbg!(j);
            }
            else
            {
                key.push(i);
            }
        }
        else if i == '-'
        {
            let (k, p1) = exclure(s,x);
            key = String::new();
            j = k;
            let tmp = union(p,p1);
            p = tmp;
        }
        else if i == '"'
        {
            let (k, p1) = guillemets(s,x);
            key = String::new();
            j = k;
            let tmp = union(p,p1);
            p = tmp;
        }
        else if i == ' '
        {
            if key != ""
            {
                p.key_word.push(key);
                key = String::new();
            }
        }
        else
        {
            key.push(i);
        }
        x += 1;
    }
    if key != ""
    {
        p.key_word.push(key);
        //key = String::new();
    }
    return p;
    /*let mut p = Parse
    {
        key_word : vec![],
        no_word : vec![],
        link : vec![],
        no_link : vec![],
    };
    let x = s.find("site:");
    if x != None
    {
        let mut n = x.unwrap();
        let mut t = 0;
        let mut key = String::new();
        let mut l = String::new();
        let mut g = false;
        for i in s.chars()
        {
            if n == 0
            {
                
                if i == ':'
                {
                    
                    t += 1;
                }
                else if (i == ' ') & (g == false)
                {
                    
                    t += 1;
                }
                else if i == '"'
                {
                    if g
                    {
                        t += 1;
                    }
                    g = true;
                }
                else if t == 1
                { 
                    l.push(i);
                }
                else if t == 2
                {
                    key.push(i)
                }
                else if t == 3
                {
                    /*dbg!(key);
                    dbg!(l);*/
                    if key != ""
                    {
                        p.key_word.push(key.clone());
                    }
                    p.link.push(l.clone());
                    /*dbg!(key);
                    dbg!(l);*/
                    t += 1;
                    break;
                }
            }
            else
            {
                n -= 1;
            }
        }
        if (t == 3) | (t == 2) 
        {
                    /*dbg!(key);
                    dbg!(l);*/
            if key != ""
            {
                p.key_word.push(key.clone());
            }
            p.link.push(l.clone());
            /*dbg!(key);
            dbg!(l);*/
        }
        /*dbg!(p.key_word);
        dbg!(p.link);*/
    }
    return p;*/
}
