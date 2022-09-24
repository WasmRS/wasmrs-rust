mod manager;

pub(crate) struct Stream {}

// #[cfg(target_family = "wasm")]
mod guest {

    use super::manager::Manager;
    use std::cell::RefCell;

    thread_local! {
      static MANAGER: RefCell<Manager> = RefCell::new(Manager::default());
    }

    fn init_manager(local_capacity: u32, foreign_capacity: u32, frame_size: u32) {
        MANAGER.with(|cell| {
            cell.borrow_mut()
                .init(local_capacity, foreign_capacity, frame_size)
        });
    }

    pub(crate) fn init(
        local_buffer_size: u32,
        foreign_buffer_size: u32,
        foreign_max_frame_size: u32,
    ) {
        init_manager(
            local_buffer_size,
            foreign_buffer_size,
            foreign_max_frame_size,
        );
        let (local_pointer, foreign_pointer) = MANAGER.with(|cell| cell.borrow().get_pointers());
        unsafe {
            super::host::init(local_pointer, foreign_pointer);
        }
    }

    pub(crate) fn send(next_pos: u32) {
        MANAGER.with(|cell| cell.borrow_mut().send(next_pos))
    }
}

mod host {
    #[link(wasm_import_module = "wasmrs")]
    extern "C" {
        #[cfg_attr(
            target_arch = "wasm32",
            link_name = "init: func(guest-buffer-pointer: u32, host-buffer-pointer: u32) -> unit"
        )]
        pub(crate) fn init(local_buffer_pointer: u32, foreign_buffer_pointer: u32);
    }
}
