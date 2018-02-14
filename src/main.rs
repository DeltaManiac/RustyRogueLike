extern crate tcod;
use tcod::Console;
use tcod::console::*;
use tcod::RootConsole;
use tcod::colors::{self, Color};
use tcod::BackgroundFlag;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
struct Object {
    x: i32,
    y: i32,
    c: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, c: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            c: c,
            color: color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }


    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.c, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

fn handle_keys(con: &mut Root, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = con.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = con.is_fullscreen();
            con.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        _ => {}
    }
    false
}

fn main() {
    let mut con = RootConsole::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("LTCOD")
        .init();

    let mut con_back = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let enemy = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    tcod::system::set_fps(LIMIT_FPS);
    let mut objects = [player, enemy];


    while !con.window_closed() {
        for object in &objects {
            object.draw(&mut con_back);
        }
        blit(&mut con_back,
             (0, 0),
             (SCREEN_WIDTH, SCREEN_HEIGHT),
             &mut con,
             (0, 0),
             1.0,
             1.0);
        con.flush();
        for object in &objects {
            object.clear(&mut con_back);
        }
        let exit = handle_keys(&mut con, &mut objects[0]);
        // let player = &mut objects[0];
        if exit {
            break;
        }
    }
}
