mod canvas;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, HtmlAudioElement, CanvasRenderingContext2d};
use crate::canvas::{Canvas, SpriteSheet, Alignment};

/// The top level game
#[wasm_bindgen]
pub struct Game {
    canvas: Canvas,
    assets: Assets,
    status: Status,
    tomi: GameObject
}

/// Enum for if the game has started or not yet
enum Status {
    Waiting,
    Running
}

/// All the assets used for the game
struct Assets {
    sprt_tomi: SpriteSheet,
    snd_step: HtmlAudioElement,
    snd_background: HtmlAudioElement
}

/// An object in the game
struct GameObject {
    x: u32,
    y: u32,
    direction: Direction,
    speed: u32,
    sprite: SpriteSheet
}


enum Direction {
    Stationary,
    Left,
    Right,
    Up,
    Down,
}

#[wasm_bindgen]
impl Game {
    /// Creates a new Game object, fetches and waits on resources before returning
    pub async fn new(canvas_context: CanvasRenderingContext2d) -> Result<Game, JsValue> {
        let mut canvas = Canvas::new(canvas_context);
        canvas.text("Loading");
        let assets = Game::load_assests().await?;

        // Due to annoying websites browsers made it a rule that you had to
        // interact with a website before we can start to play sounds etc.
        canvas.clear();
        canvas.text("Press any key to start");
        canvas.draw_sprite_with_alignment(&assets.sprt_tomi, 0, 300, 180, Alignment::Centered);

        let tomi = GameObject { x: 300, y: 300, direction: Direction::Stationary, speed: 5, sprite: assets.sprt_tomi.clone()};

        Ok(Game {
            canvas,
            assets,
            status: Status::Waiting,
            tomi
        })
    }

    async fn load_assests() -> Result<Assets, JsValue> {
        let sprt_tomi = load_image("sprites/tomi_walking.png");
        let snd_step = load_sound("sounds/step.mp3");
        let snd_background = load_sound("sounds/background.mp4");
        Ok(Assets {
            sprt_tomi: SpriteSheet::new_grid(sprt_tomi.await?,4, 1),
            snd_step: snd_step.await?,
            snd_background: snd_background.await?
        })
    }

    /// A single tick of the game(60Hz)
    #[wasm_bindgen]
    pub fn tick(&mut self, pressed_keys: &[i16]) {
        match self.status {
            Status::Waiting => {
                if !pressed_keys.is_empty() {
                    self.assets.snd_background.set_loop(true);
                    self.assets.snd_background.play().ok();
                    self.status = Status::Running;
                }
            }
            Status::Running => {
                self.tomi.direction = if pressed_keys.contains(&37) {
                    Direction::Left
                } else if pressed_keys.contains(&39) {
                    Direction::Right
                } else if pressed_keys.contains(&38) {
                    Direction::Up
                } else if pressed_keys.contains(&40) {
                    Direction::Down
                } else {
                    Direction::Stationary
                };

                // Update actual x/y's for tomi
                match self.tomi.direction {
                    Direction::Left => self.tomi.x -= self.tomi.speed,
                    Direction::Right => self.tomi.x += self.tomi.speed,
                    Direction::Up => self.tomi.y -= self.tomi.speed,
                    Direction::Down => self.tomi.y += self.tomi.speed,
                    Direction::Stationary => {}
                }
            }
        }
    }

    /// Render out to canvas(hopefully 60FPS!)
    #[wasm_bindgen]
    pub fn render(&mut self) {
        if let Status::Running = self.status {
            self.canvas.clear();
            self.canvas.draw_sprite_with_alignment(&self.tomi.sprite,
                2, self.tomi.x, self.tomi.y, Alignment::TopLeft
            )
        }
    }
}

// FFI's

async fn load_sound(src: &str) -> Result<HtmlAudioElement, JsValue> {
    _load_sound(src).await.map(HtmlAudioElement::from)
}

async fn load_image(src: &str) -> Result<HtmlImageElement, JsValue> {
    _load_image(src).await.map(HtmlImageElement::from)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name=load_image)]
    async fn _load_image(src: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name=load_sound)]
    async fn _load_sound(src: &str) -> Result<JsValue, JsValue>;
}