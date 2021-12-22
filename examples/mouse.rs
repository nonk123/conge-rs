use conge_rs::prelude::*;

struct Mouse {
    last_click: Option<MouseClick>,
    last_scroll: Option<ScrollDirection>,
}

impl App for Mouse {
    fn init(&mut self, context: &mut Context) {
        context.set_title("ConGE-rs mouse test");
    }

    fn draw(&mut self, _delta: f64, console: &mut Console) {
        let click_text = match &self.last_click {
            None => "Click anywhere on the screen!".to_string(),
            Some(click) => format!(
                "You pressed the {} mouse button at ({}, {})",
                match click.button() {
                    MouseButton::Left => "left",
                    MouseButton::Middle => "middle",
                    MouseButton::Right => "right",
                },
                click.pos().0,
                click.pos().1,
            ),
        };

        let scroll_text = match &self.last_scroll {
            None => "Scroll up or down!",
            Some(ScrollDirection::Up) => "You scrolled up",
            Some(ScrollDirection::Down) => "You scrolled down",
        };

        // Draw text in the center of the screen.
        let (width, height) = console.dimensions();

        let cx = (width - 1) / 2;
        let cy = (height - 1) / 2;

        console.text(cx - click_text.len() as i16 / 2, cy, click_text);
        console.text(cx - scroll_text.len() as i16 / 2, cy + 1, scroll_text);
    }

    fn on_click(&mut self, click: MouseClick, _context: &mut Context) {
        self.last_click = Some(click);
    }

    fn on_scroll(&mut self, direction: ScrollDirection, _context: &mut Context) {
        self.last_scroll = Some(direction);
    }
}

fn main() {
    run_app(
        Mouse {
            last_click: None,
            last_scroll: None,
        },
        60,
    );
}
