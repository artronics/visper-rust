use crate::renderer::target::Target;
use crate::primitive::quad;
use wgpu::{
    Device, Adapter, RequestAdapterOptions, BackendBit, DeviceDescriptor, Limits,
    CommandEncoderDescriptor, PowerPreference, Extensions
};
use crate::core::rectangle::Rectangle;
use crate::primitive::quad::Quad;

pub mod target;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    quad_pipeline: quad::Pipeline,
}

impl Renderer {
    pub fn device(&self) -> &Device {
        &self.device
    }
}

impl Renderer {
    pub fn new() -> Self {
        let adapter = Adapter::request(&RequestAdapterOptions {
            power_preference: PowerPreference::Default,
            backends: BackendBit::all(),
        }).expect("Request adaptor failed");

        let (mut device, queue) = adapter.request_device(&DeviceDescriptor {
            extensions: Extensions {
                anisotropic_filtering: false,
            },
            limits: Limits { max_bind_groups: 2 },
        });

        let quad_pipeline = quad::Pipeline::new(&mut device);

        Renderer {
            device,
            queue,
            quad_pipeline
        }
    }

    pub fn draw(&mut self, target: &mut Target) {
        let (width, height) = target.dimensions();
        let scale_factor = target.scale_factor();
        let transformation = target.transformation();
        let frame = target.next_frame();

        let mut encoder = self.device
            .create_command_encoder(&CommandEncoderDescriptor { todo: 0 });


        // NOTE: it looks like we create two render passes; one in primitive and one here. Why?
/*
        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            }],
            depth_stencil_attachment: None,
        });
*/

        let bounds = Rectangle {
            x: 30,
            y: 30,
            width: 100,
            height: 100
        };
        let quad = Quad {
            position: [30.0, 30.0],
        scale: [100.0, 100.0],
        color: [1.0, 0.0, 1.0, 1.0],
        border_color: [0.0, 0.0, 1.0, 1.0],
        border_radius: 5.03,
        border_width: 3.05,

        };
        let quads = &[quad];

        self.quad_pipeline.draw(
            &mut self.device,
            &mut encoder,
            transformation,
            scale_factor,
            bounds,
            quads,
            &frame.view
        );
        self.queue.submit(&[encoder.finish()]);
    }
}

