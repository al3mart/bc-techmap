use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;

use crate::app::AppState;
use crate::components::arrow::CARD_WIDTH;
use crate::components::arrow::CARD_HEIGHT;
use crate::components::ecosystem_card::EcosystemCard;
use crate::data::ecosystem::Ecosystem;
use crate::data::scoring::compute_migration;

/// Ring line radii — pushed out so innermost clears the center card.
const RING_RADII: [f64; 5] = [150.0, 240.0, 330.0, 420.0, 510.0];
const RING_LABELS: [&str; 5] = ["Trivial", "Easy", "Moderate", "Hard", "Extreme"];
/// Stroke colors for the ring lines (palette: lighter → darker, red for extreme).
const RING_COLORS: [&str; 5] = ["#92ccd6", "#2897a3", "#1bb5ce", "#032144", "#85241e"];

const CENTER_CLEAR: f64 = 90.0;

const RING_NODE_W: f64 = 100.0;
const RING_NODE_H: f64 = 36.0;

/// Returns (inner_radius, outer_radius) for each band.
fn band_bounds() -> [(f64, f64); 5] {
    let r = RING_RADII;
    let mid = |a: f64, b: f64| (a + b) / 2.0;
    [
        (CENTER_CLEAR, mid(r[0], r[1])),
        (mid(r[0], r[1]), mid(r[1], r[2])),
        (mid(r[1], r[2]), mid(r[2], r[3])),
        (mid(r[2], r[3]), mid(r[3], r[4])),
        (mid(r[3], r[4]), r[4] + 60.0),
    ]
}

fn score_to_ring(score: f64) -> usize {
    match score {
        x if x < 0.2 => 0,
        x if x < 0.4 => 1,
        x if x < 0.6 => 2,
        x if x < 0.8 => 3,
        _ => 4,
    }
}

fn ring_difficulty_class(ring: usize) -> &'static str {
    match ring {
        0 => "difficulty-trivial",
        1 => "difficulty-easy",
        2 => "difficulty-moderate",
        3 => "difficulty-hard",
        _ => "difficulty-extreme",
    }
}

#[component]
pub fn RingView(
    ecosystems: Vec<Ecosystem>,
    state: ReadSignal<AppState>,
    #[prop(into)] on_card_click: Callback<String>,
    on_canvas_click: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let (viewport_w, set_viewport_w) = signal(0.0_f64);
    let (viewport_h, set_viewport_h) = signal(0.0_f64);

    let update_viewport = move || {
        let w = web_sys::window().unwrap();
        let width = w.inner_width().unwrap().as_f64().unwrap_or(1200.0);
        let height = w.inner_height().unwrap().as_f64().unwrap_or(800.0);
        set_viewport_w.set(width);
        set_viewport_h.set(height - 48.0);
    };

    update_viewport();

    let resize_closure = Closure::<dyn Fn()>::new(move || {
        update_viewport();
    });
    let _ = web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref());
    resize_closure.forget();

    let ecosystems_for_rings = ecosystems.clone();
    let ecosystems_for_idle = ecosystems.clone();

    let handle_click = move |ev: web_sys::MouseEvent| {
        let target = ev.target().unwrap();
        let element: &web_sys::Element = target.dyn_ref().unwrap();
        if element.closest(".eco-card").unwrap().is_none()
            && element.closest(".ring-node").unwrap().is_none()
        {
            on_canvas_click();
        }
    };

    // Dynamic class: add "has-source" when a source is selected (enables CSS gradient bands)
    let canvas_class = move || {
        match state.get() {
            AppState::Idle => "ring-canvas",
            _ => "ring-canvas has-source",
        }
    };

    view! {
        <div class=canvas_class on:click=handle_click>
            {move || {
                let st = state.get();
                let vw = viewport_w.get();
                let vh = viewport_h.get();
                let cx = vw / 2.0;
                let cy = vh / 2.0;

                match st {
                    AppState::Idle => {
                        let ecos = ecosystems_for_idle.clone();
                        view! {
                            <div class="ring-idle-prompt">
                                <p class="ring-idle-text">"Click an ecosystem to center it in the ring view"</p>
                                <div class="ring-idle-grid">
                                    {ecos.iter().map(|eco| {
                                        let eco_name = eco.name.clone();
                                        let on_click = on_card_click;
                                        let click_id = eco.id.clone();
                                        view! {
                                            <button
                                                class="ring-idle-btn"
                                                on:click=move |_| on_click.run(click_id.clone())
                                            >
                                                {eco_name}
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    }
                    AppState::SourceSelected { ref source_id } | AppState::ShowResults { ref source_id, .. } => {
                        let dest_id = if let AppState::ShowResults { ref dest_id, .. } = state.get() {
                            Some(dest_id.clone())
                        } else {
                            None
                        };

                        let source = ecosystems_for_rings.iter().find(|e| e.id == *source_id).cloned();
                        let others: Vec<Ecosystem> = ecosystems_for_rings.iter()
                            .filter(|e| e.id != *source_id)
                            .cloned()
                            .collect();

                        if let Some(src) = source {
                            let mut ring_buckets: Vec<Vec<(Ecosystem, f64)>> = vec![vec![]; 5];
                            for eco in &others {
                                let report = compute_migration(&src, eco, None, None);
                                let ring_idx = score_to_ring(report.overall);
                                ring_buckets[ring_idx].push((eco.clone(), report.overall));
                            }

                            let center_x = cx - CARD_WIDTH / 2.0;
                            let center_y = cy - CARD_HEIGHT / 2.0;

                            let src_clone = src.clone();
                            let src_id = src.id.clone();
                            let is_center_selected = Signal::derive(move || {
                                matches!(state.get(),
                                    AppState::SourceSelected { ref source_id } if *source_id == src_id
                                ) || matches!(state.get(),
                                    AppState::ShowResults { ref source_id, .. } if *source_id == src_id
                                )
                            });

                            // ── Build ring node views ──
                            let mut ring_node_views = Vec::new();
                            for (ring_idx, bucket) in ring_buckets.iter().enumerate() {
                                let count = bucket.len();
                                let ring_offset = (ring_idx as f64) * 0.3;
                                for (i, (eco, _score)) in bucket.iter().enumerate() {
                                    let angle = if count == 1 {
                                        -std::f64::consts::FRAC_PI_2 + ring_offset
                                    } else {
                                        let base = -std::f64::consts::FRAC_PI_2 + ring_offset;
                                        base + (2.0 * std::f64::consts::PI * i as f64) / count as f64
                                    };
                                    let r = RING_RADII[ring_idx];
                                    let nx = cx + r * angle.cos() - RING_NODE_W / 2.0;
                                    let ny = cy + r * angle.sin() - RING_NODE_H / 2.0;

                                    let eco_id = eco.id.clone();
                                    let eco_name = eco.name.clone();
                                    let diff_class = ring_difficulty_class(ring_idx);

                                    let is_active_dest = eco_id == *dest_id.as_deref().unwrap_or("");

                                    let node_class = if is_active_dest {
                                        format!("ring-node active {}", diff_class)
                                    } else {
                                        format!("ring-node {}", diff_class)
                                    };

                                    let style = format!(
                                        "translate: {:.1}px {:.1}px;",
                                        nx, ny
                                    );

                                    let on_click = on_card_click;
                                    let click_id = eco_id.clone();
                                    ring_node_views.push(view! {
                                        <div
                                            class=node_class
                                            style=style
                                            on:click=move |ev: web_sys::MouseEvent| {
                                                ev.stop_propagation();
                                                on_click.run(click_id.clone());
                                            }
                                        >
                                            <span class="ring-node-name">{eco_name}</span>
                                        </div>
                                    });
                                }
                            }

                            // ── SVG: ring strokes + labels only (bands are CSS radial-gradient) ──
                            let bounds = band_bounds();
                            let ring_lines: Vec<_> = RING_RADII.iter().enumerate().map(|(i, &r)| {
                                let label = RING_LABELS[i];
                                let color = RING_COLORS[i];
                                let (inner, outer) = bounds[i];
                                let label_r = (inner + outer) / 2.0;
                                let label_y = cy - label_r;
                                view! {
                                    <circle
                                        cx=cx
                                        cy=cy
                                        r=r
                                        class="ring-circle"
                                        style=format!("stroke: {}; stroke-opacity: 0.35;", color)
                                    />
                                    <text
                                        x=cx
                                        y=label_y
                                        class="ring-label"
                                        text-anchor="middle"
                                    >
                                        {label}
                                    </text>
                                }
                            }).collect();

                            view! {
                                <svg class="ring-svg" xmlns="http://www.w3.org/2000/svg">
                                    {ring_lines}
                                </svg>
                                <EcosystemCard
                                    ecosystem=src_clone
                                    is_selected=is_center_selected
                                    on_click=on_card_click
                                    position_override=(center_x, center_y)
                                />
                                {ring_node_views}
                            }.into_any()
                        } else {
                            view! { <p>"Source ecosystem not found"</p> }.into_any()
                        }
                    }
                }
            }}
        </div>
    }
}
