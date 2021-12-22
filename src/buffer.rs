use crate::pixel::Pixel;

#[derive(Clone)]
pub struct Buffer {
    pixels: Vec<Vec<Pixel>>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { pixels: Vec::new() }
    }

    pub fn resize(&mut self, width: i16, height: i16) {
        let filler: Pixel = ' '.into();

        let filler_row = vec![filler.clone(); width as usize];
        self.pixels.resize(height as usize, filler_row);

        for row in &mut self.pixels {
            row.resize(width as usize, filler.clone());
        }
    }

    pub fn fill(&mut self, pixel: Pixel) {
        for row in &mut self.pixels {
            for column in row {
                *column = pixel.clone();
            }
        }
    }

    pub fn clear(&mut self) {
        self.fill(' '.into());
    }

    pub fn get(&mut self, x: i16, y: i16) -> &Pixel {
        &self.pixels[y as usize][x as usize]
    }

    pub fn set(&mut self, x: i16, y: i16, pixel: Pixel) {
        self.pixels[y as usize][x as usize] = pixel;
    }
}
