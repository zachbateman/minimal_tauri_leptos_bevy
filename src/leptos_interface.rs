use serde::Serialize;
use leptos::*;
use leptos::leptos_dom::ev::MouseEvent;
use serde_wasm_bindgen::to_value;

// use crate::{invoke, invoke_no_args, invoke_with_args};
use crate::invoke;


#[derive(Serialize)]
struct NewBoxArgs<'a> {
    new: &'a bool,
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
            let args = to_value(&NewBoxArgs { new: &true }).unwrap();
            invoke("add_box", args).await;
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
