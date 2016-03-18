use std::sync::{RwLock, Arc};
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use glium::glutin::Event as WindowEvent;

use input::{Keyboard, Mouse, Display, KeyCode, ButtonState, MouseButton, Button};
use logic::{World, IDManager, EntityData};
use math::{Vec2};
use graphics::{Window, Transforms};

pub struct Game<T: EntityData<T>> {
    world: Arc<World<T>>,
    thread_pool: Pool,
    display: Arc<RwLock<Display>>,
    mouse: Arc<RwLock<Mouse>>,
    keyboard: Arc<RwLock<Keyboard>>,
    transforms: Arc<RwLock<Transforms>>,
    manager: Arc<RwLock<IDManager>>,
}

impl<T: EntityData<T>> Game<T> {
    pub fn new(manager: Arc<RwLock<IDManager>>, thread_count: u32, resolution: Vec2) -> Game<T> {
        let keyboard = Arc::new(RwLock::new(Keyboard::new()));
        let mouse = Arc::new(RwLock::new(Mouse::new()));
        let display = Arc::new(RwLock::new(Display::new(resolution)));
        let transforms = Arc::new(RwLock::new(Transforms::new()));
        Game {
            world: Arc::new(World::new(keyboard.clone(), mouse.clone(), display.clone(), transforms.clone())),
            thread_pool: Pool::new(thread_count),
            display: display,
            mouse: mouse,
            keyboard: keyboard,
            transforms: transforms,
            manager: manager,
        }
    }

    pub fn get_world(&self) -> Arc<World<T>> {
        self.world.clone()
    }

    fn pause(&mut self) {
        println!("Paused");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }

    fn update_keyboard(&mut self, tick_number: u64, key_code: KeyCode, element_state: ButtonState) {
        self.keyboard.write().expect("Unable to Write Keyboard in Update Keyboard in Game").set_key_state(key_code, Button::new(tick_number, element_state));
    }

    fn update_mouse_button(&mut self, tick_number: u64, mouse_button: MouseButton, element_state: ButtonState, ) {
        self.mouse.write().expect("Unable to Write Mouse in Update Mouse Button in Game").set_mouse_button(mouse_button, Button::new(tick_number, element_state));
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) {
        self.mouse.write().expect("Unable to Write Mouse in Update Mouse Pos in Game").set_mouse_position(Vec2::from([mouse_pos.0 as f32, mouse_pos.1 as f32]));
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) {
        self.display.write().expect("Unable to Write Display in Update Resolution in Game").set_resolution(Vec2::from([resolution.0 as f32, resolution.1 as f32]));
    }

    pub fn run(&mut self, window: &mut Window) {
        let tps: f64 = 60.0;
        let tps_s: f64 = 1.0 / tps;

        let mut last_time: f64 = precise_time_s();
        let mut delta_time: f64 = 0.0;

        let mut i: f64 = last_time;

        let mut frames: u64 = 0;
        let mut ticks: u64 = 0;

        let mut tick_number: u64 = 0;

        loop {
            let now = precise_time_s();
            delta_time += now - last_time;
            last_time = now;
            while delta_time > 0.0 {
                for event in window.poll_events(){
                    match event {
                        WindowEvent::Resized(width, height) => self.update_resolution((width, height)),
                        // WindowEvent::Moved(x, y) => {
                        //
                        // },
                        WindowEvent::Closed => return,
                        // WindowEvent::DroppedFile(path_buffer) => {
                        //
                        // },
                        // WindowEvent::ReceivedCharacter(character) => {
                        //
                        // },
                        WindowEvent::Focused(focused) => {
                            if focused {
                                self.resume();
                            } else {
                                self.pause();
                            }
                        },
                        WindowEvent::KeyboardInput(element_state, _, virtual_key_code) => match virtual_key_code {
                            Some(virtual_key_code) => self.update_keyboard(tick_number, virtual_key_code, element_state),
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => self.update_mouse_pos(pos),
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => self.update_mouse_button(tick_number, mouse_button, element_state),
                        // WindowEvent::Awakened => {
                        //
                        // },
                        // WindowEvent::Refresh => {
                        //
                        // },
                        // WindowEvent::Suspended(suspended) => {
                        //
                        // },
                        // WindowEvent::Touch(touch) => {
                        //
                        // },
                        _ => (),
                    }
                }
                self.tick(tps_s);
                delta_time -= tps_s;
                ticks += 1;
                tick_number += 1;
            }
            self.render(window);
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("{} {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self, window: &mut Window) {
        let world = self.world.clone();
        let entity_data = world.get_entity_data();
        let entity_data = entity_data.read().expect("Unable to Read Entity Data in Render in Game");
        for entry in entity_data.iter() {
            let entity = entry.1;
            let mut entity = entity.write().expect("Unable to Write Entity in Render in Game");
            entity.render(window);
        }
        let mut frame = window.frame();
        for entry in entity_data.iter() {
            let entity = entry.1;
            let entity = entity.read().expect("Unable to Read Entity in Render in Game");
            match entity.get_graphics_data() {
                Some(data) => {
                    frame.draw_entity(data, self.transforms.clone());
                },
                None => (),
            }
        }
        frame.end();
    }

    fn tick(&mut self, delta_time: f64) {
        let world = self.world.clone();
        let manager = self.manager.clone();
        let delta_time = Arc::new(delta_time);
        self.thread_pool.scoped(|scope| {
            let entity_data = world.get_entity_data();
            let entity_data = entity_data.read().expect("Unable to Read Entity Data in Tick in Game");
            for entry in entity_data.iter() {
                let entity = entry.1.clone();
                let world = world.clone();
                let delta_time = delta_time.clone();
                scope.execute(move || {
                    entity.read().expect("Unable to Read Entity in Tick in Game").tick(delta_time, world);
                });
            }
            scope.join_all();
            for entry in entity_data.iter() {
                let entity = entry.1.clone();
                let world = world.clone();
                let manager = manager.clone();
                scope.execute(move || {
                    entity.write().expect("Unable to Write Entity in Tick in Game").tick_mut(manager, world);
                });
            }
        });
    }
}
