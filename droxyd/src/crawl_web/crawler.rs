use crate::bloom_filter::bloom_filter::*;
use crate::crawl_web::get_content::*;
use queues::*;

use crate::bloom_filter::hash_functions::*;
use crate::bloom_filter::is_present::*;

use crate::processing::extraction::*;


use std::process::Command; // Used to sleep

pub fn crawler(limit: u64) -> Vec<String>
{
    let mut i = 0;
    let mut res = vec![];
    let mut filter = init_filter(limit, 0.005_f64);
    // Error rate can change -> [0.01, 1]

    let hash_functions = vec![sha256, md5, double_sha256];
    let mut links: Queue<String> = queue![];
    let _ = links.add(String::from("https://en.wikipedia.org/wiki/42_(number)"));
    let _ = links.add(String::from("https://www.lepoint.fr/eureka/qu-est-ce-qu-une-eclipse-solaire-totale-08-04-2024-2557109_4706.php"));
    let _ = links.add(String::from("https://www.psychologue.net/articles/largent-fait-il-le-bonheur"));
    let _ = links.add(String::from("https://www.lequipe.fr/Football/Coupe-du-monde/"));
    let _ = links.add(String::from("https://stackoverflow.com/questions/tagged/web-crawler"));

    while i < limit && links.size() > 0
    {
        let temp = links.remove().unwrap();
        res.push(temp.clone());

        if is_present(&filter, &hash_functions, &temp) == false
        {
            add_elt(&mut filter, &temp);
            println!("Added: {}", &temp);

            let mut domain = String::new();
            let mut start = false;
            let mut end = false;
            for c in temp.chars()
            {
                if start == false
                {
                    if c == '/'
                    {
                        start = true;
                    }
                }
                else
                {
                    if end == false
                    {
                        if c != '/'
                        {
                            domain.push(c);
                        }
                        else
                        {
                            if domain.len() > 0
                            {
                                end = true;
                            }
                        }
                    }
                }
            }

            //println!("TRUC : {}", &domain);

            // Wait 1 second :
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();


            let content = get_content(temp);
            let urls = get_urls(&content); //Enlever .1 si Justine fix sa fct

            //dbg!(&urls);
            for elt in urls
            {
                //if is_present(&filter, &hash_functions, &temp) == false
                //{
                let mut testdoubleslash1 = true;
                let mut testdoubleslash2 = false;
                let mut resultdoubleslash = false;

                let mut testhttp1 = true;
                let mut testhttp2 = false;
                let mut resulthttp = false;

                let mut treat = true;
                for c in elt.chars()
                {
                    if treat && c == '#'
                    {
                        treat = false;
                    }
                    if testdoubleslash1
                    {
                        if c == '/'
                        {
                            testdoubleslash2 = true;
                        }
                        testdoubleslash1 = false;
                    }
                    else if testdoubleslash2
                    {
                        if c == '/'
                        {
                            resultdoubleslash = true;
                        }
                        testdoubleslash2 = false;
                    }

                    if testhttp1
                    {
                        if c == 'h'
                        {
                            testhttp2 = true;
                        }
                        testhttp1 = false;
                    }
                    else if testhttp2
                    {
                        if c == 't'
                        {
                            resulthttp = true;
                        }
                        testhttp2 = false;
                    }
                }

                if resulthttp // || resultdoubleslash
                {
                    let _ = links.add(elt);
                }
                else if resultdoubleslash
                {
                    let mut req = String::from("https://");

                    let mut elt1 = true;
                    let mut elt2 = true;
                    for c in elt.chars()
                    {
                        if elt1
                        {
                            elt1 = false;
                        }
                        else if elt2
                        {
                            elt2 = false;
                        }
                        else
                        {
                            req.push(c);
                        }
                    }
                    
                    let _ = links.add(req);
                }
                else
                {
                    if treat
                    {
                        let mut req = String::from("https://");
                        req.push_str(&domain);
                        req.push_str(&elt);
                        let _ = links.add(req);
                    }
                }
                //}
            }
        }
        else
        {
            println!("DejaV: {}", &temp);
        }

        i += 1;
    }
    return res;
}
