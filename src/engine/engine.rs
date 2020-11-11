use crate::engine::*;
use std::{path::Path, rc::Rc, sync::Mutex};
use winit::{event::*, window::Window};

pub struct Engine {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub timer: timer::Timer,
    pub music: Option<music::Music>,

    renderers: Mutex<Vec<Box<dyn renderer::Renderer>>>,
    asset_library: Mutex<assets::AssetLibrary>,
    ext_command_buffers: Mutex<Vec<wgpu::CommandBuffer>>,
}

#[allow(dead_code)]
impl Engine {
    pub async fn new(window: &Window, assets_path: &Path) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(window);
            (size, surface)
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: Default::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let asset_library = assets::AssetLibrary::new(assets_path);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            size,
            timer: timer::Timer::new(),
            music: None,

            renderers: Mutex::new(vec![]),
            asset_library: Mutex::new(asset_library),
            ext_command_buffers: Mutex::new(vec![]),
        }
    }

    pub fn set_music(&mut self, bytes: &[u8]) {
        self.music = Some(music::Music::from_bytes(bytes));
    }

    pub fn add_renderer(&self, renderer: Box<dyn renderer::Renderer>) {
        self.renderers.lock().unwrap().push(renderer);
    }

    pub fn add_command_buffer(&self, command_buffer: wgpu::CommandBuffer) {
        self.ext_command_buffers
            .lock()
            .unwrap()
            .push(command_buffer);
    }

    pub fn load_asset(&self, path: &Path) -> Rc<assets::Asset> {
        self.asset_library.lock().unwrap().load(path)
    }

    pub fn add_asset(&self, path: &Path, data: &[u8]) -> Rc<assets::Asset> {
        self.asset_library.lock().unwrap().add(path, data)
    }

    pub fn get_dir_of_asset(&self, asset: &assets::Asset) -> PathBuf {
        self.asset_library.lock().unwrap().asset_dir(asset)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        const REWIND_AMOUNT: f64 = 10.0;

        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    self.forward(-REWIND_AMOUNT);
                    return true;
                }

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    self.forward(REWIND_AMOUNT);
                    return true;
                }

                _ => {}
            },
            _ => {}
        }
        false
    }

    pub fn init(&mut self) {
        self.process_ext_command_buffers();
        #[cfg(watcher)]
        self.asset_library.lock().unwrap().start_watcher();
        if let Some(music) = self.music.as_mut() {
            music.play();
        }
        self.timer.reset();
    }

    /// Renders a frame and returns used time in
    pub fn render(&mut self) {
        #[cfg(watcher)]
        self.check_changed_files();
        self.process_ext_command_buffers();

        let frame = self
            .swap_chain
            .get_current_frame()
            .expect("Timeout getting a frame texture");

        let mut renderers = self.renderers.lock().unwrap();
        let mut context = renderer::RenderingContext {
            device: &self.device,
            queue: &mut self.queue,
            output: &frame.output.view,
            time: self.timer.elapsed(),
            screen_size: &self.size,
        };

        for renderer in renderers.iter_mut() {
            if renderer.should_render(&context) {
                renderer.update(&mut context);
                renderer.render(&mut context);
            }
        }
    }

    pub fn elapsed(&self) -> f64 {
        self.timer.elapsed()
    }

    fn forward(&mut self, seconds: f64) {
        if let Some(music) = self.music.as_mut() {
            music.forward(seconds);
        }
        self.timer.forward(seconds);
    }

    #[cfg(watcher)]
    fn check_changed_files(&mut self) {
        let mut assets_lock = self.asset_library.try_lock();
        if let Ok(ref mut assets) = assets_lock {
            if assets.detect_changes() {
                let mut renderers = self.renderers.lock().unwrap();
                for renderer in renderers.iter_mut() {
                    if let Err(error) = renderer.reload_assets(&assets) {
                        eprintln!("Error: {:?}", error);
                    }
                }
                assets.clear_assets();
            }
        }
    }

    fn process_ext_command_buffers(&mut self) {
        let buffers = self.ext_command_buffers.get_mut().unwrap();
        if !buffers.is_empty() {
            self.queue.submit(buffers.drain(0..));
        }
    }
}
