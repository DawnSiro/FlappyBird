use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
// 帧单位时间
const FRAME_DURATION : f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32,  y: i32) -> Self {
        Player { x: x, y: y, velocity: 0.0 }
    }

    // 渲染
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    // 重力移动，向下速度为正
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.x < 0 {
            self.x = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }

}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State { 
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
         }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome ");
        ctx.print_centered(9, "Play (P)");
        ctx.print_centered(12, "Quit (Q)");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.re_start(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Game Over ");
        ctx.print_centered(9, "Play (P)");
        ctx.print_centered(12, "Quit (Q)");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.re_start(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // 刷新背景
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;

        // 过了一帧的时间，更新时间并执行一次重力影响
        if self.frame_time >= FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // 这里不能放帧更新里，保证随时按空格随时有效
        if let Some(key) = ctx.key {
            match key {
                // 如果按下空格 flap 一下
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to Flap");

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn re_start(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Bird Rush")
        .build()?;

    main_loop(ctx, State::new())
}
