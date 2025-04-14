use bracket_lib::prelude::*;

const SCREEN_HEIGHT: i32 = 50;
const PADDLE_HEIGHT: i32 = 20;
const PADDLE_ONE_X: i32 = 10;
const PADDLE_TWO_X: i32 = 70;
const PADDLE_BOTH_Y: i32 = (SCREEN_HEIGHT - PADDLE_HEIGHT)/2;
const PADDLE_STEP: i32 = 3;
const PAD: i32 = 1;

fn draw_paddle(ctx: &mut BTerm, pos: Point, height: i32) {
    for i in 0..height {
        ctx.set(pos.x, pos.y + i, WHITE, BLACK, to_cp437('|'));
    }
}

fn draw_ball(ctx: &mut BTerm, pos: &Point) {
    ctx.set(pos.x, pos.y, WHITE, BLACK, to_cp437('O'));
}

#[derive(Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Vector {
    from: Point,
    to: Point,
}

impl Vector {
    fn from_points(a: Point, b: Point) -> Vector {
        Vector {
            from: Point { x: a.x, y: a.y },
            to: Point { x: b.x, y: b.y },
        }
    }
}

#[derive(Clone)]
enum PlayerType {
    Human,
    AI,
}

#[derive(Clone)]
struct Player {
    paddle_pos: Point,
    player_type: PlayerType,
    score: u8,
}

impl Player {
    fn new(pos: &Point, p_type: PlayerType) -> Self {
        Player {
            paddle_pos: pos.clone(),
            player_type: p_type,
            score: 0,
        }
    }

    fn move_up(&mut self) {
        if self.paddle_pos.y > 0 {
            self.paddle_pos.y -= PADDLE_STEP;
        }
    }

    fn move_down(&mut self) {
        if self.paddle_pos.y < SCREEN_HEIGHT - PADDLE_HEIGHT {
            self.paddle_pos.y += PADDLE_STEP;
        }
    }

    fn scores(&mut self) {
        self.score += 1;
    }
}

enum Players {
    PlayerOne,
    PlayerTwo
}


#[derive(Clone)]
struct Ball {
    pos: Point,
    dir: Vector,
}

impl Ball {
    pub fn new() -> Self {
        const INITIAL_POS: Point = Point { x: PADDLE_ONE_X + PAD, y: PAD };

        let to = Point { x: 12, y: 2 };
        Self {
            pos: INITIAL_POS,
            dir: Vector::from_points(INITIAL_POS, to),
        }
    }

    pub fn bounce(&mut self, player_one: &mut Player, player_two: &mut Player) -> Option<Players> {
        let dx = self.dir.to.x - self.dir.from.x;
        let dy = self.dir.to.y - self.dir.from.y;

        let next_x = self.pos.x + dx + PAD;
        let next_y = self.pos.y + dy + PAD;

        if next_x <= PADDLE_ONE_X + PAD {
            let paddle = player_one.paddle_pos.clone();
            if paddle.y <= next_y && next_y <= (paddle.y + PADDLE_HEIGHT) {
                let temp = self.dir.to.x;
                self.dir.to.x = self.dir.from.x;
                self.dir.from.x = temp;
            } else {
                return Some(Players::PlayerTwo);
            }
        } else if PADDLE_TWO_X - PAD <= next_x {
            let paddle = player_two.paddle_pos.clone();
            if paddle.y <= next_y && next_y <= (paddle.y + PADDLE_HEIGHT) {
                let temp = self.dir.to.x;
                self.dir.to.x = self.dir.from.x;
                self.dir.from.x = temp;
            } else {
                return Some(Players::PlayerOne);
            }
        }

        self.pos.x += dx;

        if next_y <= PAD || SCREEN_HEIGHT - PAD <= next_y {
            self.dir.to.y = -self.dir.to.y;
        }

        self.pos.y += self.dir.to.y;
        None
    }
}

#[derive(Clone, PartialEq, Eq)]
enum GameModes {
    Menu,
    Playing,
    Stopped,
}

#[derive(Clone)]
struct State {
    current_mode: GameModes,
    player_one: Player,
    player_two: Player,
    ball: Ball,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.current_mode {
            GameModes::Menu => self.show_main_menu(ctx),
            GameModes::Playing => self.play(ctx),
            GameModes::Stopped => self.stop(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        let player_one_pos = Point { x: PADDLE_ONE_X, y: PADDLE_BOTH_Y };
        let player_two_pos = Point { x: PADDLE_TWO_X, y: PADDLE_BOTH_Y };

        Self {
            current_mode: GameModes::Menu,
            player_one: Player::new(&player_one_pos, PlayerType::Human),
            player_two: Player::new(&player_two_pos, PlayerType::AI),
            ball: Ball::new(),
        }
    }

    fn show_main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Pong !!");
        ctx.print_centered(8, "Press (S) to start or");
        ctx.print_centered(9, "      (Q) to quit.");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.current_mode = GameModes::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        if self.current_mode == GameModes::Playing {
            ctx.print_centered(
                1,
                format!("{} - {}", self.player_one.score, self.player_two.score),
            );

            match ctx.key {
                Some(VirtualKeyCode::Up) | Some(VirtualKeyCode::W) => self.player_one.move_up(),
                Some(VirtualKeyCode::Down) | Some(VirtualKeyCode::S) => self.player_one.move_down(),
                Some(VirtualKeyCode::Escape) => self.stop(ctx),
                _ => {}
            }
            draw_paddle(ctx, self.player_one.paddle_pos.clone(), PADDLE_HEIGHT);

            // AI logic for player_two
            if let PlayerType::AI = self.player_two.player_type {
                if self.ball.pos.y < self.player_two.paddle_pos.y {
                    self.player_two.move_up();
                } else if self.ball.pos.y > self.player_two.paddle_pos.y {
                    self.player_two.move_down();
                }
            }

            draw_paddle(ctx, self.player_two.paddle_pos.clone(), PADDLE_HEIGHT);

            let scored = self.ball.bounce(&mut self.player_one, &mut self.player_two);
            if let Some(player) = scored {
                match player {
                    Players::PlayerOne => self.player_one.scores(),
                    Players::PlayerTwo => self.player_two.scores(),
                }
            
                self.reset_after_score();
            }

            draw_ball(ctx, &self.ball.pos);
        }

        // println!("Ball => x:{}, y:{}", self.ball.pos.x, self.ball.pos.y)
    }

    fn stop(&mut self, ctx: &mut BTerm) {
        self.current_mode = GameModes::Stopped;
    
        if self.player_one.score > self.player_two.score {
            ctx.print_centered(1, "The winner is: PlayerOne");
        } else if self.player_two.score > self.player_one.score {
            ctx.print_centered(1, "The winner is: PlayerTwo");
        } else {
            ctx.print_centered(1, "We don't have a winner!");
        }
    }
    
    fn reset_after_score(&mut self) {
        self.ball = Ball::new();

        self.player_one.paddle_pos = Point { x: PADDLE_ONE_X, y: PADDLE_BOTH_Y };
        self.player_two.paddle_pos = Point { x: PADDLE_TWO_X, y: PADDLE_BOTH_Y };
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Pong | O |")
        .build()?;
    main_loop(context.clone(), State::new())
}
