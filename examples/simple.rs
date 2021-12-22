use conge_rs::prelude::*;

struct Simple;

impl App for Simple {
    fn init(&mut self, context: &mut Context) {
        context.set_title("Simple ConGE-rs test");
    }

    fn draw(&mut self, _delta: f64, console: &mut Console) {
        let text = "Hello, world!";

        let (width, height) = console.dimensions();

        // Draw text in the center of the screen.
        console.text(
            (width - 1) / 2 - text.len() as i16 / 2,
            (height - 1) / 2,
            text,
        );
    }
}

fn main() {
    run_app(Simple, 60);
}
