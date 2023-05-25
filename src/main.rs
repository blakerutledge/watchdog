// HIDE console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// - - - IMPORTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

// use eframe::egui;

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem}, //, MenuItemExt, PredefinedMenuItem, Submenu},
    // TrayEvent,
    TrayIcon,
    TrayIconBuilder,
};

// - - - MAIN - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct WatchdogApp {
    tray: TrayIcon,
    menu_elements: Vec<MenuElement>,
    event_loop: Option<EventLoop<()>>,
    window: winit::window::Window,
    // other state ...
}

struct MenuElement {
    id: u32,
    item: MenuItem,
    handler: fn(),
}

impl MenuElement {
    fn new(text: &str, active: bool, handler: fn()) -> Self {
        let item = MenuItem::new(text, active, None);
        let id = item.id();
        MenuElement { id, item, handler }
    }
}

fn open_window() {
    println!("TODO: Open Window");
}

fn close_window() {
    println!("TODO: Close Window");
}

impl WatchdogApp {
    fn new() -> WatchdogApp {
        //
        // - - - SYSTEM TRAY
        //
        // menu
        let menu = Box::new(Menu::new());
        let mut menu_elements: Vec<MenuElement> = Vec::new();

        let open = MenuElement::new("Open Window", true, open_window);
        let close = MenuElement::new("Close Window", true, close_window);

        menu.append(&open.item.clone());
        menu.append(&close.item.clone());

        menu_elements.push(open);
        menu_elements.push(close);

        // icon
        let path = std::path::Path::new("./assets/images/watchdog-logo.png");
        dbg!(path);
        let icon = load_icon(path);
        // tray entity
        let tray = TrayIconBuilder::new()
            .with_menu(menu)
            .with_tooltip("Watchdog")
            .with_icon(icon)
            .build()
            .unwrap();

        //
        // - - - EVENT LOOP
        //
        let event_loop = EventLoopBuilder::with_user_event().build();

        //
        // - - - GUI WINDOW
        //
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        WatchdogApp {
            tray,
            menu_elements,
            event_loop: Some(event_loop),
            window,
        }
    }

    fn update<T>(&self, event: Event<'_, T>, control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            for i in &self.menu_elements {
                if i.id == event.id {
                    (i.handler)();
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

    fn close_window() {}
}

fn main() {
    let mut app = WatchdogApp::new();

    app.event_loop
        .take()
        .unwrap()
        .run(move |event, _, control_flow| app.update(event, control_flow))

    //     *control_flow = ControlFlow::Wait;

    //     if let Ok(event) = MenuEvent::receiver().try_recv() {
    //         // TO DO need scope fixed like everywhere ?

    //         //     if event.id == menu_item_open.id() {
    //         //         println!("open")s
    //         //     } else if event.id == menu_item_exit.id() {
    //         //         println!("close")
    //         //     }
    //         // }
    //         println!("{:?}", event.id);
    //     }
    // });

    /*
    app.window_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // exit
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
    */
}

// fn init_loop() -> EventLoop<()> {
// let event_loop_builder = EventLoopBuilder::new();
// event_loop_builder.build();

// EventLoop::new();

/*
event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    // if let Ok(event) = TrayEvent::receiver().try_recv() {
    //     println!("raw event: {:?}", event);
    // }

    // match Some(event) {
    // println!("{}", &menu_item_open.id());
    // &menu_item_close.id() => println!("close")
    // }

    // if let Ok(event) = TrayEvent::receiver().try_recv() {
    //     println!("tray event: {:?}", event);
    // }

    // if let Ok(event) = MenuEvent::receiver().try_recv() {
    //     if event.id == menu_item_open.id() {
    //         println!("open")s
    //     } else if event.id == menu_item_exit.id() {
    //         println!("close")
    //     }
    // }
});
*/

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
