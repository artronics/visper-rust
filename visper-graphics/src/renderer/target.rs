use crate::transformation::Transformation;
use winit::window::Window;

#[derive(Debug)]
pub struct Target {
    surface: wgpu::Surface,
    width: u16,
    height: u16,
    scale_factor: f64,
    transformation: Transformation,
    swap_chain: wgpu::SwapChain,
}

impl Target {
    pub fn new(device: &wgpu::Device, window: &winit::window::Window, width: u16, height: u16, scale_factor: f64) -> Self {
        let surface = wgpu::Surface::create(window);
        let swap_chain = new_swap_chain(device, &surface, width, height);

        Target {
            surface,
            width,
            height,
            scale_factor,
            transformation: Transformation::orthographic(width, height),
            swap_chain,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u16, height: u16, scale_factor: f64) {
        self.width = width;
        self.height = height;
        self.scale_factor = scale_factor;
        self.transformation = Transformation::orthographic(width, height);
        self.swap_chain = new_swap_chain(device, &self.surface, width, height);
    }

    pub fn dimensions(&self) -> (u16, u16) { (self.width, self.height) }

    pub fn scale_factor(&self) -> f64 { self.scale_factor }

    pub fn transformation(&self) -> Transformation { self.transformation }

    pub fn next_frame(&mut self) -> wgpu::SwapChainOutput { self.swap_chain.get_next_texture() }
}

fn new_swap_chain(
    device: &wgpu::Device,
    surface: &wgpu::Surface,
    width: u16,
    height: u16,
) -> wgpu::SwapChain {
    device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: u32::from(width),
            height: u32::from(height),
            present_mode: wgpu::PresentMode::Vsync,
        },
    )
}
