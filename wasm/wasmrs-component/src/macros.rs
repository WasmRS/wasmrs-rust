#[macro_export]
macro_rules! println {
  () => {
    #[cfg(all(debug_assertions,target = "wasm32-wasi"))]
    std::io::_print::print!("\n")
  };
  ($($arg:tt)*) => {{
    #[cfg(all(debug_assertions,target = "wasm32-wasi"))]
    std::io::_print(std::format_args_nl!($($arg)*));
  }};
}
