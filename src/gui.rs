// - - - IMPORTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};
// use eframe::egui;

use tray_icon::menu::{Menu, MenuEvent, MenuItem}; //, MenuItemExt, PredefinedMenuItem, Submenu},
use tray_icon::TrayIcon;
use tray_icon::TrayIconBuilder;
// use tray_icon::TrayEvent;

pub struct Gui {
    pub event_loop: Option<EventLoop<()>>,
    tray: TrayIcon,
    menu_elements: Vec<MenuElement>,
    window: Window,
}

struct MenuElement {
    id: u32,
    item: MenuItem,
    handler: fn(&mut Gui, &mut ControlFlow),
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

        let window_builder = WindowBuilder::new()
            .with_visible(false)
            .with_title("Watchdog")
            .with_window_icon(Some(window_icon));

        let window = window_builder.build(&event_loop).unwrap();

        //
        // - - - SYSTEM TRAY
        //
        // menu
        let menu = Box::new(Menu::new());
        let mut menu_elements: Vec<MenuElement> = Vec::new();

        let open = MenuElement::new("Open Window", true, Gui::open_window);
        let close = MenuElement::new("Close Window", true, Gui::close_window);
        let exit = MenuElement::new("Exit", true, Gui::exit);

        menu.append(&open.item.clone());
        menu.append(&close.item.clone());
        menu.append(&exit.item.clone());

        menu_elements.push(open);
        menu_elements.push(close);
        menu_elements.push(exit);

        // tray entity
        let tray = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("Watchdog")
            .with_icon(tray_icon)
            .build()
            .unwrap();

        Gui {
            tray,
            menu_elements,
            event_loop: Some(event_loop),
            window,
        }
    }

    pub fn update<T>(&mut self, event: Event<'_, T>, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            for i in &self.menu_elements {
                if i.id == event.id {
                    (i.handler)(self, control_flow);
                    break;
                }
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => self.close_window(control_flow),
            _ => {}
        }
    }

    fn open_window(&mut self, _: &mut ControlFlow) {
        self.window.set_visible(true);
    }

    fn close_window(&mut self, _: &mut ControlFlow) {
        self.window.set_visible(false);
    }

    fn hide_tray_icon(&mut self) {
        self.tray.set_visible(false).unwrap();
    }

    fn exit(&mut self, control_flow: &mut ControlFlow) {
        self.hide_tray_icon();
        *control_flow = ControlFlow::Exit;
    }
}

// fn init_gui() -> eframe::Result<()> {
//     // Our application state:
//     let mut name = "Arthur".to_owned();
//     let mut age = 42;

//     let options = eframe::NativeOptions::default();
//     eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 let name_label = ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut name)
//                     .labelled_by(name_label.id);
//             });
//             ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
//             if ui.button("Click each year").clicked() {
//                 age += 1;
//             }
//             ui.label(format!("Hello '{name}', age {age}"));
//         });
//     })
// }
