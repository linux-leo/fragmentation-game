use macroquad::{prelude::*, ui};
use std::cmp;

enum GameState {
    StartMenu,
    Game,
}

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

    fn draw(&self, font: &Font, scalingfactor: &f32) {
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        if self.active {
        draw_text_ex(
            &self.number.to_string(),
            self.x + self.size*0.25,
            self.y + self.size*0.8,
            TextParams {
                font: Some(font),
                font_size: ((30.0 / 355.0) * scalingfactor * 0.8) as u16,
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
    set_fullscreen(true);
    let fontdir = include_bytes!("../res/NotoSansMono-Medium.ttf");
    let font = load_ttf_font_from_bytes(fontdir).unwrap();
    let mut scalingfactor = 0.0;
    let mut squares = Vec::new();
    let mut counter: u8 = 0;
    let mut game_state = GameState::StartMenu;
    loop {
        clear_background(BLACK);
        match game_state {
            GameState::StartMenu => {
                let skin = {
                    let label_style = ui::root_ui()
                        .style_builder()
                        .font(fontdir)
                        .unwrap()
                        .text_color(Color::from_rgba(180, 180, 100, 255))
                        .font_size(25)
                        .build();
            
                    let button_style = ui::root_ui()
                        .style_builder()
                        .background(
                            Image::from_file_with_format(
                                include_bytes!("../res/button_background.png"),
                                None,
                            )
                            .unwrap(),
                        )
                        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
                        .background_hovered(
                            Image::from_file_with_format(
                                include_bytes!("../res/button_hovered_background.png"),
                                None,
                            )
                            .unwrap(),
                        )
                        .background_clicked(
                            Image::from_file_with_format(
                                include_bytes!("../res/button_clicked_background.png"),
                                None,
                            )
                            .unwrap(),
                        )
                        .font(fontdir)
                        .unwrap()
                        .text_color(Color::from_rgba(255, 255, 255, 255))
                        .text_color_hovered(Color::from_rgba(192, 192, 255, 255))
                        .text_color_clicked(Color::from_rgba(32, 24, 64, 255))
                        .font_size(40)
                        .build();
            
            
                    ui::Skin {
                        button_style,
                        label_style,
                        ..ui::root_ui().default_skin()
                    }
                };
                ui::root_ui().push_skin(&skin);
                let game_name = "Fragmentator";
                let button_text = "Play";
                let button_center = get_text_center(button_text, Option::None, 40, 1.0, 0.0);
                let title_center = get_text_center(game_name, Option::None, 25, 1.0, 0.0);
                let button_pos = vec2(
                    (screen_width()*0.95 / 2.0) - 10.0 - button_center.x,
                    screen_height() / 2.0,
                );
                let label_pos = vec2(
                    (screen_width()*0.95 / 2.0) - 10.0 - title_center.x,
                    button_pos.y - 100.0,
                );
                ui::root_ui().label(Some(label_pos), game_name);
                if ui::root_ui().button(Some(button_pos), button_text) {
                    game_state = GameState::Game;
                    scalingfactor = cmp::min(screen_width() as i32, screen_height() as i32) as f32;
                    let square_size = (30.0 / 360.0) * scalingfactor;
                    let gap = (5.0 / 360.0) * scalingfactor;
                    let grid_size = 10;
                    let tablesize = square_size*grid_size as f32 + gap*(grid_size - 1) as f32;
                    for i in 0..grid_size {
                        for j in 0..grid_size {
                            let x = ((screen_width() - tablesize) / 2.0) + j as f32 * (square_size + gap);
                            let y = ((screen_height() - tablesize) / 2.0) + i as f32 * (square_size + gap);
                            squares.push(Square::new(x, y, square_size, WHITE, false, 0));
                        }
                    }
                }
            }
            GameState::Game => {
                for square in squares.iter_mut() {
                    square.draw(&font, &scalingfactor);
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
            }
        }
        next_frame().await;
    }
}