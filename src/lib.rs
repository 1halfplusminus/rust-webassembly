#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
}

fn main() {
    use std::mem::ManuallyDrop;

    use hal::{
        device::Device,
        window::{Extent2D, PresentationSurface, Surface},
        Instance,
    };

    const APP_NAME: &'static str = "Part 1: Drawing a triangle";
    const WINDOW_SIZE: [u32; 2] = [512, 512];

    let event_loop = winit::event_loop::EventLoop::new();

    let (logical_window_size, physical_window_size) = {
        use winit::dpi::{LogicalSize, PhysicalSize};
        let dpi = event_loop
            .primary_monitor()
            .expect("No primary monitor")
            .scale_factor();
        let logical: LogicalSize<u32> = WINDOW_SIZE.into();
        let physical: PhysicalSize<u32> = logical.to_physical(dpi);
        (logical, physical)
    };

    let mut surface_extend = Extent2D {
        width: physical_window_size.width,
        height: physical_window_size.height,
    };

    let window = winit::window::WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size)
        .build(&event_loop)
        .expect("Failed to create window");
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, Window};
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&winit::platform::web::WindowExtWebSys::canvas(&window))
            .unwrap();
    }
}
