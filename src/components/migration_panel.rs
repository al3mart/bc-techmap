use leptos::prelude::*;

use crate::data::ecosystem::Ecosystem;
use crate::data::scoring::compute_migration;

#[component]
pub fn MigrationPanel(
    source: Ecosystem,
    dest: Ecosystem,
    on_close: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let source_name = source.name.clone();
    let dest_name = dest.name.clone();
    let source_short = source.short.clone();
    let dest_short = dest.short.clone();

    let src_has_modes = source.deploy_modes.is_some() && source.deployment_options.len() > 1;
    let dst_has_modes = dest.deploy_modes.is_some() && dest.deployment_options.len() > 1;

    let src_mode_options: Vec<String> = if src_has_modes {
        source.deployment_options.clone()
    } else {
        vec![]
    };
    let dst_mode_options: Vec<String> = if dst_has_modes {
        dest.deployment_options.clone()
    } else {
        vec![]
    };

    let (src_mode, set_src_mode) = signal(
        if src_has_modes {
            src_mode_options.first().cloned()
        } else {
            None
        },
    );
    let (dst_mode, set_dst_mode) = signal(
        if dst_has_modes {
            dst_mode_options.first().cloned()
        } else {
            None
        },
    );

    let src_for_report = source.clone();
    let dst_for_report = dest.clone();

    let report = Signal::derive(move || {
        compute_migration(
            &src_for_report,
            &dst_for_report,
            src_mode.get().as_deref(),
            dst_mode.get().as_deref(),
        )
    });

    view! {
        <div class="migration-panel">
            <button class="panel-close" on:click=move |_| on_close()>"[ESC]"</button>

            <div class="panel-header">"Migration Analysis"</div>

            <div class="panel-route">
                {source_name}
                <span class="panel-route-arrow">" → "</span>
                {dest_name}
            </div>

            {if src_has_modes {
                let options = src_mode_options.clone();
                Some(view! {
                    <div class="mode-toggle">
                        <span class="mode-label">{source_short}" deploys as"</span>
                        <div class="mode-buttons">
                            {options.into_iter().map(|mode| {
                                let mode_val = mode.clone();
                                let mode_click = mode.clone();
                                let mode_display = mode.clone();
                                view! {
                                    <button
                                        class=move || {
                                            if src_mode.get().as_deref() == Some(&mode_val) {
                                                "mode-btn active"
                                            } else {
                                                "mode-btn"
                                            }
                                        }
                                        on:click=move |_| set_src_mode.set(Some(mode_click.clone()))
                                    >{mode_display}</button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                })
            } else {
                None
            }}

            {if dst_has_modes {
                let options = dst_mode_options.clone();
                Some(view! {
                    <div class="mode-toggle">
                        <span class="mode-label">{dest_short}" deploys as"</span>
                        <div class="mode-buttons">
                            {options.into_iter().map(|mode| {
                                let mode_val = mode.clone();
                                let mode_click = mode.clone();
                                let mode_display = mode.clone();
                                view! {
                                    <button
                                        class=move || {
                                            if dst_mode.get().as_deref() == Some(&mode_val) {
                                                "mode-btn active"
                                            } else {
                                                "mode-btn"
                                            }
                                        }
                                        on:click=move |_| set_dst_mode.set(Some(mode_click.clone()))
                                    >{mode_display}</button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                })
            } else {
                None
            }}

            {move || {
                let r = report.get();
                let overall_pct = (r.overall * 100.0) as u32;
                let score_display = format!("{:.1}/5", r.overall * 5.0);
                let difficulty = r.difficulty_label.clone();
                let has_positives = !r.positives.is_empty();

                view! {
                    <div>
                        <div class="difficulty-score">
                            <div class="difficulty-label">"Overall Difficulty"</div>
                            <div class="difficulty-bar">
                                <div
                                    class="difficulty-fill"
                                    style=format!("width: {}%", overall_pct)
                                ></div>
                            </div>
                            <div class="difficulty-value">{score_display}</div>
                            <div class="difficulty-text">{difficulty}</div>
                        </div>

                        <div class="dimension-list">
                            {r.dimensions.iter().map(|dim| {
                                let pct = (dim.score * 100.0) as u32;
                                let name = dim.name.clone();
                                let label = dim.label.clone();
                                view! {
                                    <div class="dimension-item">
                                        <div class="dimension-header">
                                            <span class="dimension-name">{name}</span>
                                            <span class="dimension-score">{label}</span>
                                        </div>
                                        <div class="dimension-bar">
                                            <div
                                                class="dimension-fill"
                                                style=format!("width: {}%", pct)
                                            ></div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        <div class="challenges">
                            <div class="challenges-title">"Key Challenges"</div>
                            {r.challenges.iter().map(|c| {
                                let text = c.clone();
                                view! {
                                    <div class="challenge-item">{text}</div>
                                }
                            }).collect::<Vec<_>>()}

                            {if has_positives {
                                Some(view! {
                                    <div class="positives-title">"Advantages"</div>
                                    {r.positives.iter().map(|p| {
                                        let text = p.clone();
                                        view! {
                                            <div class="positive-item">{text}</div>
                                        }
                                    }).collect::<Vec<_>>()}
                                })
                            } else {
                                None
                            }}
                        </div>
                    </div>
                }
            }}
        </div>
    }
}
