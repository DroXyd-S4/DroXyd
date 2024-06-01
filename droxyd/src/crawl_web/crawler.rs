use crate::bloom_filter::bloom_filter::*;
use crate::crawl_web::get_content::*;
use queues::*;

use crate::bloom_filter::hash_functions::*;
use crate::bloom_filter::is_present::*;

use crate::processing::extraction::*;

use std::process::Command; // Used to sleep

use rand::prelude::*; // Useful to choose sites

use std::sync::mpsc;
use std::thread;
use std::time::Duration; // Ca va multi threader ou quoicoubeh

extern crate num_cpus;



pub fn crawler(limit: u64) -> Vec<String>
{
    let mut res = vec![];

    res.push(String::from("https://web.archive.org/web/20170131181820/http://www.dmoz.org"));
    res.push(String::from("https://en.wikipedia.org/wiki/42_(number)"));
    res.push(String::from("https://www.lepoint.fr/eureka/qu-est-ce-qu-une-eclipse-solaire-totale-08-04-2024-2557109_4706.php"));
    res.push(String::from("https://www.psychologue.net/articles/largent-fait-il-le-bonheur"));
    res.push(String::from("https://www.lequipe.fr/Football/Coupe-du-monde/"));
    res.push(String::from("https://it.wikipedia.org/wiki/Pizza"));
    res.push(String::from("https://olympics.com"));
    res.push(String::from("https://www.nytimes.com"));
    res.push(String::from("https://de.wikipedia.org/wiki/Attack_on_Titan"));
    res.push(String::from("https://es.wikipedia.org/wiki/Pulp_Fiction"));

    let mut tasks = vec![];
    for k in 0..num_cpus::get()
    {
        let x = vec![];
        tasks.push(x);
    }
    let mut j = 0;
    while j < res.len()
    {
        for k in 0..num_cpus::get()
        {
            if j < res.len()
            {
                let mut b = String::new();
                for c in res[j].chars()
                {
                    b.push(c);
                }
                tasks[k].push(b);
            }
            j += 1;
        }
    }


    let mut filter = init_filter(limit, 0.0001_f64);
    // Error rate can change -> [0.01, 1]

    let hash_functions = vec![sha256, md5, double_sha256];

    /*
       let mut links: Queue<String> = queue![];
       let _ = links.add(String::from("https://web.archive.org/web/20170131181820/http://www.dmoz.org"));
       let _ = links.add(String::from("https://en.wikipedia.org/wiki/42_(number)"));
       let _ = links.add(String::from("https://www.lepoint.fr/eureka/qu-est-ce-qu-une-eclipse-solaire-totale-08-04-2024-2557109_4706.php"));
       let _ = links.add(String::from("https://www.psychologue.net/articles/largent-fait-il-le-bonheur"));
       let _ = links.add(String::from("https://www.lequipe.fr/Football/Coupe-du-monde/"));
       let _ = links.add(String::from("https://it.wikipedia.org/wiki/Pizza"));
       let _ = links.add(String::from("https://olympics.com"));
       let _ = links.add(String::from("https://www.nytimes.com"));
       let _ = links.add(String::from("https://de.wikipedia.org/wiki/Attack_on_Titan"));
       let _ = links.add(String::from("https://es.wikipedia.org/wiki/Pulp_Fiction"));
       */


    println!("Booting...");
    // First giveaway

    let (tx, rx) = mpsc::channel();
    let mut filter_temp = filter.clone();

    add_elt(&mut filter, &String::from("https://web.archive.org/web/20170131181820/http://www.dmoz.org"));      
    add_elt(&mut filter, &String::from("https://en.wikipedia.org/wiki/42_(number)"));
    add_elt(&mut filter, &String::from("https://www.lepoint.fr/eureka/qu-est-ce-qu-une-eclipse-solaire-totale-08-04-2024-2557109_4706.php"));
    add_elt(&mut filter, &String::from("https://www.psychologue.net/articles/largent-fait-il-le-bonheur"));
    add_elt(&mut filter, &String::from("https://www.lequipe.fr/Football/Coupe-du-monde/"));
    add_elt(&mut filter, &String::from("https://it.wikipedia.org/wiki/Pizza"));
    add_elt(&mut filter, &String::from("https://olympics.com"));
    add_elt(&mut filter, &String::from("https://www.nytimes.com"));
    add_elt(&mut filter, &String::from("https://de.wikipedia.org/wiki/Attack_on_Titan"));
    add_elt(&mut filter, &String::from("https://es.wikipedia.org/wiki/Pulp_Fiction"));

    let mut thr = vec![];

    for k in 0..num_cpus::get()
    {
        let tx0 = tx.clone();
        let mut links = tasks[k].clone();

        let multi = thread::spawn(move || {
            for i in 0..links.len()
            {
                let temp = links.pop().unwrap();

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
                let content = get_content(temp);
                let urls = get_urls(&content);

                for elt in urls
                {
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
                        tx0.send(elt).unwrap();
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

                        tx0.send(req).unwrap();
                    }
                    else
                    {
                        if treat
                        {
                            let mut req = String::from("https://");
                            req.push_str(&domain);
                            req.push_str(&elt);
                            tx0.send(req).unwrap();
                        }

                    }
                }
            }

        });
        thr.push(multi);

    }

    for t in thr
    {
        t.join().unwrap();
    }

    let mut i = 0;
    let mut tasks = vec![];
    for k in 0..num_cpus::get()
    {
        let x = vec![];
        tasks.push(x);
    }
    for messages in rx
    {
        tasks[i % num_cpus::get()].push(messages);
        i += 1;
        if i >= 4500
        {
            break;
        }
    }

    i = 0;

    let mut rng = rand::thread_rng();
    println!();
    println!("Starting process...");

    let (tx, rx) = mpsc::channel();
    let mut x = 0;
    while (i as u64) < limit
    {
        let mut thr = vec![];
        if x == 0
        {
            for k in 0..((limit as usize) % num_cpus::get())
            {
                let tx0 = tx.clone();
                let index = rng.gen_range(0..tasks[k].len());
                let mut link = String::new();
                for c in tasks[k][index].chars()
                {
                    link.push(c);
                }
                res.push(link.clone());
                tasks[k].remove(index);

                let presence = is_present(&filter, &hash_functions, &link);
                if presence == false
                {
                    add_elt(&mut filter, &link);
                }

                let resu = res.clone();
                let multi = thread::spawn(move || {
                    if presence == false
                    {
                        println!("Added: {}", &link);

                        let mut domain = String::new();
                        let mut start = false;
                        let mut end = false;
                        for c in link.chars()
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

                        let content = get_content(link);
                        let urls = get_urls(&content); //Enlever .1 si Justine fix sa fct

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
                                match tx0.send(elt)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
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

                                match tx0.send(req)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                            }
                            else
                            {
                                if treat
                                {
                                    let mut req = String::from("https://");
                                    req.push_str(&domain);
                                    req.push_str(&elt);


                                    match tx0.send(req)
                                    {
                                        Ok(x) => (),
                                        _ => {
                                            println!("Problem while sending data to thread");
                                        },
                                    };

                                }
                            }
                            //}
                        }
                    }
                    else
                    {









                        let mut found = false;
                        for elt in resu
                        {
                            if elt.eq(&link)
                            {
                                println!("Found: {}", &link);
                                found = true;
                                break;
                            }
                        }
                        if found == false
                        {
                            println!("Add+C: {}", &link);

                            let mut domain = String::new();
                            let mut start = false;
                            let mut end = false;
                            for c in link.chars()
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

                            let content = get_content(link);
                            let urls = get_urls(&content); //Enlever .1 si Justine fix sa fct

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



                                    match tx0.send(elt)
                                    {
                                        Ok(x) => (),
                                        _ => {
                                            println!("Problem while sending data to thread");
                                        },
                                    };

                                    //tx0.send(elt).unwrap();
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



                                    match tx0.send(req)
                                    {
                                        Ok(x) => (),
                                        _ => {
                                            println!("Problem while sending data to thread");
                                        },
                                    };
                                    //tx0.send(req).unwrap();
                                }
                                else
                                {
                                    if treat
                                    {
                                        let mut req = String::from("https://");
                                        req.push_str(&domain);
                                        req.push_str(&elt);


                                        match tx0.send(req)
                                        {
                                            Ok(x) => (),
                                            _ => {
                                                println!("Problem while sending data to thread");
                                            },
                                        };

                                        //tx0.send(req).unwrap();
                                    }
                                }
                                //}
                            }
                        }





                    }
                });

                thr.push(multi);
            }

            x = 1;
            i += (limit as usize) % num_cpus::get();
        }
        else
        {
            for k in 0..num_cpus::get()
            {
                let tx0 = tx.clone();
                let index = rng.gen_range(0..tasks[k].len());
                let mut link = String::new();
                for c in tasks[k][index].chars()
                {
                    link.push(c);
                }
                res.push(link.clone());
                tasks[k].remove(index);

                let presence = is_present(&filter, &hash_functions, &link);
                if presence == false
                {
                    add_elt(&mut filter, &link);
                }

                let resu = res.clone();
                let multi = thread::spawn(move || {
                    if presence == false
                    {
                        println!("Added: {}", &link);

                        let mut domain = String::new();
                        let mut start = false;
                        let mut end = false;
                        for c in link.chars()
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

                        let content = get_content(link);
                        let urls = get_urls(&content); //Enlever .1 si Justine fix sa fct

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

                                match tx0.send(elt)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                //tx0.send(elt).unwrap();
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


                                match tx0.send(req)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                //tx0.send(req).unwrap();
                            }
                            else
                            {
                                if treat
                                {
                                    let mut req = String::from("https://");
                                    req.push_str(&domain);
                                    req.push_str(&elt);
                                    

                                match tx0.send(req)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                    //tx0.send(req).unwrap();
                                }
                            }
                            //}
                        }
                    }
                    else
                    {


                        let mut found = false;
                        for elt in resu
                        {
                            if elt.eq(&link)
                            {
                                println!("Found: {}", &link);
                                found = true;
                                break;
                            }
                        }
                        if found == false
                        {
                            println!("Add+C: {}", &link);

                            let mut domain = String::new();
                            let mut start = false;
                            let mut end = false;
                            for c in link.chars()
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

                            let content = get_content(link);
                            let urls = get_urls(&content); //Enlever .1 si Justine fix sa fct

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

                                match tx0.send(elt)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                    //tx0.send(elt).unwrap();
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


                                match tx0.send(req)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                    //tx0.send(req).unwrap();
                                }
                                else
                                {
                                    if treat
                                    {
                                        let mut req = String::from("https://");
                                        req.push_str(&domain);
                                        req.push_str(&elt);
                                       

                                match tx0.send(req)
                                {
                                    Ok(x) => (),
                                    _ => {
                                        println!("Problem while sending data to thread");
                                    },
                                };
                                        // tx0.send(req).unwrap();
                                    }
                                }
                                //}
                            }
                        }



                    }
                    thread::sleep(Duration::from_millis(1));
                });

                i += num_cpus::get();
                thr.push(multi);
            }
        }

        for t in thr
        {
            t.join().unwrap();
        }
    }
    return res;
}
