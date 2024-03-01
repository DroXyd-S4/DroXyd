use crate::processing::extraction::*;
use crate::processing::IO_helper::*;
use crate::processing::language::*;

pub fn demo(src: &str, show_website: bool) {
    if show_website { println!("Le code soure du site web est: \n{}\n", src); }
    else {
        println!("éléments détectés dans {}: \n\n", src);

        let file = file_to_str(&src);

        let lang = get_lang(&file);
        match is_valid_lang(&lang) {
            true => println!("La langue détectée est: {}\n", get_lang(&file)),
            _ => println!("Aucune langue valide n'a été détectée"),
        }
        let file =  file_to_str(&src);
        let urls = get_urls(&file);
        println!("Les URLs référencées dans {} sont: \n {:?}\n", src, urls.1);

        let file = file_to_str(&src);
        let para = tag_extraction(&file);
        println!("Les textes trouvés sont: \n{}\n", para);
    }

}
