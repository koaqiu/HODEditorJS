pub mod collision;
pub mod compiler;
pub mod dae;
pub mod hod;
pub mod serialize;
pub mod parse;
pub mod iff;
pub mod kdop;
pub mod math;
pub mod shader_map;
pub mod types;
pub mod texture_codec;
pub mod animation;
pub mod model_methods;
pub mod engine_nozzles;
pub mod xpress;

use std::sync::atomic::{AtomicBool, Ordering};

/// 解析器的诊断输出开关。默认开启，保持旧行为。
static VERBOSE: AtomicBool = AtomicBool::new(true);

/// 开/关诊断输出。库使用者（CLI、GUI、WASM）可以关掉，
/// 避免它们的 stdout 被解析器的调试信息淹没。
pub fn set_verbose(on: bool) {
    VERBOSE.store(on, Ordering::Relaxed);
}

/// 当前是否输出诊断信息。
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// 诊断输出：行为和 `println!` 一样，但受 [`set_verbose`] 控制。
#[macro_export]
macro_rules! pdbg {
    ($($arg:tt)*) => {
        if $crate::is_verbose() {
            ::std::println!($($arg)*);
        }
    };
}
