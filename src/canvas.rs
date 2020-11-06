use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};

/// A wrapper around an html canvas.
pub struct Canvas {
    inner: CanvasRenderingContext2d,
    width: u32,
    height: u32
}

impl Canvas {
    pub fn new(inner: CanvasRenderingContext2d) -> Self {
        let canvas_element = inner.canvas().unwrap();
        // Set up basic drawing contexts
        inner.set_text_align("center");
        inner.set_font("50px Arial");
        inner.set_fill_style(&"white".into());

        Canvas {
            inner,
            width: canvas_element.width(),
            height: canvas_element.height()
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear_rect(0.0,0.0,self.width as f64, self.height as f64)
    }

    pub fn text(&mut self, text: &str) {
        self.inner.fill_text(text, self.width as f64/2.0, self.height as f64/2.0).unwrap();
    }

    pub fn draw_sprite(&mut self, sprite: &HtmlImageElement, x: u32, y: u32) {
        self.inner.draw_image_with_html_image_element(sprite, x as f64, y as f64).unwrap();
    }
}