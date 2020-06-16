

// 子模块可以访问父级模块，通过super::
// 但是同级模块并不能简单的通过命名空间: user_model::*
// 必须通过父级模块访问，跟其他人一样
// 不同的是，可以直接访问父模块的私有属性

pub fn get_user_orders(x: i32) -> i32 {
    //super::user_model::get_user_info(x);

    // 或者简单的导入
    //use super::user_model::*;
    // 然后直接使用
    // get_user_orders(x);

    // 也可以用 {} 限制作用域
    let res = {
        use super::user_model::*; 
        get_user_orders(x)
    };
    // 此时不能再使用get_user_orders(x);

    x
}
