#[macro_export]
macro_rules! flux_try {
  ($expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => {
        let flux = Flux::new();
        let _ = flux.error(PayloadError::application_error(e.to_string()));
        return flux.take_rx().unwrap();
      }
    }
  }};
  ($tx:ident, $expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => {
        let _ = $tx.error(PayloadError::application_error(e.to_string()));
        return;
      }
    }
  }};
}

#[macro_export]
macro_rules! mono_try {
  ($expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => return Mono::new_error(PayloadError::application_error(e.to_string())),
    }
  }};
}

#[macro_export]
macro_rules! clock {
  ($expr:expr, $msg:literal) => {{
    let start = std::time::Instant::now();
    $expr;
    let end = std::time::Instant::now();
    println!("{} took {}ns", $msg, (end - start).as_nanos());
  }};
}
