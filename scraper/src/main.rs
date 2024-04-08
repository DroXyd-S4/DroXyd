mod processing;

use crate::processing::demo::demo;

fn main() {
    println!("\n ////////////////////////////////////////////////////////////////////// \n");

    let header =  "
  ∧,,,∧
 (• v •) ☆
|￣U U￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣|
|          DATA PROCESSING            |   
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";

    println!("{}", std::env::current_dir().unwrap().display());
    println!("{}", header);
    let url = "https://www.mathsisfun.com/algebra/vectors-cross-product.html";
    demo(&url, false);



    let header_2 = "
                            ∧,,,∧
                           (- _ -) ☆
|￣￣￣￣￣￣￣￣￣￣￣￣￣￣U U￣￣￣|
|              TF - IDF               |   
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header_2);
}



