use egui::epaint::text::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use std::time::Instant;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

pub struct Renderer {
    surface_config: SurfaceConfiguration,
    egui_rpass: RenderPass,
    platform: Platform,
    surface: Surface,
    device: Device,
    queue: Queue,
}

pub fn init(window: &winit::window::Window) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    let surface = unsafe { instance.create_surface(window) }.unwrap();

    // WGPU 0.11+ support force fallback (if HW implementation not supported), set it to true or false (optional).
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();

    let size = window.inner_size();
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        // .filter(|f| f.describe().srgb) // describe not defined here anymore ... ?!?
        .next()
        .unwrap_or(surface_caps.formats[0]);
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &surface_config);

    // We use the egui_winit_platform crate as the platform.
    let platform = Platform::new(PlatformDescriptor {
        physical_width: size.width as u32,
        physical_height: size.height as u32,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    // We use the egui_wgpu_backend crate as the render backend.
    let egui_rpass = egui_wgpu_backend::RenderPass::new(&device, surface_format, 1);

    Renderer {
        surface_config,
        egui_rpass,
        platform,
        surface,
        device,
        queue,
    }
}

pub fn update(
    event: &winit::event::Event<'_, ()>,
    window: &winit::window::Window,
    renderer: &mut Renderer,
    draw_ui: &mut Box<dyn FnMut(&egui::Context, &mut super::state::State)>,
    state: &mut super::state::State,
) {
    // Pass the winit events to the platform integration.
    let start_time = Instant::now();
    renderer.platform.handle_event(&event);

    match event {
        winit::event::Event::RedrawRequested(..) => {
            renderer
                .platform
                .update_time(start_time.elapsed().as_secs_f64());

            let output_frame = match renderer.surface.get_current_texture() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return;
                }
                Err(e) => {
                    eprintln!("Dropped frame with error: {}", e);
                    return;
                }
            };
            let output_view = output_frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            // Begin to draw the UI frame.
            renderer.platform.begin_frame();

            // Draw the application
            (draw_ui(&renderer.platform.context(), state));

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let full_output = renderer.platform.end_frame(Some(window));
            let paint_jobs = renderer.platform.context().tessellate(full_output.shapes);

            let mut encoder =
                renderer
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("encoder"),
                    });

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: renderer.surface_config.width,
                physical_height: renderer.surface_config.height,
                scale_factor: window.scale_factor() as f32,
            };
            let tdelta = full_output.textures_delta;
            renderer
                .egui_rpass
                .add_textures(&renderer.device, &renderer.queue, &tdelta)
                .expect("add texture ok");
            renderer.egui_rpass.update_buffers(
                &renderer.device,
                &renderer.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            // Record all render passes.
            renderer
                .egui_rpass
                .execute(
                    &mut encoder,
                    &output_view,
                    &paint_jobs,
                    &screen_descriptor,
                    Some(wgpu::Color::BLACK),
                )
                .unwrap();

            // Submit the commands.
            renderer.queue.submit(std::iter::once(encoder.finish()));

            // Redraw egui
            output_frame.present();

            renderer
                .egui_rpass
                .remove_textures(tdelta)
                .expect("remove texture ok");
        }

        // Fix this to use threads
        // MainEventsCleared | UserEvent(Event::RequestRedraw) => {
        winit::event::Event::MainEventsCleared => {
            window.request_redraw();
        }
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::Resized(size) => {
                // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                // See: https://github.com/rust-windowing/winit/issues/208
                // This solves an issue where the app would panic when minimizing on Windows.
                if size.width > 0 && size.height > 0 {
                    renderer.surface_config.width = size.width;
                    renderer.surface_config.height = size.height;
                    renderer
                        .surface
                        .configure(&renderer.device, &renderer.surface_config);
                }
            }
            _ => {}
        },
        _ => (),
    }
}