use engine::Game;
use engine::Color;
use engine::Kind;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::path;

struct MainState {
    game: Game,
    history: Vec<(i32, i32)>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            game: Game::new(),
            history: Vec::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.history.len() == 2 {
            let possible_moves = self.game.possible_moves(self.history[0]);
            if possible_moves.contains(&self.history[1]) {
                self.game.move_piece(self.history[0], self.history[1], Kind::None);
                // Promotion
                if self.history[1].0 == 7 || self.history[1].0 == 0 {
                    if self.game.get_board()[self.history[0].0 as usize][self.history[0].1 as usize]
                        == (Kind::Pawn, Color::White)
                        || self.game.get_board()[self.history[0].0 as usize]
                            [self.history[0].1 as usize]
                            == (Kind::Pawn, Color::Black)
                    {
                        self.game.move_piece(
                            self.history[0],
                            self.history[1],
                            Kind::Queen,
                        );
                    }
                }
            }

            //Castling
            if self.game.small_castling_available() && self.history[1].1 == 6 {
                self.game.small_castling();
            }
            if self.game.large_castling_available() && self.history[1].1 == 2 {
                self.game.large_castling();
            }
            self.history = Vec::new();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        let w_w: f32 = 800.0;
        let w_h: f32 = 600.0;
        let mut s_x: f32 = 0.0;
        let mut s_y: f32 = 0.0;
        let mut color: graphics::Color;
        let dark_color = graphics::Color::from_rgb(145, 108, 58);
        let light_color = graphics::Color::from_rgb(238, 238, 210);
        let warning_color = graphics::Color::from_rgba(231, 76, 60, 250);
        for i in 0..8 {
            for j in 0..8 {
                if i % 2 == 0 {
                    if j % 2 == 0 {
                        color = light_color;
                    } else {
                        color = dark_color;
                    }
                } else {
                    if j % 2 == 0 {
                        color = dark_color;
                    } else {
                        color = light_color;
                    }
                }
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, w_w / 8.0, w_h / 8.0),
                    color,
                )?;
                graphics::draw(ctx, &rectangle, (na::Point2::new(s_x, s_y),))?;
                let piece = self.game.get_board()[7 - i][j].clone();
                if piece.0 != Kind::None {
                    let piece_color = match piece.1 {
                        Color::White => "white",
                        Color::Black => "black",
                        _ => "",
                    };
                    let path = "/icons/".to_string() + piece_color + "_" + &piece.0.name() + ".png";
                    let image = graphics::Image::new(ctx, path).unwrap();
                    graphics::draw(ctx, &image, (na::Point2::new(s_x + 17.0, s_y + 10.0),))?;
                }
                if self.history.len() == 1 {
                    if self
                        .game
                        .possible_moves(self.history[0])
                        .contains(&(7 - i as i32, j as i32))
                    {
                        let circle_color = warning_color;
                        let cirlce = graphics::Mesh::new_circle(
                            ctx,
                            graphics::DrawMode::fill(),
                            na::Point2::new(0.0, 0.0),
                            30.0,
                            2.0,
                            circle_color,
                        )?;
                        graphics::draw(ctx, &cirlce, (na::Point2::new(s_x + 50.0, s_y + 37.0),))?;
                    }
                }
                s_x += w_w / 8.0;
            }
            s_x = 0.0;
            s_y += w_h / 8.0;
        }

        if self.game.checkmate() {
            graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
            let winner;
            if self.game.turn() % 2 == 0 {
                winner = "White is the winner!";
            } else {
                winner = "Black is the winner!";
            }
            let mut text = graphics::Text::new(winner);
            text.set_font(graphics::Font::default(), graphics::Scale::uniform(60.0));
            graphics::draw(ctx, &text, (na::Point2::new(150.0, 250.0), warning_color))?;
        }
        if self.game.check() && !self.game.checkmate() {
            let mut text = graphics::Text::new("Checkmate!");
            text.set_font(graphics::Font::default(), graphics::Scale::uniform(100.0));
            graphics::draw(ctx, &text, (na::Point2::new(150.0, 250.0), warning_color))?;
        }

        if self.game.stalemate() {
            graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
            let mut text = graphics::Text::new("Draw!");
            text.set_font(graphics::Font::default(), graphics::Scale::uniform(100.0));
            graphics::draw(ctx, &text, (na::Point2::new(150.0, 250.0), warning_color))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == ggez::input::mouse::MouseButton::Left {
            let x = (x / 100.0).floor() as i32;
            let y = 7 - (y / 75.0).floor() as i32;
            self.history.push((y, x));
            println!("{:?}", self.history)
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./gui/resources");
    let cb = ggez::ContextBuilder::new("Chess", "Hisham").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
