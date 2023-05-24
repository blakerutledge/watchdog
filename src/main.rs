// HIDE console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// - - - IMPORTS - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

use eframe::egui;

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    TrayEvent, TrayIcon, TrayIconBuilder,
};

// - - - MAIN - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

struct WatchdogApp {
    tray: TrayIcon,
    // menu: Box<Menu>,
    event_loop: EventLoop<()>,
    event_task: Box<dyn FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow)>,
    window: Window,
    // window_loop: EventLoop<()>,
    // other state ...
}

impl WatchdogApp {
    fn build() -> WatchdogApp {
        //
        // - - - SYSTEM TRAY
        //
        // menu
        let menu = Menu::new();
        let menu_item_open = MenuItem::new("Open Window", true, None);
        let menu_item_exit = MenuItem::new("Exit App", true, None);
        menu.append(&menu_item_open);
        menu.append(&menu_item_exit);
        // icon
        let path = std::path::Path::new("./assets/images/watchdog-logo.png");
        dbg!(path);
        let icon = load_icon(path);
        // tray entity
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
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

        // FnMut(Event<'_, T>, &EventLoopWindowTarget<T>, &mut ControlFlow),
        // F: 'static + FnMut(Event<'_, T>, &EventLoopWindowTarget<T>, &mut ControlFlow),
        let event_task = Box::new(
            move |event: Event<'_, ()>,
                  _: &EventLoopWindowTarget<()>,
                  mut control_flow: &mut ControlFlow| {
                *control_flow = ControlFlow::Wait;

                if let Ok(event) = MenuEvent::receiver().try_recv() {
                    // TO DO need scope fixed like everywhere ?

                    //     if event.id == menu_item_open.id() {
                    //         println!("open")s
                    //     } else if event.id == menu_item_exit.id() {
                    //         println!("close")
                    //     }
                    // }
                    println!("{:?}", event.id);
                }

                // exit
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            },
        );

        WatchdogApp {
            tray,
            // menu,
            event_loop,
            event_task,
            window,
        }
    }
}

fn main() {
    let app = WatchdogApp::build();

    app.event_loop.run(app.event_task);

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

fn open_window() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.run(move |event, _, control_flow| {
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

fn init_gui() -> eframe::Result<()> {
    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
