use crate::processing::extraction::*;
use crate::processing::IO_helper::*;
use crate::processing::language::*;
use crate::processing::cleansing::*;

pub fn demo(url: &str, show_tf_idf: bool) {
    let body = URL_to_String(url).unwrap(); 
    println!("éléments détectés dans {}: \n\n", url);

    let header =  "
  ∧,,,∧
 (• v •) ☆
|￣U U￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣|
|          DATA PROCESSING            |
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header);

    let lang = get_lang(&body);
    match is_valid_lang(&lang) {
        true => println!("La langue détectée est: {}\n", get_lang(&body)),
        _ => println!("Aucune langue valide n'a été détectée"),
    }
    let urls = get_urls(&body);
    println!("Les URLs référencées dans {} sont: \n {:?}\n", url, urls.1);
    let para = tag_extraction(&body);
    let (sent, nb) = get_sentences(&para);
    println!("Les {} phrases trouvées sont: \n", nb);
    for i in sent.iter() {println!("{}\n", i); }


    let header_2 = "
                            ∧,,,∧
                           (- _ -) ☆
|￣￣￣￣￣￣￣￣￣￣￣￣￣￣U U￣￣￣|
|              TF - IDF               |   
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header_2);

    let tmp = TF_IDF(para);
    if show_tf_idf {
        for (curr_w, tfidf) in &tmp {
            println!("{curr_w}: \"{tfidf}\"");
        }
    }
    let keywords = keywords(tmp);
    println!("{:?}", keywords);

}
