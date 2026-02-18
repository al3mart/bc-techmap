use leptos::prelude::*;

use crate::data::ecosystem::Ecosystem;

#[component]
pub fn EcosystemCard(
    ecosystem: Ecosystem,
    is_selected: Signal<bool>,
    #[prop(into)] on_click: Callback<String>,
    #[prop(optional)] position_override: Option<(f64, f64)>,
) -> impl IntoView {
    let id = ecosystem.id.clone();
    let name = ecosystem.name.clone();
    let languages = ecosystem.languages.join(", ");
    let vm = ecosystem.vm.clone();
    let evm_compat = ecosystem.evm_compatibility.clone();
    let deploy = ecosystem.deployment_options.join(" / ");
    let chain_layer = ecosystem.chain_layer.clone();

    let (x, y) = position_override.unwrap_or((ecosystem.position[0], ecosystem.position[1]));
    let style = format!("translate: {}px {}px;", x, y);

    let class = move || {
        if is_selected.get() {
            "eco-card selected"
        } else {
            "eco-card"
        }
    };

    let click_id = id.clone();
    let on_card_click = move |_: web_sys::MouseEvent| {
        on_click.run(click_id.clone());
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
        <div
            class=class
            style=style
            data-eco-id=id
            on:click=on_card_click
        >
            <div class="eco-card-header">
                <span class="eco-card-name">{name}</span>
                <div class="eco-card-badges">
                    {l2_badge.map(|badge| view! {
                        <span class="eco-card-badge l2-badge">{badge}</span>
                    })}
                    {evm_badge.map(|badge| view! {
                        <span class="eco-card-badge">{badge}</span>
                    })}
                </div>
            </div>
            <div class="eco-card-detail">"Lang: "<span>{languages}</span></div>
            <div class="eco-card-detail">"VM: "<span>{vm}</span></div>
            <div class="eco-card-detail">"Deploy: "<span>{deploy}</span></div>
        </div>
    }
}
