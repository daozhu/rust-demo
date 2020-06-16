
mod models;

use models::get_user_info;
use models::set_user_info;


// preclude 模式 TODO


fn main() {
    println!("Hello, world!");
    println!("u -> : {}", get_user_info(22));
    println!("u -> : {}", set_user_info(22));
}
