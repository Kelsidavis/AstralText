use druid::widget::{Button, Flex, Scroll, SizedBox, TextBox};
use druid::{
    AppLauncher, Command, Data, DelegateCtx, Env, Lens, Selector, Target, Widget, WidgetExt,
    WindowDesc,
};
use rfd::FileDialog;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const NEW_FILE: Selector = Selector::new("astraltext.new-file");
const OPEN_FILE: Selector = Selector::new("astraltext.open-file");
const SAVE_FILE: Selector = Selector::new("astraltext.save-file");
const SAVE_FILE_AS: Selector = Selector::new("astraltext.save-file-as");

#[derive(Clone, Data, Lens, Debug)]
struct EditorState {
    content: String,
    file_path: Option<String>,
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("AstralText - Rust Text Editor | v1.0 | Developed by Kelsi Davis | https://geekastro.dev")
        .window_size((800.0, 600.0));

    let initial_state = EditorState {
        content: String::new(),
        file_path: None,
    };

    AppLauncher::with_window(main_window)
        .delegate(AppDelegate)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<EditorState> {
    let text_editor = TextBox::multiline()
        .with_placeholder("Start typing here...")
        .lens(EditorState::content)
        .fix_width(780.0)
        .fix_height(500.0);

    let text_area = SizedBox::new(Scroll::new(text_editor))
        .fix_width(780.0)
        .fix_height(500.0);

    let status_bar = Flex::row()
        .with_child(Button::new("New").on_click(|ctx, _, _| ctx.submit_command(NEW_FILE)))
        .with_spacer(10.0)
        .with_child(Button::new("Open").on_click(|ctx, _, _| ctx.submit_command(OPEN_FILE)))
        .with_spacer(10.0)
        .with_child(Button::new("Save").on_click(|ctx, _, _| ctx.submit_command(SAVE_FILE)))
        .with_spacer(10.0)
        .with_child(Button::new("Save As").on_click(|ctx, _, _| ctx.submit_command(SAVE_FILE_AS)))
        .padding((8.0, 5.0))
        .fix_height(50.0);

    Flex::column()
        .with_child(text_area)
        .with_spacer(5.0)
        .with_child(status_bar)
}

struct AppDelegate;

impl druid::AppDelegate<EditorState> for AppDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut EditorState,
        _env: &Env,
    ) -> druid::Handled {
        match cmd {
            c if c.is(NEW_FILE) => {
                data.content.clear();
                data.file_path = None;
            }
            c if c.is(OPEN_FILE) => self.open_file(data),
            c if c.is(SAVE_FILE) => self.save_file(data),
            c if c.is(SAVE_FILE_AS) => self.save_file_as(data),
            _ => return druid::Handled::No,
        }
        druid::Handled::Yes
    }
}

impl AppDelegate {
    fn open_file(&self, data: &mut EditorState) {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Ok(mut file) = File::open(&path) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    data.content = content;
                    data.file_path = Some(path.display().to_string());
                }
            }
        }
    }

    fn save_file(&self, data: &mut EditorState) {
        if let Some(path) = &data.file_path {
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
            {
                file.write_all(data.content.as_bytes()).ok();
            }
        } else {
            self.save_file_as(data);
        }
    }

    fn save_file_as(&self, data: &mut EditorState) {
        if let Some(path) = FileDialog::new().save_file() {
            if let Ok(mut file) = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
            {
                file.write_all(data.content.as_bytes()).ok();
                data.file_path = Some(path.display().to_string());
            }
        }
    }
}
