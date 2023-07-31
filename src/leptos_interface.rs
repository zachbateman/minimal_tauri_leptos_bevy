use serde::Serialize;
use leptos::*;
use leptos::leptos_dom::ev::MouseEvent;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;

use crate::invoke;


/// Use this to run "invoke" without args.
/// (Have to have some sort of args for how "invoke" is brought in as a Rust function.)
#[derive(Serialize)]
struct NoArgs;
fn no_args() -> JsValue {
    to_value(&NoArgs).unwrap()
}

#[component]
fn UiBox(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    let change_num = {move |_| set_count.set(count.get() + 1)};

    let add_box = move |ev: MouseEvent| {
        // Modify signal within Leptos
        change_num(ev);

        // Fire off Tauri command that will update Tauri global state
        spawn_local(async move {
            invoke("add_box", no_args()).await;
        });
    };


    view! { cx,
        <div class="ui_box">
            <p>"Minimal Tauri + Leptos + Bevy Example."</p>
            <p>{count}</p>
            <button on:click=add_box>"Add One Box"</button>
        </div>
    }
}


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <main class="leptos_container">
            <UiBox />
        </main>
    }
}
