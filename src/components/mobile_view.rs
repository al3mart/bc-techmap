use leptos::prelude::*;

use crate::app::AppState;
use crate::data::ecosystem::Ecosystem;

#[component]
pub fn MobileView(
    ecosystems: Vec<Ecosystem>,
    state: ReadSignal<AppState>,
    #[prop(into)] on_card_click: Callback<String>,
) -> impl IntoView {
    let cards = ecosystems
        .into_iter()
        .map(|eco| {
            let id = eco.id.clone();
            let name = eco.name.clone();
            let languages = eco.languages.join(", ");
            let vm = eco.vm.clone();
            let deploy = eco.deployment_options.join(" / ");
            let evm_compat = eco.evm_compatibility.clone();
            let chain_layer = eco.chain_layer.clone();

            let click_id = id.clone();
            let on_click = move |_: web_sys::MouseEvent| {
                on_card_click.run(click_id.clone());
            };

            let card_class = {
                let id = id.clone();
                move || {
                    match state.get() {
                        AppState::SourceSelected { ref source_id } if *source_id == id => {
                            "mobile-card source-selected"
                        }
                        AppState::ShowResults { ref source_id, .. } if *source_id == id => {
                            "mobile-card source-selected"
                        }
                        AppState::ShowResults { ref dest_id, .. } if *dest_id == id => {
                            "mobile-card dest-selected"
                        }
                        _ => "mobile-card",
                    }
                }
            };

            let evm_badge = match evm_compat.as_str() {
                "native" => Some("EVM"),
                "supported" => Some("EVM compat"),
                _ => None,
            };

            let l2_badge = if chain_layer == "ETH L2" {
                Some("L2")
            } else {
                None
            };

            view! {
                <div class=card_class on:click=on_click>
                    <div class="mobile-card-header">
                        <span class="mobile-card-name">{name}</span>
                        <div class="eco-card-badges">
                            {l2_badge.map(|badge| view! {
                                <span class="eco-card-badge l2-badge">{badge}</span>
                            })}
                            {evm_badge.map(|badge| view! {
                                <span class="eco-card-badge">{badge}</span>
                            })}
                        </div>
                    </div>
                    <div class="mobile-card-details">
                        <span class="mobile-card-detail">"Lang: "{languages}</span>
                        <span class="mobile-card-detail">"VM: "{vm}</span>
                        <span class="mobile-card-detail">"Deploy: "{deploy}</span>
                    </div>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="mobile-list">
            {cards}
        </div>
    }
}
