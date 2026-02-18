use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::app::AppState;
use crate::components::arrow::{Arrow, FixedArrow};
use crate::components::ecosystem_card::EcosystemCard;
use crate::data::ecosystem::Ecosystem;

#[component]
pub fn Canvas(
    ecosystems: Vec<Ecosystem>,
    state: ReadSignal<AppState>,
    mouse_pos: ReadSignal<(f64, f64)>,
    #[prop(into)] on_card_click: Callback<String>,
    on_canvas_click: impl Fn() + 'static + Copy,
    on_mouse_move: impl Fn(f64, f64) + 'static + Copy,
) -> impl IntoView {
    let ecosystems_for_arrow = ecosystems.clone();
    let ecosystems_for_fixed = ecosystems.clone();

    let handle_mousemove = move |ev: web_sys::MouseEvent| {
        on_mouse_move(ev.client_x() as f64, (ev.client_y() - 48) as f64);
    };

    let handle_click = move |ev: web_sys::MouseEvent| {
        let target = ev.target().unwrap();
        let element: &web_sys::Element = target.dyn_ref().unwrap();
        if element.closest(".eco-card").unwrap().is_none() {
            on_canvas_click();
        }
    };

    view! {
        <div
            class="canvas"
            on:mousemove=handle_mousemove
            on:click=handle_click
        >
            {ecosystems
                .iter()
                .map(|eco| {
                    let eco_clone = eco.clone();
                    let eco_id = eco.id.clone();
                    let is_selected = Signal::derive(move || {
                        match state.get() {
                            AppState::SourceSelected { ref source_id } => *source_id == eco_id,
                            AppState::ShowResults { ref source_id, ref dest_id, .. } => {
                                *source_id == eco_id || *dest_id == eco_id
                            }
                            _ => false,
                        }
                    });
                    view! {
                        <EcosystemCard
                            ecosystem=eco_clone
                            is_selected=is_selected
                            on_click=on_card_click
                        />
                    }
                })
                .collect::<Vec<_>>()}

            {move || {
                let st = state.get();
                match st {
                    AppState::SourceSelected { ref source_id } => {
                        let source = ecosystems_for_arrow
                            .iter()
                            .find(|e| e.id == *source_id);
                        if let Some(src) = source {
                            let (mx, my) = mouse_pos.get();
                            Some(view! {
                                <Arrow
                                    source_pos=src.position
                                    end_x=mx
                                    end_y=my
                                />
                            }.into_any())
                        } else {
                            None
                        }
                    }
                    AppState::ShowResults { ref source_id, ref dest_id, .. } => {
                        let source = ecosystems_for_fixed
                            .iter()
                            .find(|e| e.id == *source_id);
                        let dest = ecosystems_for_fixed
                            .iter()
                            .find(|e| e.id == *dest_id);
                        if let (Some(src), Some(dst)) = (source, dest) {
                            Some(view! {
                                <FixedArrow
                                    source_pos=src.position
                                    dest_pos=dst.position
                                />
                            }.into_any())
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }}
        </div>
    }
}
