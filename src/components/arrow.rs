use leptos::prelude::*;

use crate::theme;

pub const CARD_WIDTH: f64 = 180.0;
pub const CARD_HEIGHT: f64 = 130.0;

pub fn card_center(pos: &[f64; 2]) -> (f64, f64) {
    (pos[0] + CARD_WIDTH / 2.0, pos[1] + CARD_HEIGHT / 2.0)
}

#[component]
pub fn Arrow(
    source_pos: [f64; 2],
    end_x: f64,
    end_y: f64,
) -> impl IntoView {
    let (sx, sy) = card_center(&source_pos);
    let marker_id = "arrowhead";

    view! {
        <svg class="canvas-svg" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <marker
                    id=marker_id
                    markerWidth="10"
                    markerHeight="7"
                    refX="9"
                    refY="3.5"
                    orient="auto"
                >
                    <polygon points="0 0, 10 3.5, 0 7" class="arrow-head" fill=theme::CYAN />
                </marker>
            </defs>
            <line x1=sx y1=sy x2=end_x y2=end_y class="arrow-line-glow" />
            <line
                x1=sx
                y1=sy
                x2=end_x
                y2=end_y
                class="arrow-line"
                marker-end=format!("url(#{})", marker_id)
            />
        </svg>
    }
}

#[component]
pub fn FixedArrow(
    source_pos: [f64; 2],
    dest_pos: [f64; 2],
) -> impl IntoView {
    let (sx, sy) = card_center(&source_pos);
    let (dx, dy) = card_center(&dest_pos);
    let marker_id = "arrowhead-fixed";

    view! {
        <svg class="canvas-svg" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <marker
                    id=marker_id
                    markerWidth="10"
                    markerHeight="7"
                    refX="9"
                    refY="3.5"
                    orient="auto"
                >
                    <polygon points="0 0, 10 3.5, 0 7" class="arrow-head" fill=theme::CYAN />
                </marker>
            </defs>
            <line x1=sx y1=sy x2=dx y2=dy class="arrow-line-glow" />
            <line
                x1=sx
                y1=sy
                x2=dx
                y2=dy
                class="arrow-line"
                marker-end=format!("url(#{})", marker_id)
            />
        </svg>
    }
}
