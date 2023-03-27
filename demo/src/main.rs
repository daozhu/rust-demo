

mod model;

use crate::model::test_queue;
use crate::model::test_hot_potato;
use crate::model::test_pal_checker;

fn main() {
    println!("======================= start ===============");
    // test_queue();
    // test_hot_potato();
    test_pal_checker();

    // ... 
    println!("=======================  end  ===============");
}
