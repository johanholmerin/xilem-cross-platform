// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use xilem::{
    view::{button, button_any_pointer, checkbox, flex, label, prose, textbox},
    AnyWidgetView, Axis, Color, EventLoopBuilder, TextAlignment, WidgetView, Xilem,
};
const LOREM: &str = r"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi cursus mi sed euismod euismod. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nullam placerat efficitur tellus at semper. Morbi ac risus magna. Donec ut cursus ex. Etiam quis posuere tellus. Mauris posuere dui et turpis mollis, vitae luctus tellus consectetur. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur eu facilisis nisl.

Phasellus in viverra dolor, vitae facilisis est. Maecenas malesuada massa vel ultricies feugiat. Vivamus venenatis et nibh nec pharetra. Phasellus vestibulum elit enim, nec scelerisque orci faucibus id. Vivamus consequat purus sit amet orci egestas, non iaculis massa porttitor. Vestibulum ut eros leo. In fermentum convallis magna in finibus. Donec justo leo, maximus ac laoreet id, volutpat ut elit. Mauris sed leo non neque laoreet faucibus. Aliquam orci arcu, faucibus in molestie eget, ornare non dui. Donec volutpat nulla in fringilla elementum. Aliquam vitae ante egestas ligula tempus vestibulum sit amet sed ante. ";

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> {
    // here's some logic, deriving state for the view from our state
    let count = data.count;
    let button_label = if count == 1 {
        "clicked 1 time".to_string()
    } else {
        format!("clicked {count} times")
    };

    // The actual UI Code starts here

    let axis = if data.active {
        Axis::Horizontal
    } else {
        Axis::Vertical
    };

    let sequence = (0..count)
        .map(|x| button(format!("+{x}"), move |data: &mut AppData| data.count += x))
        .collect::<Vec<_>>();
    flex((
        flex((
            label("Label")
                .brush(Color::REBECCA_PURPLE)
                .alignment(TextAlignment::Start),
            // TODO masonry doesn't allow setting disabled manually anymore?
            // label("Disabled label").disabled(),
        ))
        .direction(Axis::Horizontal),
        flex(textbox(
            data.textbox_contents.clone(),
            |data: &mut AppData, new_value| {
                data.textbox_contents = new_value;
            },
        ))
        .direction(Axis::Horizontal),
        prose(LOREM).alignment(TextAlignment::Middle),
        button_any_pointer(button_label, |data: &mut AppData, button| match button {
            masonry::PointerButton::None => {},
            masonry::PointerButton::Primary => data.count += 1,
            masonry::PointerButton::Secondary => data.count -= 1,
            masonry::PointerButton::Auxiliary => data.count *= 2,
            _ => (),
        }),
        checkbox("Check me", data.active, |data: &mut AppData, checked| {
            data.active = checked;
        }),
        toggleable(data),
        button("Decrement", |data: &mut AppData| data.count -= 1),
        button("Reset", |data: &mut AppData| data.count = 0),
        flex(sequence).direction(axis),
    ))
}

fn toggleable(data: &mut AppData) -> impl WidgetView<AppData> {
    let inner_view: Box<AnyWidgetView<_>> = if data.active {
        Box::new(
            flex((
                button("Deactivate", |data: &mut AppData| {
                    data.active = false;
                }),
                button("Unlimited Power", |data: &mut AppData| {
                    data.count = -1_000_000;
                }),
            ))
            .direction(Axis::Horizontal),
        )
    } else {
        Box::new(button("Activate", |data: &mut AppData| data.active = true))
    };
    inner_view
}

struct AppData {
    textbox_contents: String,
    count: i32,
    active: bool,
}

/// Entry point for the app
pub fn run(event_loop: EventLoopBuilder) {
    let data = AppData {
        count: 0,
        textbox_contents: "Not quite a placeholder".into(),
        active: false,
    };

    let app = Xilem::new(data, app_logic);
    app.run_windowed(event_loop, "First Example".into())
        .unwrap();
}
