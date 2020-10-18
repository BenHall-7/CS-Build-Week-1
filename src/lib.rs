use wasm_bindgen::{prelude::*, JsValue, JsCast};
use conway::Game;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct GameOfLife {
    inner: Game,
    width: usize,
    height: usize,
    block_size: usize,
}

#[wasm_bindgen]
impl GameOfLife {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, block_size: usize) -> GameOfLife {
        GameOfLife {
            inner: Game::new(width, height, false),
            width,
            height,
            block_size,
        }
    }

    pub fn step(&mut self) {
        self.inner.step()
    }

    pub fn draw(&self, canvas: &HtmlCanvasElement, alive_color: &str, dead_color: &str) -> Result<(), JsValue> {
        let width = self.width;
        let height = self.height;
        let size = self.block_size;
        canvas.set_width((self.width * self.block_size) as u32);
        canvas.set_height((self.height * self.block_size) as u32);
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        for row in 0..height {
            for col in 0..width {
                if self.inner.front()[[row as usize, col as usize]] != 0 {
                    context.set_fill_style(&JsValue::from(alive_color));
                } else {
                    context.set_fill_style(&JsValue::from(dead_color));
                }
                context.fill_rect((row * size) as f64, (col * size) as f64, size as f64, size as f64);
            }
        }
        Ok(())
    }
}
