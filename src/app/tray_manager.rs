use std::collections::HashMap;
use tray_icon::menu::{Menu, MenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

pub struct MenuElement {
    id: u32,
    item: MenuItem,
    handler: String,
}

impl MenuElement {
    fn new(text: &str, active: bool, handler: String) -> Self {
        let item = MenuItem::new(text, active, None);
        let id = item.id();
        MenuElement { id, item, handler }
    }
}

pub fn init() -> (TrayIcon, HashMap<String, MenuElement>) {
    //
    // - - - ICON
    //
    const ICON_IMAGE_DATA: &[u8] = include_bytes!("./../../assets/icons/watchdog-logo.png");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_IMAGE_DATA)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let tray_icon = tray_icon::icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("Failed to open tray icon");

    //
    // - - - SYSTEM TRAY MENU
    //
    // menu
    let menu = Box::new(Menu::new());

    // let open = MenuElement::new("Open Window", true, Gui::open_window);
    // let close = MenuElement::new("Close Window", true, Gui::close_window);
    let exit = MenuElement::new("Exit", true, String::from("app/exit"));

    // menu.append(&open.item.clone());
    // menu.append(&close.item.clone());
    menu.append(&exit.item.clone());

    let mut menu_elements = HashMap::new();
    // menu_elements.insert(String::from("open"), open);
    // menu_elements.insert(String::from("close"), close);
    menu_elements.insert(String::from("exit"), exit);

    // tray entity
    let tray = TrayIconBuilder::new()
        .with_menu(menu)
        .with_tooltip("Watchdog")
        .with_icon(tray_icon)
        .build()
        .unwrap();

    (tray, menu_elements)
}

pub fn update(tray_menu: &HashMap<String, MenuElement>, state: &mut super::state::State) {
    //
    // Listen for left click on tray icon
    if let Ok(event) = tray_icon::TrayEvent::receiver().try_recv() {
        if event.event == tray_icon::ClickEvent::Left {
            state.action_window_open = true
        }
    }
    //
    // Tray Menu Event (left click menu item, after right clicking to open menu)
    else if let Ok(event) = tray_icon::menu::MenuEvent::receiver().try_recv() {
        let instruction = test_handlers(tray_menu, event.id);
        match instruction.as_str() {
            "app/exit" => {
                state.action_exit = true;
            }
            "app/instruction_not_mapped" => {
                println!(
                    "App Error: Menu Item with handle {:?} is not mapped to any action",
                    instruction
                );
            }
            _ => {}
        }
    }
}

pub fn on_exit(tray: &mut TrayIcon) {
    tray.set_visible(false).unwrap();
}

pub fn test_handlers(tray_menu: &HashMap<String, MenuElement>, event_id: u32) -> String {
    for (_key, element) in tray_menu {
        if element.id == event_id {
            return element.handler.clone();
        }
    }
    String::from("app/instruction_not_mapped")
}
