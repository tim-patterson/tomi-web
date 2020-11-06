mod canvas;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, HtmlAudioElement, CanvasRenderingContext2d};
use crate::canvas::Canvas;

#[wasm_bindgen]
pub struct Game {
    canvas: Canvas,
    assets: Assets,
    state: State
}

enum State {
    Waiting,
    Running
}

pub struct Assets {
    sprt_tomi: HtmlImageElement,
    snd_step: HtmlAudioElement,
    snd_background: HtmlAudioElement
}

#[wasm_bindgen]
impl Game {
    /// Creates a new Game object, fetches and waits on resources before returning
    pub async fn new(canvas_context: CanvasRenderingContext2d) -> Result<Game, JsValue> {
        let mut canvas = Canvas::new(canvas_context);
        canvas.text("Loading");

        let mut game = Game {
            canvas,
            assets: Game::load_assests().await?,
            state: State::Waiting
        };

        // Due to annoying websites browsers made it a rule that you had to
        // interact with a website before we can start to play sounds etc.
        game.canvas.clear();
        game.canvas.text("Press any key to start");

        Ok(game)
    }

    async fn load_assests() -> Result<Assets, JsValue> {
        let sprt_tomi = load_image("sprites/tomi_stationary.png");
        let snd_step = load_sound("sounds/step.mp3");
        let snd_background = load_sound("sounds/background.mp4");
        Ok(Assets {
            sprt_tomi: sprt_tomi.await?,
            snd_step: snd_step.await?,
            snd_background: snd_background.await?
        })
    }

    /// A single tick of the game
    #[wasm_bindgen]
    pub fn tick(&mut self, pressed_keys: &[i16]) {
        match self.state {
            State::Waiting => {
                if !pressed_keys.is_empty() {
                    self.assets.snd_background.set_loop(true);
                    self.assets.snd_background.play().ok();
                    self.state = State::Running;
                }
            }
            State::Running => {
                if !pressed_keys.is_empty() {
                    self.assets.snd_step.play().ok();
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        if let State::Running = self.state {
            self.canvas.clear();
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