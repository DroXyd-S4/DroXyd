mod processing;

use crate::processing::demo::demo;


fn main() {
    println!("\n ////////////////////////////////////////////////////////////////////// \n");
    let path = "processing/src_demo.html";
    demo(&path, false);

}
