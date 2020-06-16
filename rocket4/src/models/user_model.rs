

// 对于user_model. 同级的mod.算其父级

pub fn get_user_info(x: i32) -> i32 {
    if super::DEBUG {
        println!("user mod debug true")
    }

    x
}

pub fn set_user_info(x: i32) -> i32 {
    if super::DEBUG {
        println!("user mod debug true")
    }

    x
}
