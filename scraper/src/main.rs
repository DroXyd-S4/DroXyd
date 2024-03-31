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
    println!("{}", header);
    let path = "processing/src3.html";
    demo(&path, false);



    let header_2 = "  	    ∧,,,∧
                           (- _ -) ☆
|￣￣￣￣￣￣￣￣￣￣￣￣￣￣U U￣￣￣|
|              TF - IDF               |   
￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣￣";
    println!("{}", header_2);

}
