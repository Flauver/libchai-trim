//! 目标函数接口，以及默认目标函数的实现
//!
//!

use crate::data::{元素映射, 编码信息};
use circular_buffer::CircularBuffer;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::fmt::Display;
pub mod cache;
pub mod default;
pub mod metric;

pub trait 目标函数 {
    type 目标值: Display + Clone + Serialize;
    fn 计算(
        &mut self, 编码结果: &mut [编码信息], 映射: &元素映射, 进度: f64
    ) -> (Self::目标值, f64, FxHashMap<usize, f64>, FxHashMap<usize, FxHashMap<usize, CircularBuffer<2, f64>>>);
}
