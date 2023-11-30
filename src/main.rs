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
        // 在整个世界中向前走
        self.x += 5;

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
    obstacle: Obstacle,
    score: i32,
    max_score: i32,
    is_new_record : bool,
}

impl State {
    fn new() -> Self {
        State { 
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
            max_score: 0,
            is_new_record: false,
         }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Flappy Bird");
        ctx.print_centered(8, &format!("Max Score: {}", self.max_score));
        ctx.print_centered(10, "Play (P)");
        ctx.print_centered(15, "Quit (Q)");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.re_start(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

    }

    // 结束页面，这个函数会一直执行
    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.print_centered(5, "Game Over");
        if self.score > self.max_score {
            // 记录最高分
            self.max_score = self.score;
            self.is_new_record = true;
        }

        // 由于多次执行这个函数，所以需要用变量做保存新纪录的状态
        if self.is_new_record {
            ctx.print_centered( 7, "New Record!");
        }

        ctx.print_centered( 9, &format!("Score: {}", self.score));
        ctx.print_centered(11, &format!("Max Score: {}", self.max_score));
        ctx.print_centered(13, "Play (P)");
        ctx.print_centered(15, "Quit (Q)");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.re_start();
                }
                VirtualKeyCode::Q => {
                    ctx.quitting = true;
                }
                _ => {}
            }
        }
    }

    // 游玩中页面
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
        ctx.print(0, 2, &format!("Score: {}", self.score));

        // 渲染障碍
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // 如果超出屏幕或者撞到障碍就结束游戏
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player)  {
            self.mode = GameMode::End;
        }
    }

    // 重新开始游戏，重置相关参数
    fn re_start(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
        self.is_new_record = false;
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.game_over(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

/*
|
|
|a

 <- gap_y

|
|b
|

a -> b = size
a -> gap_y = gap_z - (size / 2)
a -> gap_y = gap_z + (size / 2)
*/

struct Obstacle {
    // 世界空间
    x:i32, 
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        // 通过的空间位置为随机数，通过的大小最少为 2
        Obstacle { x, gap_y: random.range(10, 30) , size:  i32::max(2, 20 - score)}
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..(self.gap_y - half_size) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in (self.gap_y + half_size)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let is_match_x = self.x == player.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        is_match_x && (player_above_gap || player_below_gap)
    }

}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Bird Rush")
        .build()?;

    main_loop(ctx, State::new())
}
