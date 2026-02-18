use leptos::prelude::*;

use crate::components::canvas::Canvas;
use crate::components::migration_panel::MigrationPanel;
use crate::components::ring_view::RingView;
use crate::data::ecosystem::{load_ecosystems, Ecosystem};

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Grid,
    Ring,
}

#[derive(Clone, PartialEq)]
pub enum AppState {
    Idle,
    SourceSelected { source_id: String },
    ShowResults {
        source_id: String,
        dest_id: String,
    },
}

#[component]
pub fn App() -> impl IntoView {
    let ecosystems: Vec<Ecosystem> = load_ecosystems();
    let ecosystems_stored = StoredValue::new(ecosystems.clone());

    let (state, set_state) = signal(AppState::Idle);
    let (mouse_pos, set_mouse_pos) = signal((0.0_f64, 0.0_f64));
    let (view_mode, set_view_mode) = signal(ViewMode::Grid);

    let on_card_click = move |eco_id: String| {
        let current = state.get();
        match current {
            AppState::Idle => {
                set_state.set(AppState::SourceSelected {
                    source_id: eco_id,
                });
            }
            AppState::SourceSelected { ref source_id } => {
                if *source_id == eco_id {
                    set_state.set(AppState::Idle);
                } else {
                    set_state.set(AppState::ShowResults {
                        source_id: source_id.clone(),
                        dest_id: eco_id,
                    });
                }
            }
            AppState::ShowResults {
                ref source_id,
                ref dest_id,
            } => {
                // Only Ring mode needs special dest-switching behavior
                let mode = view_mode.get_untracked();
                if mode == ViewMode::Ring {
                    if eco_id == *source_id {
                        set_state.set(AppState::Idle);
                    } else if eco_id == *dest_id {
                        set_state.set(AppState::SourceSelected {
                            source_id: source_id.clone(),
                        });
                    } else {
                        set_state.set(AppState::ShowResults {
                            source_id: source_id.clone(),
                            dest_id: eco_id,
                        });
                    }
                } else {
                    set_state.set(AppState::SourceSelected {
                        source_id: eco_id,
                    });
                }
            }
        }
    };

    let on_canvas_click = move || {
        let current = state.get();
        if matches!(current, AppState::ShowResults { .. }) {
            set_state.set(AppState::Idle);
        }
    };

    let on_mouse_move = move |x: f64, y: f64| {
        set_mouse_pos.set((x, y));
    };

    let on_close_panel = move || {
        set_state.set(AppState::Idle);
    };

    let hint_text = move || {
        let mode = view_mode.get();
        match state.get() {
            AppState::Idle => {
                if mode == ViewMode::Ring {
                    "Click an ecosystem to center it".to_string()
                } else {
                    "Click an ecosystem to start".to_string()
                }
            }
            AppState::SourceSelected { ref source_id } => {
                if mode == ViewMode::Ring {
                    format!("{} centered — click a ring node to compare", source_id.to_uppercase())
                } else {
                    format!("Select destination for {}", source_id.to_uppercase())
                }
            }
            AppState::ShowResults { .. } => "ESC to close".to_string(),
        }
    };

    let show_panel = move || matches!(state.get(), AppState::ShowResults { .. });

    let panel_data = move || {
        if let AppState::ShowResults {
            ref source_id,
            ref dest_id,
        } = state.get()
        {
            ecosystems_stored.with_value(|ecos| {
                let src = ecos.iter().find(|e| e.id == *source_id).cloned();
                let dst = ecos.iter().find(|e| e.id == *dest_id).cloned();
                src.zip(dst)
            })
        } else {
            None
        }
    };

    let grid_btn_class = move || {
        if view_mode.get() == ViewMode::Grid {
            "view-btn active"
        } else {
            "view-btn"
        }
    };

    let ring_btn_class = move || {
        if view_mode.get() == ViewMode::Ring {
            "view-btn active"
        } else {
            "view-btn"
        }
    };

    let ecosystems_grid = ecosystems.clone();
    let ecosystems_ring = ecosystems.clone();

    view! {
        <div
            on:keydown=move |ev: web_sys::KeyboardEvent| {
                if ev.key() == "Escape" {
                    set_state.set(AppState::Idle);
                }
            }
            tabindex="0"
            style="outline: none; width: 100%; height: 100%;"
        >
            <div class="header">
                <span class="header-title">"BLOCKCHAIN TECH MAP"</span>
                <div class="view-switcher">
                    <button
                        class=grid_btn_class
                        on:click=move |_| set_view_mode.set(ViewMode::Grid)
                    >
                        "GRID"
                    </button>
                    <button
                        class=ring_btn_class
                        on:click=move |_| set_view_mode.set(ViewMode::Ring)
                    >
                        "RING"
                    </button>
                </div>
                <span class="header-hint">{hint_text}</span>
            </div>

            {move || {
                if view_mode.get() == ViewMode::Grid {
                    view! {
                        <Canvas
                            ecosystems=ecosystems_grid.clone()
                            state=state
                            mouse_pos=mouse_pos
                            on_card_click=on_card_click
                            on_canvas_click=on_canvas_click
                            on_mouse_move=on_mouse_move
                        />
                    }.into_any()
                } else {
                    view! {
                        <RingView
                            ecosystems=ecosystems_ring.clone()
                            state=state
                            on_card_click=on_card_click
                            on_canvas_click=on_canvas_click
                        />
                    }.into_any()
                }
            }}

            {move || {
                if show_panel() {
                    panel_data().map(|(src, dst)| {
                        view! {
                            <MigrationPanel
                                source=src
                                dest=dst
                                on_close=on_close_panel
                            />
                        }
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
