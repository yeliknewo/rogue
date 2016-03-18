use dorp::{init, Game, Window, WindowArgs};

pub fn main() {
    let manager = init();

    let mut window = Window::new(WindowArgs::Borderless("Iso".to_string()));

    let resolution = window.get_resolution_vec2();

    let thread_count = 8;

    let mut game = Game::<IsoData>::new(manager.clone(), thread_count, resolution);

    game.run(&mut window);
}

pub struct IsoData;
