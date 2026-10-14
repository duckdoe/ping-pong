use macroquad::prelude::*;

#[derive(Debug)]
struct Ball {
    pub pos: Vec2,
    size: f32,
}

struct Pad {
    pub pos: Vec2,
    thickness: f32,
    length: f32,
}

impl Ball {
    pub fn new (size: f32, pos: Vec2) -> Self {
        Self {size, pos}
    }
    pub fn draw (&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, WHITE);
    }
    pub fn update_x(&mut self, dx: f32) {
        self.pos.x += dx;
    }
    pub fn update_y(&mut self, dy: f32) {
        self.pos.y += dy;
    }
}

impl Pad {
    pub fn new(pos: Vec2, thickness: f32, length: f32) -> Self {
        Self { pos, thickness, length }
    }
    pub fn update_y(&mut self, dy: f32) {
        self.pos.y += dy;
    }
    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.thickness, self.length, WHITE);
    }
}

const BALL_SPEED: f32 = 10.0;
const BALL_SIZE: f32 = 15.0;

#[macroquad::main("Ping Pong")]
async fn main() {
    let ball_pos = Vec2::new(screen_width()/2.0, screen_height()/2.0);

    let mut dx = BALL_SPEED*0.75;
    let mut dy = BALL_SPEED*0.75;
    let mut ball = Ball::new(BALL_SIZE, ball_pos);
    let mut pad1 = Pad::new(Vec2 { x: 30.0, y: screen_height()/2.0 - (60.0*0.5) }, BALL_SIZE, 100.0);
    let mut pad2 = Pad::new(Vec2 { x: screen_width() - 50.0, y: screen_height()/2.0 - (60.0*0.5)}, BALL_SIZE, 100.0);
    let mut score_1 = 0;
    let mut score_2 = 0;
    let segments = 30;

    loop {
        clear_background(BLACK);
        pad1.draw();
        pad2.draw();
        ball.draw();

        let w = screen_width();
        let h = screen_height();

        draw_text(format!("{score_1}"), 6.0, 6.0 * BALL_SIZE,100.0,WHITE);
        draw_text(format!("{score_2}"), w - 6.0 * BALL_SIZE, 6.0 * BALL_SIZE,100.0,WHITE);

        if ball.pos.x < 0.0 {
            score_2 += 1;
            dx = -dx;
        } else if ball.pos.x + ball.size >= w{
            score_1 += 1;
            dx = -dx;
        }
        if ball.pos.y < 0.0 || (ball.pos.y + ball.size) >= h {
            dy = -dy;
        }

        if is_key_down(KeyCode::S) {
            pad1.update_y(BALL_SPEED);
            if pad1.pos.y > h - pad1.length{
                pad1.pos.y = h - pad1.length;
            }
        } else if is_key_down(KeyCode::W) && pad1.pos.y > 0.0{
            pad1.update_y(-BALL_SPEED);
            if pad1.pos.y < 0.0 {
                pad1.pos.y = 0.0;
            }
        }

        if is_key_down(KeyCode::Down) {
            pad2.update_y(BALL_SPEED);
            if pad2.pos.y > h - pad2.length{
                pad2.pos.y = h - pad2.length;
            }
        } else if is_key_down(KeyCode::Up) && pad2.pos.y > 0.0 {
            pad2.update_y(-BALL_SPEED);
            if pad2.pos.y < 0.0 {
                pad2.pos.y = 0.0;
            }
        }


        // TODO: Handle Ball Collisions
        if ball.pos.y >= pad1.pos.y
            && ball.pos.y < pad1.pos.y + pad1.length
            && ball.pos.x < pad1.pos.x + pad1.thickness
        {
            dx = -dx;
        }

        if ball.pos.y >= pad2.pos.y
            && ball.pos.y < pad2.pos.y + pad2.length
            && ball.pos.x + ball.size >= pad2.pos.x
        {
            dx = -dx;
        }

        /* Draws dottend line */
        let seg_length = h/segments as f32;
        for n in 0..segments  {
            draw_line(w/2.0, seg_length*(n as f32)+5.0,w/2.0, seg_length*(n+1) as f32, 3.0, WHITE);
        }

        ball.update_x(dx);
        ball.update_y(dy);
        next_frame().await;
    }
}
