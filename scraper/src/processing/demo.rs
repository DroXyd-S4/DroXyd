use crate::processing::extraction::*;
use crate::processing::IO_helper::*;
use crate::processing::language::*;

pub fn demo(URL: &str, show_website: bool) {
    let body = URL_to_String(URL).unwrap(); 
    if show_website { println!("Le code soure du site web est: \n{}\n", body ); }

    else {
        println!("éléments détectés dans {}: \n\n", URL);

        let lang = get_lang(&body);
        match is_valid_lang(&lang) {
            true => println!("La langue détectée est: {}\n", get_lang(&body)),
            _ => println!("Aucune langue valide n'a été détectée"),
        }
        let urls = get_urls(&body);
        println!("Les URLs référencées dans {} sont: \n {:?}\n", URL, urls.1);
        let para = tag_extraction(&body);
        let (sent, nb) = get_sentences(&para);
        println!("Les {} phrases trouvées sont: \n", nb);
        for i in sent.iter() {println!("{}\n", i); }
            
    }

}
