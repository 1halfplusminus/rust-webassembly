#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main();
}
#[cfg(target_arch = "android")]
use ndk_glue;
use std::iter;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Debug).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    use std::mem::ManuallyDrop;

    use gfx_hal::{
        adapter::{Adapter, MemoryType},
        buffer, command,
        format::{self as f, AsFormat},
        image as i, memory as m, pass, pool,
        prelude::*,
        pso,
        queue::QueueGroup,
        window, Backend,
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

    let mut surface_extend = window::Extent2D {
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
        web_sys::console::log_1(&"Hello using web-sys".into());
    }

    let (instance, surface, adapter) = {
        let instance = backend::Instance::create(APP_NAME, 1).expect("Backend not supported");

        let surface = unsafe {
            instance
                .create_surface(&window)
                .expect("Failed to create surface for window")
        };

        let adapter = instance.enumerate_adapters().remove(0);
        for adapter in &instance.enumerate_adapters() {
            println!("{:?}", adapter.info);
        }
        (instance, surface, adapter)
    };

    let (device, mut queue_group) = {
        use gfx_hal::queue::QueueFamily;

        let queue_family = adapter
            .queue_families
            .iter()
            .find(|family| {
                surface.supports_queue_family(family) && family.queue_type().supports_graphics()
            })
            .expect("No compatible queue family found");

        let mut gpu = unsafe {
            use gfx_hal::adapter::PhysicalDevice;

            adapter
                .physical_device
                .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
                .expect("Failed to open device")
        };
        (gpu.device, gpu.queue_groups.pop().unwrap())
    };

    let (command_pool, mut command_buffer) = unsafe {
        use gfx_hal::command::Level;
        use gfx_hal::pool::CommandPoolCreateFlags;

        let mut command_pool = device
            .create_command_pool(queue_group.family, CommandPoolCreateFlags::empty())
            .expect("Out of memory");

        let command_buffer = command_pool.allocate_one(Level::Primary);

        (command_pool, command_buffer)
    };

    // Render passes
    let surface_color_format = {
        use gfx_hal::format::{ChannelType, Format};

        let supported_formats = surface
            .supported_formats(&adapter.physical_device)
            .unwrap_or(vec![]);

        let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgba8Srgb);

        supported_formats
            .into_iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .unwrap_or(default_format)
    };
    let render_pass = {
        use gfx_hal::image::Layout;
        use gfx_hal::pass::{
            Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc,
        };

        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        unsafe {
            device
                .create_render_pass(
                    iter::once(color_attachment),
                    iter::once(subpass),
                    iter::empty(),
                )
                .expect("Out of memory")
        }
    };

    let pipeline_layout = unsafe {
        device
            .create_pipeline_layout(iter::empty(), iter::empty())
            .expect("Out of memory")
    };
    let vertex_shader = include_bytes!("shaders/part-1.vert.spv");
    let fragment_shader = include_bytes!("shaders/part-1.frag.spv");
    //Create a pipeline with the given layout and shadaers
    unsafe fn make_pipeline<B: gfx_hal::Backend>(
        device: &B::Device,
        render_pass: &B::RenderPass,
        pipeline_layout: &B::PipelineLayout,
        vertex_shader: &[u8],
        fragment_shader: &[u8],
    ) -> B::GraphicsPipeline {
        use gfx_hal::pass::Subpass;
        use gfx_hal::pso::{
            BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
            InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Specialization,
        };
        let spirv: Vec<u32> = auxil::read_spirv(std::io::Cursor::new(vertex_shader)).unwrap();
        let vertex_shader_module = device
            .create_shader_module(&spirv)
            .expect("Failed to create vertex shader module");
        let spirv: Vec<u32> = auxil::read_spirv(std::io::Cursor::new(fragment_shader)).unwrap();
        let fragment_shader_module = device
            .create_shader_module(&spirv)
            .expect("Failed to create fragment shader module");

        let (vs_entry, fs_entry) = (
            EntryPoint::<B> {
                entry: "main",
                module: &vertex_shader_module,
                specialization: Specialization::default(),
            },
            EntryPoint::<B> {
                entry: "main",
                module: &fragment_shader_module,
                specialization: Specialization::default(),
            },
        );

        let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
            buffers: &[],
            attributes: &[],
            input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
            vertex: vs_entry,
            tessellation: None,
            geometry: None,
        };

        let mut pipeline_desc = GraphicsPipelineDesc::new(
            primitive_assembler,
            Rasterizer {
                cull_face: Face::BACK,
                ..Rasterizer::FILL
            },
            Some(fs_entry),
            pipeline_layout,
            Subpass {
                index: 0,
                main_pass: render_pass,
            },
        );

        pipeline_desc.blender.targets.push(ColorBlendDesc {
            mask: ColorMask::ALL,
            blend: Some(BlendState::ALPHA),
        });

        let pipeline = device
            .create_graphics_pipeline(&pipeline_desc, None)
            .expect("Failed to create graphics pipeline");

        device.destroy_shader_module(vertex_shader_module);
        device.destroy_shader_module(fragment_shader_module);
        pipeline
    }
    let pipeline = unsafe {
        make_pipeline::<backend::Backend>(
            &device,
            &render_pass,
            &pipeline_layout,
            vertex_shader,
            fragment_shader,
        )
    };

    struct Resources<B: gfx_hal::Backend> {
        instance: B::Instance,
        surface: B::Surface,
        device: B::Device,
        render_passes: Vec<B::RenderPass>,
        pipeline_layouts: Vec<B::PipelineLayout>,
        pipelines: Vec<B::GraphicsPipeline>,
        command_pool: B::CommandPool,
        submission_complete_fence: B::Fence,
        rendering_complete_semaphore: B::Semaphore,
    }

    struct ResourceHolder<B: gfx_hal::Backend>(ManuallyDrop<Resources<B>>);

    impl<B: gfx_hal::Backend> Drop for ResourceHolder<B> {
        fn drop(&mut self) {
            unsafe {
                let Resources {
                    instance,
                    mut surface,
                    device,
                    command_pool,
                    render_passes,
                    pipeline_layouts,
                    pipelines,
                    submission_complete_fence,
                    rendering_complete_semaphore,
                } = ManuallyDrop::take(&mut self.0);

                device.destroy_semaphore(rendering_complete_semaphore);
                device.destroy_fence(submission_complete_fence);
                for pipeline in pipelines {
                    device.destroy_graphics_pipeline(pipeline);
                }
                for pipeline_layout in pipeline_layouts {
                    device.destroy_pipeline_layout(pipeline_layout);
                }

                for render_pass in render_passes {
                    device.destroy_render_pass(render_pass);
                }

                device.destroy_command_pool(command_pool);
                surface.unconfigure_swapchain(&device);
                instance.destroy_surface(surface);
            }
        }
    }

    let submission_complete_fence = device.create_fence(true).expect("Out of memory");
    let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");
    let mut _should_configure_swapchain = true;
    let mut resource_holder: ResourceHolder<backend::Backend> =
        ResourceHolder(ManuallyDrop::new(Resources {
            instance,
            surface,
            device,
            command_pool,
            render_passes: vec![render_pass],
            pipeline_layouts: vec![pipeline_layout],
            pipelines: vec![pipeline],
            submission_complete_fence,
            rendering_complete_semaphore,
        }));
    event_loop.run(move |event, _, control_flow| {
        use winit::event::{Event, WindowEvent};
        use winit::event_loop::ControlFlow;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(dims) => {
                    surface_extend = window::Extent2D {
                        width: dims.width,
                        height: dims.height,
                    };
                    _should_configure_swapchain = true;
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    surface_extend = window::Extent2D {
                        height: new_inner_size.height,
                        width: new_inner_size.width,
                    };
                    _should_configure_swapchain = true;
                }
                _ => (),
            },
            Event::MainEventsCleared => window.request_redraw(),
            winit::event::Event::RedrawEventsCleared => {
                let res: &mut Resources<_> = &mut resource_holder.0;
                let render_pass = &res.render_passes[0];
                let pipeline = &res.pipelines[0];

                unsafe {
                    let render_timeout_ns = 1_000_000_000;
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        res.device
                            .wait_for_fence(&res.submission_complete_fence, render_timeout_ns)
                            .expect("Out of memory or device lost");
                    }
                    res.device
                        .reset_fence(&mut res.submission_complete_fence)
                        .expect("Out of memory");

                    res.command_pool.reset(false);
                }

                if _should_configure_swapchain {
                    use gfx_hal::window::SwapchainConfig;

                    let caps = res.surface.capabilities(&adapter.physical_device);

                    let mut swapchain_config =
                        SwapchainConfig::from_caps(&caps, surface_color_format, surface_extend);

                    if caps.image_count.contains(&3) {
                        swapchain_config.image_count = 3;
                    }

                    surface_extend = swapchain_config.extent;

                    unsafe {
                        res.surface
                            .configure_swapchain(&res.device, swapchain_config)
                            .expect("Failed re configure swapchain");
                    }

                    _should_configure_swapchain = false;
                }

                let surface_image = unsafe {
                    let acquire_timeout_ns = 1_000_000_000;

                    match res.surface.acquire_image(acquire_timeout_ns) {
                        Ok((image, _)) => image,
                        Err(_) => {
                            _should_configure_swapchain = true;
                            return;
                        }
                    }
                };

                let framebuffer = unsafe {
                    use gfx_hal::image::Extent;
                    use gfx_hal::window::SwapchainConfig;
                    let caps = res.surface.capabilities(&adapter.physical_device);
                    let swapchain_config =
                        SwapchainConfig::from_caps(&caps, surface_color_format, surface_extend);
                    res.device
                        .create_framebuffer(
                            render_pass,
                            iter::once(swapchain_config.framebuffer_attachment()),
                            Extent {
                                width: surface_extend.width,
                                height: surface_extend.height,
                                depth: 1,
                            },
                        )
                        .unwrap()
                };

                let viewport = {
                    use gfx_hal::pso::{Rect, Viewport};

                    Viewport {
                        rect: Rect {
                            x: 0,
                            y: 0,
                            w: surface_extend.width as i16,
                            h: surface_extend.height as i16,
                        },
                        depth: 0.0..1.0,
                    }
                };

                unsafe {
                    use gfx_hal::command::{
                        ClearColor, ClearValue, CommandBufferFlags, SubpassContents,
                    };
                    command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

                    command_buffer.set_viewports(0, iter::once(viewport.clone()));

                    command_buffer.set_scissors(0, iter::once(viewport.rect));

                    command_buffer.begin_render_pass(
                        render_pass,
                        &framebuffer,
                        viewport.rect,
                        iter::once(command::RenderAttachmentInfo {
                            image_view: std::borrow::Borrow::borrow(&surface_image),
                            clear_value: command::ClearValue {
                                color: command::ClearColor {
                                    float32: [0.0, 0.0, 0.0, 1.0],
                                },
                            },
                        }),
                        SubpassContents::Inline,
                    );

                    command_buffer.bind_graphics_pipeline(pipeline);

                    command_buffer.draw(0..4, 0..1);

                    command_buffer.end_render_pass();
                    command_buffer.finish();

                    queue_group.queues[0].submit(
                        iter::once(&command_buffer),
                        iter::empty(),
                        iter::once(&res.rendering_complete_semaphore),
                        Some(&mut res.submission_complete_fence),
                    );
                    // present frame
                    if let Err(_) = queue_group.queues[0].present(
                        &mut res.surface,
                        surface_image,
                        Some(&mut res.rendering_complete_semaphore),
                    ) {
                        _should_configure_swapchain = true;
                        res.device.destroy_framebuffer(framebuffer);
                    }
                }
            }
            _ => (),
        }
    })
}
