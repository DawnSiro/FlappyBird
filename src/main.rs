use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State { mode: GameMode::Menu }
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
        self.mode = GameMode::End;
    }

    fn re_start(&mut self) {
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
