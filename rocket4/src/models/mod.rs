
// 私有的模块 (也就是model模块下的子模块)
mod user_model;

// 重新导出模块包含的函数
// 有选择性的对外公开实现细节
pub use user_model::get_user_info;
pub use user_model::set_user_info;

// 这样做，隐藏了这个模块的组成及其实现细节
// 外界无法直接使用
//

// mod由于算是同级目录下其他模块的父级
// 所以，子模块可以引用父级的属性: super::  即便是私有的
const DEBUG:bool = false;

