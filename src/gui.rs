// - - - IMPORTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
use std::collections::HashMap;
use std::iter;
use std::time::Instant;

use wgpu::Surface;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use egui::epaint::text::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};

use tray_icon::menu::{Menu, MenuEvent, MenuItem}; //, MenuItemExt, PredefinedMenuItem, Submenu},
use tray_icon::{ClickEvent, TrayEvent, TrayIcon, TrayIconBuilder};

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

pub struct Gui {
    pub event_loop: Option<EventLoop<()>>,
    _tray: TrayIcon,
    menu_elements: HashMap<String, MenuElement>,
    window: Window,
    platform: Platform,
    surface: Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    egui_rpass: RenderPass,
    state: State,
}

struct MenuElement {
    id: u32,
    item: MenuItem,
    handler: fn(&mut Gui, &mut ControlFlow),
}

struct State {
    age: u32,
    name: String,
}

impl MenuElement {
    fn new(text: &str, active: bool, handler: fn(&mut Gui, &mut ControlFlow)) -> Self {
        let item = MenuItem::new(text, active, None);
        let id = item.id();
        MenuElement { id, item, handler }
    }
}

impl Gui {
    pub fn new() -> Gui {
        let event_loop = EventLoopBuilder::with_user_event().build();

        //
        // - - - ICON
        //
        const ICON_IMAGE_DATA: &[u8] = include_bytes!("../assets/icons/watchdog-logo.png");
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(ICON_IMAGE_DATA)
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let window_icon =
            winit::window::Icon::from_rgba(icon_rgba.clone(), icon_width, icon_height)
                .expect("Failed to open window icon");

        let tray_icon = tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .expect("Failed to open tray icon");

        let window = WindowBuilder::new()
            .with_visible(false)
            .with_title("Watchdog")
            .with_window_icon(Some(window_icon))
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_inner_size(winit::dpi::PhysicalSize {
                width: INITIAL_WIDTH,
                height: INITIAL_HEIGHT,
            })
            .build(&event_loop)
            .unwrap();

        //
        // - - - SYSTEM TRAY
        //
        // menu
        let menu = Box::new(Menu::new());

        // let open = MenuElement::new("Open Window", true, Gui::open_window);
        // let close = MenuElement::new("Close Window", true, Gui::close_window);
        let exit = MenuElement::new("Exit", true, Gui::exit);

        // menu.append(&open.item.clone());
        // menu.append(&close.item.clone());
        menu.append(&exit.item.clone());

        let mut menu_elements = HashMap::new();
        // menu_elements.insert(String::from("open"), open);
        // menu_elements.insert(String::from("close"), close);
        menu_elements.insert(String::from("exit"), exit);

        // tray entity
        let _tray = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("Watchdog")
            .with_icon(tray_icon)
            .build()
            .unwrap();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

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

        let size = &window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .filter(|f| f.describe().srgb) // describe not defined here anymore ... ?
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

        // Display the demo application that ships with egui.
        // let mut demo_app = egui_demo_lib::DemoWindows::default();

        // Our application state:
        let state = State {
            name: String::from("Blake"),
            age: 31,
        };

        Gui {
            _tray,
            menu_elements,
            event_loop: Some(event_loop),
            window,
            platform,
            surface,
            device,
            queue,
            surface_config,
            egui_rpass,
            state,
        }
    }

    pub fn init_ui(&mut self) {}

    pub fn update<T>(&mut self, event: winit::event::Event<'_, T>, control_flow: &mut ControlFlow) {
        let start_time = Instant::now();
        *control_flow = ControlFlow::Poll;

        // Tray Event
        if let Ok(event) = TrayEvent::receiver().try_recv() {
            if event.event == ClickEvent::Left {
                self.open_window(control_flow);
            }
        }
        // Tray Menu Event
        else if let Ok(event) = MenuEvent::receiver().try_recv() {
            for (_key, element) in &self.menu_elements {
                if element.id == event.id {
                    (element.handler)(self, control_flow);
                    break;
                }
            }
        }

        // EGUI + WINIT THINGZ
        // Pass the winit events to the platform integration.
        self.platform.handle_event(&event);

        match event {
            Event::RedrawRequested(..) => {
                self.platform
                    .update_time(start_time.elapsed().as_secs_f64());

                let output_frame = match self.surface.get_current_texture() {
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
                self.platform.begin_frame();

                // Draw the hello world application.
                egui::CentralPanel::default().show(&self.platform.context(), |ui| {
                    ui.heading("My egui Application");
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Your name: ");
                        ui.text_edit_singleline(&mut self.state.name)
                            .labelled_by(name_label.id);
                    });
                    ui.add(egui::Slider::new(&mut self.state.age, 0..=120).text("age"));
                    if ui.button("Click each year").clicked() {
                        self.state.age += 1;
                    }
                    ui.label(format!(
                        "Hello '{}', age {}",
                        self.state.name, self.state.age,
                    ));
                });

                // End the UI frame. We could now handle the output and draw the UI with the backend.
                let full_output = self.platform.end_frame(Some(&self.window));
                let paint_jobs = self.platform.context().tessellate(full_output.shapes);

                let mut encoder =
                    self.device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("encoder"),
                        });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: self.surface_config.width,
                    physical_height: self.surface_config.height,
                    scale_factor: self.window.scale_factor() as f32,
                };
                let tdelta = full_output.textures_delta;
                self.egui_rpass
                    .add_textures(&self.device, &self.queue, &tdelta)
                    .expect("add texture ok");
                self.egui_rpass.update_buffers(
                    &self.device,
                    &self.queue,
                    &paint_jobs,
                    &screen_descriptor,
                );

                // Record all render passes.
                self.egui_rpass
                    .execute(
                        &mut encoder,
                        &output_view,
                        &paint_jobs,
                        &screen_descriptor,
                        Some(wgpu::Color::BLACK),
                    )
                    .unwrap();
                // Submit the commands.
                self.queue.submit(iter::once(encoder.finish()));

                // Redraw egui
                output_frame.present();

                self.egui_rpass
                    .remove_textures(tdelta)
                    .expect("remove texture ok");
            }

            // Fix this to use threads
            // MainEventsCleared | UserEvent(Event::RequestRedraw) => {
            winit::event::Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                    // See: https://github.com/rust-windowing/winit/issues/208
                    // This solves an issue where the app would panic when minimizing on Windows.
                    if size.width > 0 && size.height > 0 {
                        self.surface_config.width = size.width;
                        self.surface_config.height = size.height;
                        self.surface.configure(&self.device, &self.surface_config);
                    }
                }
                WindowEvent::CloseRequested => {
                    // *control_flow = ControlFlow::Exit;
                    self.close_window(control_flow)
                }
                _ => {}
            },
            _ => (),
        }
    }

    fn open_window(&mut self, _: &mut ControlFlow) {
        self.window.set_visible(true);
        self.window.focus_window();
    }

    fn close_window(&mut self, _: &mut ControlFlow) {
        self.window.set_visible(false);
    }

    fn exit(&mut self, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit;
    }
}

// A custom event type for the winit app.
// enum Event {
//     RequestRedraw,
// }

// This is the repaint signal type that egui needs for requesting a repaint from another thread.
// It sends the custom RequestRedraw event to the winit event loop.

/*
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl egui_wgpu_backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}
*/
