use wgpu::Instance;
use winit::{window::{Window, self}, event::WindowEvent};

struct State {
    surface: wgpu::Surface,
    device : wgpu::Device,
    queue  : wgpu::Queue,
    config : wgpu::SurfaceConfiguration,
    size   : winit::dpi::PhysicalSize<u32>
}

impl State {

    async fn
    new(window: &Window) -> Self {
        let wsize = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backend::all());
        let surface = unsafe {instance.create_surface(window)};
        let adapter: instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();
    }

    fn
    resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn
    input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn
    update(&mut self) {
        todo!()
    }

    fn
    render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
    
}