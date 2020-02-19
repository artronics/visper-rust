use winit::{
    event::Event::{ WindowEvent, RedrawRequested },
    event_loop::{ControlFlow, EventLoop},
};
use visper_graphics::renderer::Renderer;
use visper_graphics::renderer::target::Target;
use std::future::Future;
use visper_gui::proxy::Proxy;

fn main() {
    let event_loop = EventLoop::<UiEvent>::with_user_event();
    let ep = event_loop.create_proxy();
    let p = Proxy::new(ep);
    let mut external_messages = Vec::new();

    let window = winit::window::Window::new(&event_loop).unwrap();
//    let window = winit::window::WindowBuilder::new().build(&event_loop).expect("error window");
    let size = window.inner_size();

    let mut renderer = Renderer::new();
    let mut target = Target::new(renderer.device(), &window, size.width as u16, size.height as u16, window.scale_factor());

    let mut events = Vec::new();
    window.request_redraw();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            events.iter().for_each(| event | {
                match event {
                    UiEvent::Resized(w, h) => {
                        println!("kir");
                        target = Target::new(renderer.device(), &window, w.to_owned(), h.to_owned(), window.scale_factor());
                        renderer.draw(&mut target);
                    }
                }

            });

        },
        WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => *control_flow = winit::event_loop::ControlFlow::Exit,

        WindowEvent {
            event: winit::event::WindowEvent::Resized(physicalSize),
            ..
        } => {
//            events.push(winit::event::WindowEvent::Resized(physicalSize));
            events.push(UiEvent::Resized(physicalSize.width as u16, physicalSize.height as u16));
//            target = Target::new(renderer.device(), &window, physicalSize.width as u16, physicalSize.height as u16, window.scale_factor());
//            renderer.draw(&mut target)
//            window.request_redraw();
        },
        winit::event::Event::UserEvent(message) => {
            external_messages.push(message);
        }

        RedrawRequested(_) => {
            renderer.draw(&mut target)
        }

        _ => {
            *control_flow = winit::event_loop::ControlFlow::Wait;
        }
    })
}

#[derive(Debug, Clone, Copy)]
pub enum UiEvent {
    Resized(u16, u16)
}