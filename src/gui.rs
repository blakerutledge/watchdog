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
    _tray: TrayIcon,
    menu_elements: Vec<MenuElement>,
    window: Window,
}

struct MenuElement {
    id: u32,
    item: MenuItem,
    handler: fn(&Gui),
}

impl MenuElement {
    fn new(text: &str, active: bool, handler: fn(&Gui)) -> Self {
        let item = MenuItem::new(text, active, None);
        let id = item.id();
        MenuElement { id, item, handler }
    }
}

impl Gui {
    pub fn new() -> Gui {
        let event_loop = EventLoopBuilder::with_user_event().build();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_visible(false);
        let menu = Box::new(Menu::new());
        let mut menu_elements: Vec<MenuElement> = Vec::new();

        //
        // - - - SYSTEM TRAY
        //
        // menu

        let open = MenuElement::new("Open Window", true, Gui::open_window);
        let close = MenuElement::new("Close Window", true, Gui::close_window);

        menu.append(&open.item.clone());
        menu.append(&close.item.clone());

        menu_elements.push(open);
        menu_elements.push(close);

        // icon
        let path = std::path::Path::new("./assets/images/watchdog-logo.png");
        dbg!(path);
        let icon = load_icon(path);
        // tray entity
        let _tray = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("Watchdog")
            .with_icon(icon)
            .build()
            .unwrap();

        Gui {
            _tray,
            menu_elements,
            event_loop: Some(event_loop),
            window,
        }
    }

    pub fn update<T>(&self, event: Event<'_, T>, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            for i in &self.menu_elements {
                if i.id == event.id {
                    (i.handler)(self);
                }
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    }

    fn open_window(&self) {
        println!("TODO: Open Window, {:?}", self.window);
        self.window.set_visible(true);
    }

    fn close_window(&self) {
        println!("TODO: Close Window");
        self.window.set_visible(false);
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open icon")
}

// fn open_window() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();
//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Wait;

//         // exit
//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 window_id,
//             } if window_id == window.id() => *control_flow = ControlFlow::Exit,
//             _ => (),
//         }
//     });
// }

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
