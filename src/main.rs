#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::prelude::*;
use gpui::*;
use gpui_component::{button::*, input::*, *};
use std::fs;

const OFFSET: usize = 0xD280;

// Original bytes from MFatigue.exe at offset 0xD280
const ORIGINAL_BYTES: [u8; 48] = [
    0x68, 0x00, 0x00, 0x15, 0x01, 0x6A, 0x00, 0xFF, 0x15, 0x14, 0x21, 0x4D,
    0x00, 0x85, 0xC0, 0x74, 0x2F, 0x8D, 0x88, 0xF4, 0xFF, 0x14, 0x01, 0xC7,
    0x41, 0x08, 0xD8, 0x2D, 0x52, 0x00, 0xC7, 0x41, 0x04, 0x00, 0x00, 0x00,
    0x00, 0x89, 0x01, 0x89, 0x08, 0x2B, 0xC8, 0xC7, 0x40, 0x08, 0x00, 0x00,
];

// Example: After patching with multiplier x2 (0x01150000 * 2 = 0x022A0000):
// 0x68, 0x00, 0x00, 0x2A, 0x02, 0x6A, 0x00, 0xFF, 0x15, 0x14, 0x21, 0x4D,
// 0x00, 0x85, 0xC0, 0x74, 0x2F, 0x8D, 0x88, 0xF4, 0xFF, 0x14, 0x01, 0xC7,
// 0x41, 0x08, 0xD8, 0x2D, 0x52, 0x00, 0xC7, 0x41, 0x04, 0x00, 0x00, 0x00,
// 0x00, 0x89, 0x01, 0x89, 0x08, 0x2B, 0xC8, 0xC7, 0x40, 0x08, 0x00, 0x00,
// Note: Only bytes at index 1-4 are changed: [0x00, 0x00, 0x15, 0x01] -> [0x00, 0x00, 0x2A, 0x02]

fn create_patch(multiplier: u32) -> [u8; 48] {
    let mut bytes = ORIGINAL_BYTES;
    let size = (0x01150000u32 * multiplier).to_le_bytes();
    bytes[1..5].copy_from_slice(&size);
    bytes
}

fn apply_patch(path: &str, patch: &[u8]) -> std::io::Result<()> {
    let mut data = fs::read(path)?;
    data[OFFSET..OFFSET + patch.len()].copy_from_slice(patch);
    fs::write(path, data)
}

fn check_patch_status(path: &str) -> Option<u32> {
    let data = fs::read(path).ok()?;
    if data.len() <= OFFSET + 5 {
        return None;
    }
    let current_bytes = &data[OFFSET..OFFSET + 5];
    if current_bytes == &ORIGINAL_BYTES[0..5] {
        return Some(0);
    }
    let size = u32::from_le_bytes([current_bytes[1], current_bytes[2], current_bytes[3], current_bytes[4]]);
    let multiplier = size / 0x01150000;
    if size == multiplier * 0x01150000 {
        Some(multiplier)
    } else {
        None
    }
}

pub struct MFatiguePatcher {
    file_path_input: Entity<InputState>,
    selected_multiplier: u32,
    status: SharedString,
    patch_info: SharedString,
    focus_handle: FocusHandle,
}

impl MFatiguePatcher {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            file_path_input: cx.new(|cx| InputState::new(window, cx).placeholder("Path to MFatigue.exe")),
            selected_multiplier: 2,
            status: "".into(),
            patch_info: "".into(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn update_patch_info(&mut self, cx: &mut Context<Self>) {
        let file_path = self.file_path_input.read(cx).value();
        self.patch_info = match check_patch_status(&file_path) {
            Some(0) => "Original (not patched)".into(),
            Some(m) => format!("Already patched ({}x)", m).into(),
            None => "".into(),
        };
    }

    fn check_file(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.update_patch_info(cx);
        cx.notify();
    }

    fn set_multiplier(&mut self, multiplier: u32, cx: &mut Context<Self>) {
        self.selected_multiplier = multiplier;
        cx.notify();
    }

    fn patch(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        let file_path = self.file_path_input.read(cx).value();
        
        self.status = match apply_patch(&file_path, &create_patch(self.selected_multiplier)) {
            Ok(_) => format!("Successfully patched ({}x)", self.selected_multiplier).into(),
            Err(e) => format!("Error: {}", e).into(),
        };
        self.update_patch_info(cx);
        cx.notify();
    }

    fn reset(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        let file_path = self.file_path_input.read(cx).value();
        
        self.status = match apply_patch(&file_path, &ORIGINAL_BYTES) {
            Ok(_) => "Successfully reset".into(),
            Err(e) => format!("Error: {}", e).into(),
        };
        self.update_patch_info(cx);
        cx.notify();
    }

    fn clear_focus(&mut self, _: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
        cx.notify();
    }
}

impl Render for MFatiguePatcher {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let multipliers = vec![2, 4, 8, 16, 32];
        
        div()
            .track_focus(&self.focus_handle)
            .v_flex()
            .size_full()
            .child(
                div()
                    .flex_1()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::clear_focus))
            )
            .child(
                div()
                    .v_flex()
                    .gap_4()
                    .items_center()
                    .p_4()
                    .child(
                        div()
                            .v_flex()
                            .gap_3()
                            .w(px(500.0))
                            .child(
                                div()
                                    .v_flex()
                                    .gap_1()
                                    .child("EXE-File:")
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .child(Input::new(&self.file_path_input).flex_1())
                                            .child(
                                                Button::new("check")
                                                    .label("check")
                                                    .on_click(cx.listener(Self::check_file)),
                                            ),
                                    )
                                    .when(!self.patch_info.is_empty(), |d| {
                                        d.child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(0x888888))
                                                .child(self.patch_info.clone())
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .v_flex()
                                    .gap_1()
                                    .child("Multiplier:")
                                    .child(
                                        div()
                                            .flex()
                                            .gap_2()
                                            .children(multipliers.iter().map(|&m| {
                                                Button::new(("mult", m))
                                                    .label(format!("{}x", m))
                                                    .when(m == self.selected_multiplier, |b| b.primary())
                                                    .on_click(cx.listener(move |view, _, _, cx| {
                                                        view.set_multiplier(m, cx);
                                                    }))
                                            })),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        Button::new("patch")
                                            .primary()
                                            .label("Patch")
                                            .on_click(cx.listener(Self::patch)),
                                    )
                                    .child(
                                        Button::new("reset")
                                            .label("Reset")
                                            .on_click(cx.listener(Self::reset)),
                                    ),
                            )
                            .when(!self.status.is_empty(), |d| {
                                d.child(
                                    div()
                                        .p_2()
                                        .bg(rgb(0x333333))
                                        .rounded(px(4.0))
                                        .child(self.status.clone())
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::clear_focus))
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(100.0), px(100.0)),
                        size: size(px(600.0), px(350.0)),
                    })),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Metal Fatigue Patcher".into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| MFatiguePatcher::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
