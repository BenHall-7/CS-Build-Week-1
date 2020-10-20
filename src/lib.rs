use wasm_bindgen::{prelude::*, JsValue, JsCast};
use js_sys::Array;
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

    pub fn set_on(&mut self, coordinates: &Array) {
        let coordinates = coordinates.iter().map(|c| {
            let pair = c.dyn_into::<Array>().unwrap();
            let y = pair.get(0).as_f64().unwrap() as usize;
            let x = pair.get(1).as_f64().unwrap() as usize;
            [y, x]
        });
        self.inner.set_on(coordinates);
    }

    pub fn set_off(&mut self, coordinates: &Array) {
        let coordinates = coordinates.iter().map(|c| {
            let pair = c.dyn_into::<Array>().unwrap();
            let y = pair.get(0).as_f64().unwrap() as usize;
            let x = pair.get(1).as_f64().unwrap() as usize;
            [y, x]
        });
        self.inner.set_off(coordinates);
    }

    pub fn invert(&mut self, coordinates: &Array) {
        let coordinates = coordinates.iter().map(|c| {
            let pair = c.dyn_into::<Array>().unwrap();
            let y = pair.get(0).as_f64().unwrap() as usize;
            let x = pair.get(1).as_f64().unwrap() as usize;
            [y, x]
        });
        self.inner.invert(coordinates);
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn step(&mut self) {
        self.inner.step()
    }

    pub fn draw(&self, canvas: &HtmlCanvasElement, alive_color: &str, dead_color: &str) {
        let width = self.width;
        let height = self.height;
        let size = self.block_size;
        canvas.set_width((width * size) as u32);
        canvas.set_height((height * size) as u32);
        let context = canvas
            .get_context("2d").unwrap().unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        context.set_fill_style(&JsValue::from(dead_color));
        context.fill_rect(0.0, 0.0, (width * size) as f64, (height * size) as f64);
        context.set_fill_style(&JsValue::from(alive_color));
        for row in 0..height {
            for col in 0..width {
                if self.inner.front()[[row as usize, col as usize]] != 0 {
                    context.fill_rect((col * size) as f64, (row * size) as f64, size as f64, size as f64);
                }
            }
        }
    }
}
