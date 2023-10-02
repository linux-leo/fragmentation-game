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

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
    }

    fn is_mouse_over(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        mouse_x > self.x && mouse_x < self.x + self.size && mouse_y > self.y && mouse_y < self.y + self.size
    }
}


#[macroquad::main("Clickable Squares")]
async fn main() {
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
            square.draw();
            if is_mouse_button_down(MouseButton::Left) && square.is_mouse_over() {
                square.color = RED;
            }
        }

        next_frame().await;
    }
}