use web_sys::{ CanvasRenderingContext2d, HtmlImageElement};
use wasm_bindgen::__rt::std::rc::Rc;

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

    pub fn draw_sprite_with_alignment(&mut self, sprite_sheet: &SpriteSheet, sprite_idx: u32, x: u32, y: u32, alignment: Alignment) {
        let grid_x = sprite_idx % sprite_sheet.rc.x_count;
        let grid_y = sprite_idx / sprite_sheet.rc.x_count;

        let sx = grid_x * sprite_sheet.rc.x_size;
        let sy = grid_y * sprite_sheet.rc.y_size;

        let (left, top) = match alignment {
            Alignment::TopLeft => (x, y),
            Alignment::Centered => (x - sprite_sheet.rc.x_size / 2, y - sprite_sheet.rc.y_size / 2)
        };

        self.inner.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &sprite_sheet.rc.inner,
            sx as f64,
            sy as f64,
            sprite_sheet.rc.x_size as f64,
            sprite_sheet.rc.y_size as f64,
            left as f64,
            top as f64,
            sprite_sheet.rc.x_size as f64,
            sprite_sheet.rc.y_size as f64,
        ).unwrap();
    }
}

/// Where we consider the x/y 0,0 to be when rendering to the
/// canvas
pub enum Alignment {
    TopLeft,
    Centered
}





/// A wrapper around an image, an image may contain multiple sprites
#[derive(Clone)]
pub struct SpriteSheet {
    rc: Rc<SpriteSheetInner>
}

struct SpriteSheetInner {
    inner: HtmlImageElement,
    x_count: u32,
    y_count: u32,
    /// Size in pixels (source) of a single sprite
    x_size: u32,
    /// Size in pixels (source) of a single sprite
    y_size: u32,
}


impl SpriteSheet {
    pub fn new_single(inner: HtmlImageElement) -> Self {
        let x_size = inner.natural_width();
        let y_size = inner.natural_height();
        SpriteSheet {
            rc: Rc::new(SpriteSheetInner {
                inner,
                x_count: 1,
                y_count: 1,
                x_size,
                y_size
            })
        }
    }

    pub fn new_grid(inner: HtmlImageElement, x_count: u32, y_count: u32) -> Self {
        let x_total_size = inner.natural_width();
        let y_total_size = inner.natural_height();
        SpriteSheet {
            rc: Rc::new(SpriteSheetInner {
                inner,
                x_count,
                y_count,
                x_size: x_total_size / x_count,
                y_size: y_total_size / y_count
            })
        }
    }
}