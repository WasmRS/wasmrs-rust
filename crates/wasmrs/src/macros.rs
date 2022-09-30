#[macro_export]
macro_rules! flux_try {
    ($expr:expr) => {{
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let flux = Flux::new();
                let _ = flux.error(PayloadError::application_error(e.to_string()));
                return flux.split_receiver().unwrap();
            }
        }
    }};
}
