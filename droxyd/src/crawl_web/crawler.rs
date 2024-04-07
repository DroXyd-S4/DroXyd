use crate::bloom_filter::bloom_filter::*;
use crate::crawl_web::get_content::*;
use queues::*;

use crate::bloom_filter::hash_functions::*;
use crate::bloom_filter::is_present::*;

use std::process::Command; // Used to sleep

pub fn crawler(limit: u64, start: String) -> Vec<String>
{
    let mut i = 0;
    let mut res = vec![];
    let mut filter = init_filter(limit, 0.1_f64);
    // Error rate can change -> [0.01, 1]

    let hash_functions = vec![sha256, md5, double_sha256];
    let mut links: Queue<String> = queue![];
    let _ = links.add(start);
    while i < limit && links.size() > 0
    {
        let temp = links.remove().unwrap();
        res.push(temp.clone());

        if is_present(&filter, &hash_functions, &temp) == false
        {
            add_elt(&mut filter, &temp);
            println!("Added: {}", &temp);


            // Just wait
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();

            let content = get_content(temp);



            // Traitement du scraper, on recupere les href dans [content]

            // Enqueue les autres liens


        }
        else
        {
            println!("DejaV: {}", &temp);
        }

        i += 1;
    }
    return res;
}
