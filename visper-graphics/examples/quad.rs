use wgpu::{BackendBit, BindGroupLayoutBinding, BindingType, TextureViewDimension};
use winit::{
    event::Event::{ WindowEvent, RedrawRequested },
    event_loop::{ControlFlow, EventLoop},
};
use visper_graphics::primitive::quad::Pipeline;
use visper_graphics::transformation::Transformation;
use winit::dpi::{Size, LogicalSize, PhysicalSize};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let event_loop = EventLoop::new();

    let (window, size, surface) = {
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size();
        let surface = wgpu::Surface::create(&window);
        (window, size, surface)
    };

    let sl = LogicalSize { width: 300.0, height: 300.0 };
    let sp = PhysicalSize { width: 300, height: 300 };
    window.set_inner_size(Size::Logical(sl));
//    window.set_inner_size(Size::Physical(sp));

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: BackendBit::all(),
    }).unwrap();

    let (mut device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let size = window.inner_size();
    let mut swap_chain = create_swap_chain(&surface, size.width, size.height, &device);
    // Render loop
    window.request_redraw();

    let mut quad_pipeline = Pipeline::new(&mut device);

//    let mut current_time_ms = SystemTime::now();

    event_loop.run(move |event, _, control_flow| match event {
        WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *control_flow = winit::event_loop::ControlFlow::Exit,

        WindowEvent {
            event: winit::event::WindowEvent::ScaleFactorChanged { .. },
            ..
        } => {
//            window.request_redraw();
        },
        WindowEvent {
            event: winit::event::WindowEvent::Resized(physicalSize),
            ..
        } => {
            swap_chain = create_swap_chain(&surface, physicalSize.width, physicalSize.height, &device);
//            window.request_redraw();
            let frame = swap_chain
                .get_next_texture();
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            let t = Transformation::orthographic(physicalSize.width as u16, physicalSize.height as u16);
//            let t = get_transformation(&window);
            quad_pipeline.draw(&mut device, &mut encoder, t, 1.0, &frame.view);

            queue.submit(&[encoder.finish()]);
        },

        RedrawRequested(_) => {
            let frame = swap_chain
                .get_next_texture();
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

            let t = get_transformation(&window);
            quad_pipeline.draw(&mut device, &mut encoder, t, 1.0, &frame.view);

            queue.submit(&[encoder.finish()]);
//            let new_time = SystemTime::now();
//            let result = new_time.duration_since(current_time_ms).expect("can't get time");
//            if result.as_micros() > 16666 {
//                current_time_ms = new_time;

//            }
        }

        _ => {
            *control_flow = winit::event_loop::ControlFlow::Wait;
        }
    })
}

/*
fn draw(pipeline: &mut Pipeline, device: &mut wgpu::Device, swap_chain: &mut wgpu::SwapChain, encoder: &mut wgpu::CommandEncoder, window: &winit::window::Window) {
    let frame = swap_chain
        .get_next_texture();
//    let mut encoder =
//        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let t = get_transformation(&window);
    pipeline.draw(&mut device, &mut encoder, t, 1.0, &frame.view);

    queue.submit(&[encoder.finish()]);
}
*/

fn get_transformation(window: &winit::window::Window) -> Transformation {
    let physical_size = window.inner_size();
    let scale_factor = window.scale_factor();
//    let (width, height) = (physical_size.to_logical(scale_factor).width, physical_size.to_logical(scale_factor).height);
    let (width, height) = (physical_size.width, physical_size.height);

    Transformation::orthographic(width as u16, height as u16)
}

fn create_swap_chain(surface: &wgpu::Surface, width: u32, height: u32, device: &wgpu::Device) -> wgpu::SwapChain {
    device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Vsync,
        },
    )
}