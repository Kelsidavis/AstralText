use druid::widget::{Label, Flex, TextBox, Scroll, Button};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Clone, Data, Lens)]
struct EditorState {
    content: String,
}

fn main() {
    let main_window = WindowDesc::new(build_ui()).title("AstralText");
    let initial_state = EditorState {
        content: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<EditorState> {
    let text_editor = TextBox::multiline()
        .lens(EditorState::content)
        .expand();

    Flex::column()
        .with_child(Label::new("AstralText - Rust Text Editor"))
        .with_flex_child(Scroll::new(text_editor), 1.0)
        .with_child(Button::new("Open").on_click(|_ctx, data: &mut EditorState, _| {
            // Basic File Open (Windows-specific example)
            if let Ok(mut file) = File::open("example.txt") {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                data.content = content;
            }
        }))
        .with_child(Button::new("Save").on_click(|_ctx, data: &mut EditorState, _| {
            // Basic File Save (Windows-specific example)
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open("example.txt")
                .unwrap();
            file.write_all(data.content.as_bytes()).unwrap();
        }))
}
