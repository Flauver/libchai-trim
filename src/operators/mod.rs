//! 算子接口，以及默认操作的实现（包含变异算子）
//!

use rustc_hash::FxHashMap;

use crate::data::{元素, 元素映射};

pub mod default;

pub trait 变异 {
    /// 基于现有的一个解通过随机扰动创建一个新的解，返回变异的元素
    fn 变异(&mut self, 映射: &mut 元素映射, 概率: &FxHashMap<usize, f64>, 冲突: &FxHashMap<usize, FxHashMap<usize, f64>>, 进度: f64) -> Vec<元素>;
}

pub trait 杂交 {
    /// 基于现有的一个解通过随机扰动创建一个新的解
    fn 杂交(&mut self, 映射一: &元素映射, 映射二: &元素映射) -> 元素映射;
}
