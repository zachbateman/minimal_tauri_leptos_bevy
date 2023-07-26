
use leptos::*;
use bevy::prelude::*;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue, JsCast};

mod leptos_interface;
use leptos_interface::App;

mod bevy_interface;
use bevy_interface::BevyInterfacePlugin;




#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}




fn main() {
    // Launch Leptos
    let mount_point = document().query_selector("#leptos_div").unwrap().unwrap();
    mount_to(mount_point.unchecked_into(), |cx| {
        view! { cx,
            <App />
        }
    });

    // Launch Bevy
    bevy::app::App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                canvas: Some("#bevy_canvas".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BevyInterfacePlugin)
        .run();
}
