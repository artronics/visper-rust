use std::mem::size_of;
use crate::transformation::Transformation;
use crate::core::rectangle::Rectangle;

#[derive(Debug)]
pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    constants: wgpu::BindGroup,
    constants_buffer: wgpu::Buffer,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    instances: wgpu::Buffer,
}

impl Pipeline {
    pub fn new(device: &mut wgpu::Device) -> Pipeline {
        let vs_module: wgpu::ShaderModule;
        let fs_module: wgpu::ShaderModule;
        let (vertex_stage, fragment_stage) = {
            let vs = include_bytes!("shaders/hello/vert");
            vs_module = device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(&vs[..]))
                    .expect("Read quad vertex shader as SPIR-V"),
            );

            let fs = include_bytes!("shaders/hello/frag");
            fs_module = device.create_shader_module(
                &wgpu::read_spirv(std::io::Cursor::new(&fs[..]))
                    .expect("Read quad fragment shader as SPIR-V"),
            );

            let vertex_stage = wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            };

            let fragment_stage = wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            };

            (vertex_stage, Some(fragment_stage))
        };

        let vertex_buffers = {
            &[
                wgpu::VertexBufferDescriptor {
                    stride: size_of::<Vertex>() as u64,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[wgpu::VertexAttributeDescriptor {
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float2,
                        offset: 0,
                    }],
                },
                wgpu::VertexBufferDescriptor {
                    stride: size_of::<Quad>() as u64,
                    step_mode: wgpu::InputStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float2,
                            offset: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float2,
                            offset: 4 * 2,
                        },
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float4,
                            offset: 4 * (2 + 2),
                        },
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 4,
                            format: wgpu::VertexFormat::Float4,
                            offset: 4 * (2 + 2 + 4),
                        },
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 5,
                            format: wgpu::VertexFormat::Float,
                            offset: 4 * (2 + 2 + 4 + 4),
                        },
                        wgpu::VertexAttributeDescriptor {
                            shader_location: 6,
                            format: wgpu::VertexFormat::Float,
                            offset: 4 * (2 + 2 + 4 + 4 + 1),
                        },
                    ],
                },
            ]
        };

        let (layout, bind_group, constants_buffer) = {
            let constant_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[wgpu::BindGroupLayoutBinding {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    }],
                });

            let constants_buffer = device
                .create_buffer_mapped(
                    1,
                    wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                )
                .fill_from_slice(&[Uniforms::default()]);

            let constants = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &constant_layout,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &constants_buffer,
                        range: 0..std::mem::size_of::<Uniforms>() as u64,
                    },
                }],
            });

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&constant_layout],
            });

            (layout, constants, constants_buffer)
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage,
            fragment_stage,
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Cw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let vertices = device
            .create_buffer_mapped(QUAD_VERTS.len(), wgpu::BufferUsage::VERTEX)
            .fill_from_slice(&QUAD_VERTS);

        let indices = device
            .create_buffer_mapped(QUAD_INDICES.len(), wgpu::BufferUsage::INDEX)
            .fill_from_slice(&QUAD_INDICES);

        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            size: size_of::<Quad>() as u64 * Quad::MAX as u64,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        Pipeline {
            pipeline,
            constants: bind_group,
            constants_buffer,
            vertices,
            indices,
            instances,
        }
    }

    pub fn draw(&mut self,
                device: &mut wgpu::Device,
                encoder: &mut wgpu::CommandEncoder,
                transformation: Transformation,
                scale: f64,
                bounds: Rectangle<u32>,
                instances: &[Quad],
                target: &wgpu::TextureView,
    ) {
        let uniforms = Uniforms::new(transformation, scale);
        let constants_buffer = device
            .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&[uniforms]);

        encoder.copy_buffer_to_buffer(
            &constants_buffer,
            0,
            &self.constants_buffer,
            0,
            std::mem::size_of::<Uniforms>() as u64,
        );

        let amount = 1;
        let instance_buffer = device
            .create_buffer_mapped(amount, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&instances[0..amount]);

        encoder.copy_buffer_to_buffer(
            &instance_buffer,
            0,
            &self.instances,
            0,
            (size_of::<Quad>() * amount) as u64,
        );

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::WHITE,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.constants, &[]);
            rpass.set_vertex_buffers(
                0,
                &[(&self.vertices, 0), (&self.instances, 0)]);
//            rpass.set_vertex_buffers(
//                0,
//                &[(&self.vertices, 0)]);
            rpass.set_index_buffer(&self.indices, 0);
//            rpass.set_scissor_rect(bounds.x, bounds.y, bounds.width, bounds.height);
            rpass.draw_indexed(
                0..QUAD_INDICES.len() as u32,
                0,
                0..1 as u32,
            )
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    _position: [f32; 2],
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        _position: [0.0, 0.0],
    },
    Vertex {
        _position: [1.0, 0.0],
    },
    Vertex {
        _position: [1.0, 1.0],
    },
    Vertex {
        _position: [0.0, 1.0],
    },
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
}

impl Quad {
    const MAX: usize = 100_000;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Uniforms {
    transform: [f32; 16],
    scale: f32,
}

impl Uniforms {
    fn new(transformation: Transformation, scale: f64) -> Uniforms {
        Self {
            transform: *transformation.as_ref(),
            scale: scale as f32,
        }
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::identity().as_ref(),
            scale: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wgpu::BackendBit;

    #[test]
    fn pipeline() {
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

        let p = Pipeline::new(&mut device);
    }

    #[test]
    fn foo() {
        assert_eq!(1, 1)
    }
}