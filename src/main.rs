use druid::widget::{Button, Controller, Flex, Label, Scroll, TextBox};
use druid::{
    AppLauncher, Data, Env, Event, EventCtx, Lens, LifeCycle, LifeCycleCtx, Widget, WidgetExt,
    WindowDesc,
};

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Clone, Data, Lens)]
struct EditorState {
    content: String,
}

fn main() {
    println!("🚀 AstralText is launching...");

    let main_window = WindowDesc::new(build_ui())
        .title("AstralText - Rust Text Editor")
        .window_size((600.0, 400.0));

    let initial_state = EditorState {
        content: String::new(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

// Custom Controller to force focus on the TextBox
struct FocusController;

impl<W: Widget<EditorState>> Controller<EditorState, W> for FocusController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut EditorState,
        env: &Env,
    ) {
        if let Event::WindowConnected = event {
            println!("✅ TextBox Window Connected - requesting focus");
            ctx.request_focus();
        }

        if let Event::MouseDown(_) = event {
            println!("✅ TextBox clicked - requesting focus");
            ctx.request_focus();
        }

        child.event(ctx, event, data, env);
    }
}

fn build_ui() -> impl Widget<EditorState> {
    println!("✅ UI is being built...");

    let text_editor = TextBox::multiline()
        .with_placeholder("Start typing here...")
        .lens(EditorState::content)
        .expand_width()
        .fix_height(300.0)
        .controller(FocusController);

    let open_button = Button::new("Open").on_click(|_ctx, data: &mut EditorState, _| {
        println!("📂 Open button clicked");
        if let Ok(mut file) = File::open("example.txt") {
            let mut content = String::new();
            if file.read_to_string(&mut content).is_ok() {
                data.content = content;
                println!("✅ File loaded successfully");
            } else {
                println!("❌ Failed to read file");
            }
        } else {
            println!("❌ Failed to open file");
        }
    });

    let save_button = Button::new("Save").on_click(|_ctx, data: &mut EditorState, _| {
        println!("💾 Save button clicked");
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .open("example.txt")
        {
            if file.write_all(data.content.as_bytes()).is_ok() {
                println!("✅ File saved successfully");
            } else {
                println!("❌ Failed to save file");
            }
        } else {
            println!("❌ Failed to create/open file for saving");
        }
    });

    Flex::column()
        .with_child(Label::new("AstralText - Rust Text Editor").center())
        .with_flex_child(Scroll::new(text_editor).vertical().expand(), 1.0)
        .with_child(
            Flex::row()
                .with_child(open_button.padding(5.0))
                .with_child(save_button.padding(5.0)),
        )
        .padding(10.0)
}
