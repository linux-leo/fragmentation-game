use macroquad::prelude::*;

struct Square {
    x: f32,
    y: f32,
    size: f32,
    color: Color,
}

impl Square {
    fn new(x: f32, y: f32, size: f32, color: Color) -> Square {
        Square { x, y, size, color }
    }

    fn draw(&self, font: &Font) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        draw_text_ex(
            "1",
            self.x + self.size/3.75,
            self.y - self.size/2.75,
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

    // Initialize the squares grid
    for i in 0..grid_size {
        for j in 0..grid_size {
            let x = start_x + j as f32 * (square_size + gap);
            let y = start_y + i as f32 * (square_size + gap);
            squares.push(Square::new(x, y, square_size, WHITE));
        }
    }

    loop {
        clear_background(BLACK);

        for square in squares.iter_mut() {
            square.draw(&font);
            if is_mouse_button_down(MouseButton::Left) && square.is_mouse_over() {
                square.color = RED;
            }
        }

        next_frame().await;
    }
}