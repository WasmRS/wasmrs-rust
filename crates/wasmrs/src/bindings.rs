#[allow(clippy::all)]
mod wasmrs {
  pub fn init_buffers(guest_buffer_pointer: u32,host_buffer_pointer: u32,) -> (){
    unsafe {
      #[link(wasm_import_module = "wasmrs")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "init-buffers: func(guest-buffer-pointer: u32, host-buffer-pointer: u32) -> unit")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasmrs_init-buffers: func(guest-buffer-pointer: u32, host-buffer-pointer: u32) -> unit")]
        fn wit_import(_: i32, _: i32, );
      }
      wit_import(wit_bindgen_guest_rust::rt::as_i32(guest_buffer_pointer), wit_bindgen_guest_rust::rt::as_i32(host_buffer_pointer));
      ()
    }
  }
  pub fn send(next_pos: u32,) -> (){
    unsafe {
      #[link(wasm_import_module = "wasmrs")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "send: func(next-pos: u32) -> unit")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "wasmrs_send: func(next-pos: u32) -> unit")]
        fn wit_import(_: i32, );
      }
      wit_import(wit_bindgen_guest_rust::rt::as_i32(next_pos));
      ()
    }
  }
}
#[allow(clippy::all)]
mod wasmrs {
  #[export_name = "init-buffers: func(guest-buffer-pointer: u32, host-buffer-pointer: u32) -> unit"]
  unsafe extern "C" fn __wit_bindgen_wasmrs_init_buffers(arg0: i32, arg1: i32, ){
    let result = <super::Wasmrs as Wasmrs>::init_buffers(arg0 as u32, arg1 as u32);
    let () = result;
  }
  #[export_name = "send: func(next-pos: u32) -> unit"]
  unsafe extern "C" fn __wit_bindgen_wasmrs_send(arg0: i32, ){
    let result = <super::Wasmrs as Wasmrs>::send(arg0 as u32);
    let () = result;
  }
  pub trait Wasmrs {
    fn init_buffers(guest_buffer_pointer: u32,host_buffer_pointer: u32,) -> ();
    fn send(next_pos: u32,) -> ();
  }
}
