use std::{
    collections::HashSet,
    thread::sleep,
    time::{Duration, Instant},
};

use winapi::um::wincon::SetConsoleTitleW;

use crate::prelude::*;

pub trait App {
    fn init(&mut self, context: &mut Context) {
        context.set_title("ConGE-rs app");
    }

    fn tick(&mut self, _time_step: f64, _context: &mut Context) {}

    fn draw(&mut self, delta: f64, console: &mut Console);

    fn key_down(&mut self, _key: Key, _context: &mut Context) {}

    fn key_up(&mut self, key: Key, context: &mut Context) {
        if key.code() == KeyCode::Esc {
            context.request_exit();
        }
    }

    fn button_down(&mut self, _button: MouseButton, _context: &mut Context) {}

    fn button_up(&mut self, _button: MouseButton, _context: &mut Context) {}

    fn on_click(&mut self, _click: MouseClick, _context: &mut Context) {}

    fn on_scroll(&mut self, _direction: ScrollDirection, _context: &mut Context) {}
}

pub struct Context {
    should_exit: bool,
    title: String,
    down_keys: HashSet<KeyCode>,
    down_buttons: HashSet<MouseButton>,
    mouse_pos: (i16, i16),
}

impl Context {
    pub fn request_exit(&mut self) {
        self.should_exit = true;
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Can be used every frame to check if a key is held down.
    ///
    /// For single presses, handle the `key_up` and `key_down` events.
    pub fn is_key_down(&self, key: &KeyCode) -> bool {
        self.down_keys.contains(key)
    }

    pub fn mouse_pos(&self) -> (i16, i16) {
        self.mouse_pos
    }

    pub fn is_button_down(&self, button: &MouseButton) -> bool {
        self.down_buttons.contains(button)
    }

    pub fn is_shift_down(&self) -> bool {
        self.down_keys.contains(&KeyCode::LeftShift)
    }

    pub fn is_ctrl_down(&self) -> bool {
        self.down_keys.contains(&KeyCode::LeftCtrl)
    }

    pub fn is_alt_down(&self) -> bool {
        self.down_keys.contains(&KeyCode::LeftAlt)
    }

    pub(crate) fn hold_key(&mut self, key: &KeyCode) {
        self.down_keys.insert(key.clone());
    }

    pub(crate) fn release_key(&mut self, key: &KeyCode) {
        self.down_keys.retain(|x| x != key);
    }

    pub(crate) fn set_buttons(
        &mut self,
        buttons: HashSet<MouseButton>,
        mouse_pos: (i16, i16),
        app: &mut impl App,
    ) {
        let new_down = buttons
            .difference(&self.down_buttons)
            .cloned()
            .collect::<Vec<MouseButton>>();

        let new_up = self
            .down_buttons
            .difference(&buttons)
            .cloned()
            .collect::<Vec<MouseButton>>();

        self.down_buttons = buttons;

        for button in new_down {
            app.button_down(button.clone(), self);
        }

        for button in new_up {
            app.button_up(button.clone(), self);

            let click = MouseClick::new(button, mouse_pos);
            app.on_click(click, self);
        }
    }

    pub(crate) unsafe fn update_console_title(&self) {
        let mut vec: Vec<u16> = Vec::new();
        vec.resize(self.title.len() + 1, 0);

        let mut idx = 0;

        for c in self.title.chars() {
            *vec.as_mut_ptr().add(idx) = c as u16;
            idx += 1;
        }

        SetConsoleTitleW(vec.as_ptr());
    }

    pub(crate) fn update_mouse_pos(&mut self, x: i16, y: i16) {
        self.mouse_pos = (x, y);
    }
}

pub fn run_app(mut app: impl App, fps: i32) {
    let mut context = Context {
        should_exit: false,
        title: "ConGE app".to_string(),
        down_keys: HashSet::new(),
        down_buttons: HashSet::new(),
        mouse_pos: (0, 0),
    };

    let mut console = Console::new();
    console.init();

    app.init(&mut context);

    let min_delta = 1.0 / fps as f64;
    let mut delta = min_delta;

    let mut prev_size = (-1, -1);

    loop {
        let now = Instant::now();

        let size = console.request_size();

        if size != prev_size {
            console.resize(size.0, size.1);
            console.reset_back_buffer();
            console.disable_cursor();
        }

        prev_size = size;

        console.handle_input(&mut context, &mut app);
        app.tick(min_delta, &mut context);

        if context.should_exit {
            break;
        }

        unsafe {
            context.update_console_title();
        }

        app.draw(delta, &mut console);

        console.draw();

        delta = now.elapsed().as_secs_f64();

        if delta < min_delta {
            sleep(Duration::from_secs_f64(min_delta - delta));
        }
    }
}
