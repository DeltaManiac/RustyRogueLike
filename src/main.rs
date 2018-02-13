extern crate tcod;
use tcod::Console;
use tcod::console::*;
use tcod::RootConsole;
use tcod::colors;
use tcod::BackgroundFlag;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT:i32 = 50;
const LIMIT_FPS:i32 = 20;

fn handle_keys(con: &mut Root, player_x: &mut i32, player_y:&mut i32) -> bool{
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = con.wait_for_keypress(true);

    match key {
        Key {code:Enter,alt:true,..}=>{
            let fullscreen = con.is_fullscreen();
            con.set_fullscreen(!fullscreen);
        }
        Key{code:Escape, ..} => return true,
        Key{code:Up, ..} => *player_y -= 1,
        Key{code:Down, ..} => *player_y += 1,
        Key{code:Left, ..} => *player_x -= 1,
        Key{code:Right, ..} => *player_x += 1,
        _ => {},
    }
    false
}

fn main() {
    let mut con = RootConsole::initializer()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("LTCOD")
        .init();

    let mut con_back = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH/2;
    let mut player_y = SCREEN_HEIGHT/2;

    while !con.window_closed() {
        con_back.set_default_foreground(colors::WHITE);
        con_back.put_char(player_x, player_y, ' ', BackgroundFlag::None);
        let exit = handle_keys(&mut con, &mut player_x, &mut player_y);
        con_back.put_char(player_x, player_y, '@', BackgroundFlag::None);
        blit(&mut con_back, (0,0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut con, (0,0),1.0,1.0);
        con.flush();
        if exit {
            break;
        }
    }
}
