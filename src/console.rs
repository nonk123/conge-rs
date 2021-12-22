use std::{collections::HashSet, ptr::null_mut};

use num_traits::FromPrimitive;
use winapi::{
    ctypes::c_void,
    shared::{minwindef::HIWORD, ntdef::HANDLE},
    um::{
        consoleapi::{GetNumberOfConsoleInputEvents, ReadConsoleInputW, SetConsoleMode},
        fileapi::WriteFile,
        processenv::GetStdHandle,
        winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
        wincon::{
            GetConsoleScreenBufferInfo, SetConsoleCursorInfo, SetConsoleCursorPosition,
            SetConsoleTextAttribute, CONSOLE_CURSOR_INFO, ENABLE_EXTENDED_FLAGS,
            ENABLE_MOUSE_INPUT,
        },
        wincontypes::{
            COORD, FROM_LEFT_1ST_BUTTON_PRESSED, FROM_LEFT_2ND_BUTTON_PRESSED, INPUT_RECORD,
            KEY_EVENT, MOUSE_EVENT, MOUSE_WHEELED, RIGHTMOST_BUTTON_PRESSED,
        },
    },
};

use crate::{
    app::{App, Context},
    buffer::Buffer,
    input::{Key, KeyCode, MouseButton, ScrollDirection},
    pixel::{Color, Pixel},
};

pub struct Console {
    input: HANDLE,
    output: HANDLE,
    front_buffer: Buffer,
    back_buffer: Buffer,
    dimensions: (i16, i16),
    cursor_position: (i16, i16),
    text_color: u16,
}

impl Console {
    pub fn new() -> Self {
        unsafe {
            Self {
                input: GetStdHandle(STD_INPUT_HANDLE),
                output: GetStdHandle(STD_OUTPUT_HANDLE),
                front_buffer: Buffer::new(),
                back_buffer: Buffer::new(),
                dimensions: (0, 0),
                cursor_position: (-2, -2),
                text_color: 255,
            }
        }
    }

    pub fn pixel(&mut self, x: i16, y: i16, pixel: Pixel) {
        self.front_buffer.set(x, y, pixel);
    }

    pub fn text(&mut self, x: i16, y: i16, text: impl Into<String>) {
        self.text_ext(x, y, text, Color::White, Color::Black);
    }

    pub fn text_ext(&mut self, mut x: i16, y: i16, text: impl Into<String>, fg: Color, bg: Color) {
        for c in text.into().chars() {
            if x == self.dimensions.0 {
                break;
            }

            self.pixel(x, y, Pixel::new(c, fg.clone(), bg.clone()));

            x += 1;
        }
    }

    pub(crate) fn init(&mut self) {
        unsafe {
            SetConsoleMode(self.input, ENABLE_MOUSE_INPUT | ENABLE_EXTENDED_FLAGS);
        }
    }

    pub(crate) fn resize(&mut self, width: i16, height: i16) {
        self.dimensions = (width, height);
        self.front_buffer.resize(width, height);
        self.back_buffer.resize(width, height);
    }

    pub(crate) fn reset_back_buffer(&mut self) {
        self.back_buffer.fill('\0'.into());
    }

    pub(crate) fn move_cursor(&mut self, x: i16, y: i16) {
        if self.cursor_position == (x, y) {
            return;
        }

        unsafe {
            SetConsoleCursorPosition(self.output, COORD { X: x, Y: y });
        }

        self.cursor_position = (x, y);
    }

    pub(crate) fn set_text_color(&mut self, fg: Color, bg: Color) {
        let color = bg as u16 * 16 + fg as u16;

        if self.text_color != color {
            unsafe {
                SetConsoleTextAttribute(self.output, color);
            }

            self.text_color = color;
        }
    }

    pub(crate) fn disable_cursor(&self) {
        let mut info: CONSOLE_CURSOR_INFO = Default::default();

        info.dwSize = 100;
        info.bVisible = 0;

        unsafe {
            SetConsoleCursorInfo(self.output, &info);
        }
    }

    pub(crate) fn draw(&mut self) {
        self.move_cursor(0, 0);

        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                let front = self.front_buffer.get(x, y).clone();
                let back = self.back_buffer.get(x, y).clone();

                if front == back {
                    continue;
                }

                if self.cursor_position.0 != x - 1 || self.cursor_position.1 != y {
                    self.move_cursor(x, y);
                }

                self.set_text_color(front.fg, front.bg);

                unsafe {
                    WriteFile(
                        self.output,
                        vec![front.character].as_ptr() as *const c_void,
                        1,
                        null_mut(),
                        null_mut(),
                    );
                }
            }
        }

        self.move_cursor(0, 0);
        self.set_text_color(Color::White, Color::Black);

        self.back_buffer = self.front_buffer.clone();
        self.front_buffer.clear();
    }

    pub(crate) fn handle_input(&mut self, context: &mut Context, app: &mut impl App) {
        let mut count = 0;

        unsafe {
            GetNumberOfConsoleInputEvents(self.input, &mut count);
        }

        if count == 0 {
            return;
        }

        let buffer_size: u32 = 64;

        let mut events: Vec<INPUT_RECORD> = Vec::new();
        let mut count = 0;

        events.resize(buffer_size as usize, Default::default());

        unsafe {
            ReadConsoleInputW(self.input, events.as_mut_ptr(), buffer_size, &mut count);
        }

        events.resize(count as usize, Default::default());

        for record in events {
            if record.EventType == KEY_EVENT {
                let event = unsafe { record.Event.KeyEvent() };
                let key_code: Option<KeyCode> = FromPrimitive::from_u16(event.wVirtualScanCode);

                // Skip uncovered keys.
                if key_code.is_none() {
                    continue;
                }

                let key_code = key_code.unwrap().clone();
                let character = char::from_u32(unsafe { *event.uChar.UnicodeChar() as u32 });

                let key = Key::new(key_code.clone(), character.unwrap());

                if event.bKeyDown == 1 {
                    context.hold_key(&key_code);
                    app.key_down(key, context);
                } else {
                    context.release_key(&key_code);
                    app.key_up(key, context);
                }
            } else if record.EventType == MOUSE_EVENT {
                let event = unsafe { record.Event.MouseEvent() };

                if event.dwEventFlags & MOUSE_WHEELED != 0 {
                    let sign_bit = HIWORD(event.dwButtonState).to_be_bytes()[0] & 1;

                    let direction = if sign_bit == 1 {
                        ScrollDirection::Down
                    } else {
                        ScrollDirection::Up
                    };

                    app.on_scroll(direction, context);
                } else {
                    let mouse_pos = event.dwMousePosition;
                    context.update_mouse_pos(mouse_pos.X, mouse_pos.Y);

                    let mouse_pos = context.mouse_pos();

                    let mut buttons = HashSet::new();

                    if event.dwButtonState & FROM_LEFT_1ST_BUTTON_PRESSED != 0 {
                        buttons.insert(MouseButton::Left);
                    }

                    if event.dwButtonState & FROM_LEFT_2ND_BUTTON_PRESSED != 0 {
                        buttons.insert(MouseButton::Middle);
                    }

                    if event.dwButtonState & RIGHTMOST_BUTTON_PRESSED != 0 {
                        buttons.insert(MouseButton::Right);
                    }

                    context.set_buttons(buttons.clone(), mouse_pos, app);
                }
            }
        }
    }

    pub fn dimensions(&self) -> (i16, i16) {
        self.dimensions
    }

    pub(crate) fn request_size(&self) -> (i16, i16) {
        let mut size = Default::default();

        unsafe {
            GetConsoleScreenBufferInfo(self.output, &mut size);
        }

        (size.dwSize.X as i16, size.dwSize.Y as i16)
    }
}
