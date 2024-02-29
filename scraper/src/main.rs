mod processing;

use crate::processing::IO_helper::file_to_str;
use crate::processing::IO_helper::file_to_vec;
use crate::processing::language::get_lang;
use crate::processing::language::is_valid_lang;
use crate::processing::extraction::get_urls;
use crate::processing::extraction::get_paragraphs;

fn main() {
    let path = "processing/iso639-1.txt";
    let tmp = file_to_vec(&path);
    println!("{:?}", tmp);


    let path = "processing/src2.txt";
    let tmp = file_to_str(&path);
    //println!("HTML DOC = \n{}\n\n", tmp);

    println!("\n ////////////////////////////////////////////////////////////////////// \n");
    let lang = get_lang(&tmp);
    println!("LANGUAGE DETECTED = {}\n", lang); 

    let valid_lang = is_valid_lang(&lang);
    dbg!(valid_lang);

    let path = "processing/src2.txt";
    let tmp = file_to_str(&path);
    let urls = get_urls(&tmp);
    println!("URLS FOUND = {:?}\n", urls);


    let tmp = file_to_str(&path);
    let para = get_paragraphs(&tmp);
    //println!("PARAGRAPHS FOUND = {}\n", para);

    /*println!(" -------------------- get_lang tests --------------------");
    let src = "<!DOCTYPE html> <html lang=\"en-US\" prefix=\"og: http://ogp.me/ns#\"> <head> <!-- Google tag (gtag.js) --> <script async src=\"https://www.googletagmanager.com/gtag/js?id=G-KP5YVWZDRL\"></script> <script> ";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\"fr\">";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\"fr-CA\">";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\"en-GB\">";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\"zh-Hans\">";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\" \">";
    let lang = get_lang(&src);
    dbg!(lang);
    let src = "<html lang=\"kw\">";
    let lang = get_lang(&src);
    dbg!(lang);
    

    println!("\n -------------------- get_urls tests --------------------");
    let src = "<script src=\"/template/scripts/bootstrap/popper.js\"></script>
	<script src=\"/template/scripts/bootstrap/bootstrap.bundle.min.js\"></script>
	<script src=\"/template/scripts/bootstrap/bootstrap.js\"></script>
	<link rel=\"stylesheet\" href=\"/template/css/bootstrap/bootstrap.css\" />
	<link rel=\"stylesheet\" href=\"/template/css/bootstrap/bootstrap-social.css\" />
	<link rel=\"stylesheet\" href=\"/template/fonts/fontawesome/css/all.min.css\" />
	<link rel=\"stylesheet\" href=\"/template/fonts/fontawesome/css/fontawesome.min.css\" />
	<script async src=\"https://www.developpez.com/ws/pageview/url/c,cours,bernard-cassagne,node74.php/\"></script>
	<script async=\"async\" src=\"https://www.googletagservices.com/tag/js/gpt.js\"></script>
	<script>";
    let (b, urls) = get_urls(src);
    dbg!(b);
    dbg!(urls);
    

    println!("\n -------------------- valid_urls tests --------------------");
    let src = "<script src=\"/template/scripts/bootstrap/popper.js\"></script>
	<script src=\"/template/scripts/bootstrap/bootstrap.bundle.min.js\"></script>
	<script src=\"/template/scripts/bootstrap/bootstrap.js\"></script>
	<link rel=\"stylesheet\" href=\"/template/css/bootstrap/bootstrap.css\" />
	<link rel=\"stylesheet\" href=\"/template/css/bootstrap/bootstrap-social.css\" />
	<link rel=\"stylesheet\" href=\"/template/fonts/fontawesome/css/all.min.css\" />
	<link rel=\"stylesheet\" href=\"/template/fonts/fontawesome/css/fontawesome.min.css\" />
	<script async src=\"https://www.developpez.com/ws/pageview/url/c,cours,bernard-cassagne,node74.php/\"></script>
	<script async=\"async\" src=\"https://www.googletagservices.com/tag/js/gpt.js\"></script>
	<script>";
    let urls = valid_urls(get_urls(src));
    dbg!(urls);

    let src = "<li><a href=\"https://www.developpez.net/forums/\" title=\"\">Forums</a></li>
		<li><a href=\"https://general.developpez.com/cours/\" title=\"\">Tutoriels</a></li>
		<li><a href=\"https://general.developpez.com/faq/\" title=\"\">FAQ</a></li>
		<li><a href=\"https://www.developpez.net/forums/blogs/\" title=\"\">Blogs</a></li>i";
    let urls = valid_urls(get_urls(src));
    dbg!(urls); 
    

    println!("\n -------------------- get_paragraphs tests --------------------");
    let src = "<p>
  Geckos are a group of usually small, usually nocturnal lizards. They are found on every continent except Antarctica.
</p>
<p>Some species live in houses where they hunt insects attracted by artificial light.</p>";
    let para = get_paragraphs(&src);
    for i in para.lines() { dbg!(i); };*/


}
