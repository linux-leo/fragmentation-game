use macroquad::prelude::*;

struct Square {
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    active: bool,
    number: u8
}

impl Square {
    fn new(x: f32, y: f32, size: f32, color: Color, active: bool, number: u8) -> Square {
        Square { x, y, size, color, active, number }
    }

    fn draw(&self, font: &Font) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        if self.active {
        draw_text_ex(
            &self.number.to_string(),
            self.x + self.size*0.25,
            self.y + self.size*0.8,
            TextParams {
                font: Some(font),
                font_size: 24,
                font_scale: 1.0,
                font_scale_aspect: 1.0, // Added this parameter
                rotation: 0.0, // Added this parameter
                color: BLACK,
            },
        );
        }
    }

    fn is_mouse_over(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        mouse_x > self.x && mouse_x < self.x + self.size && mouse_y > self.y && mouse_y < self.y + self.size
    }
}


#[macroquad::main("Clickable Squares")]
async fn main() {
    let font = load_ttf_font("./res/NotoSansMono-Medium.ttf").await.unwrap();
    let mut squares = Vec::new();
    let square_size = 30.0;
    let gap = 5.0;
    let grid_size = 10;
    let start_x = 100.0;
    let start_y = 100.0;
    let mut counter: u8 = 0;

    // Initialize the squares grid
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = start_x + j as f32 * (square_size + gap);
            let y = start_y + i as f32 * (square_size + gap);
            squares.push(Square::new(x, y, square_size, WHITE, false, 0));
        }
    }

    loop {
        clear_background(BLACK);

        for square in squares.iter_mut() {
            square.draw(&font);
            if is_mouse_button_pressed(MouseButton::Left) && square.is_mouse_over() && square.active == false  {
                square.color = RED;
                square.active = true;
                square.number = counter;
                if counter < 9 {
                    counter += 1;
                } else {
                    counter = 0;
                }
            }
        }

        next_frame().await;
    }
}