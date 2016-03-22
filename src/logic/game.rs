use std::sync::{Arc};
use std::fmt;
use std::error::{Error};
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use glium::glutin::Event as WindowEvent;

use input::{Keyboard, Mouse, Display, KeyCode, ButtonState, MouseButton, Button};
use logic::{World, WorldErr, EntityData, IdManager};
use math::{Vec2};
use graphics::{Window, MatrixData};

pub struct Game<T: EntityData<T>> {
    world: Arc<World<T>>,
    matrix_data: Arc<MatrixData>,
    thread_pool: Pool,
}

impl<T: EntityData<T>> Game<T> {
    pub fn new(thread_count: u32, resolution: Vec2) -> Game<T> {
        let keyboard = Arc::new(Keyboard::new());
        let mouse = Arc::new(Mouse::new());
        let display = Arc::new(Display::new(resolution));
        let matrix_data = Arc::new(MatrixData::new());
        Game {
            world: Arc::new(World::new(keyboard.clone(), mouse.clone(), display.clone())),
            matrix_data: matrix_data,
            thread_pool: Pool::new(thread_count),
        }
    }

    pub fn get_world(&self) -> Arc<World<T>> {
        self.world.clone()
    }

    pub fn get_mut_world(&mut self) -> Result<&mut World<T>, GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => Ok(world),
            None => Err(GameErr::GetMut),
        }
    }

    fn pause(&mut self) {
        println!("Paused");
    }

    fn resume(&mut self) {
        println!("Resumed");
    }

    fn update_keyboard(&mut self, tick_number: u64, key_code: KeyCode, element_state: ButtonState) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_key(key_code, Button::new(tick_number, element_state)) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(GameErr::World(err)),
                }
            },
            None => Err(GameErr::GetMut),
        }
    }

    fn update_mouse_button(&mut self, tick_number: u64, mouse_button: MouseButton, element_state: ButtonState) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_button(mouse_button, Button::new(tick_number, element_state)) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::World(err)),
                }
            }
            None => Err(GameErr::GetMut),
        }
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_position(Vec2::from([mouse_pos.0 as f32, mouse_pos.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::World(err)),
                }
            },
            None => Err(GameErr::GetMut),
        }
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_resolution(Vec2::from([resolution.0 as f32, resolution.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::World(err)),
                }
            },
            None => Err(GameErr::GetMut),
        }
    }

    pub fn run(&mut self, window: &mut Window, manager: &mut IdManager) -> Result<(), GameErr> {
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
                        WindowEvent::Resized(width, height) => match self.update_resolution((width, height)) {
                            Ok(()) => (),
                            Err(err) => return Err(GameErr::Game(Box::new(err))),
                        },
                        // WindowEvent::Moved(x, y) => {
                        //
                        // },
                        WindowEvent::Closed => return Ok(()),
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
                            Some(virtual_key_code) => match self.update_keyboard(tick_number, virtual_key_code, element_state) {
                                Ok(()) => (),
                                Err(err) => return Err(GameErr::Game(Box::new(err))),
                            },
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => match self.update_mouse_pos(pos) {
                            Ok(()) => (),
                            Err(err) => return Err(GameErr::Game(Box::new(err))),
                        },
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => match self.update_mouse_button(tick_number, mouse_button, element_state) {
                            Ok(()) => (),
                            Err(err) => return Err(GameErr::Game(Box::new(err))),
                        },
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
                match self.tick(tps_s, manager) {
                    Ok(()) => (),
                    Err(err) => return Err(GameErr::Game(Box::new(err))),
                };
                delta_time -= tps_s;
                ticks += 1;
                tick_number += 1;
            }
            match self.render(window) {
                Ok(()) => (),
                Err(err) => return Err(GameErr::Game(Box::new(err))),
            }
            frames += 1;
            if now > i + 1.0 {
                i += 1.0;
                println!("{} {}", frames.to_string(), ticks.to_string());
                frames = 0;
                ticks = 0;
            }
        }
    }

    fn render(&mut self, window: &mut Window) -> Result<(), GameErr> {
        let mut world = match Arc::get_mut(&mut self.world) {
            Some(world) => world,
            None => return Err(GameErr::GetMut),
        };
        for (_, entity) in match world.get_mut_entity_data() {
            Ok(entity_data) => entity_data,
            Err(err) => return Err(GameErr::World(err)),
        }.iter_mut() {
            match match Arc::get_mut(entity) {
                    Some(entity) => entity,
                    None => return Err(GameErr::GetMut),
                }.render(window, match Arc::get_mut(&mut self.matrix_data) {
                    Some(matrix_data) => matrix_data,
                    None => return Err(GameErr::GetMut),
                }) {
                Ok(()) => (),
                Err(err) => return Err(GameErr::Entity(err)),
            }
        }
        let mut frame = window.frame();
        for entry in match world.get_mut_entity_data() {
            Ok(entity_data) => entity_data,
            Err(err) => return Err(GameErr::World(err)),
        }.iter_mut() {
            match entry.1.get_renderable(){
                Some(data) => {
                    frame.draw_entity(data, self.matrix_data.clone());
                },
                None => (),
            }
        }
        frame.end();
        Ok(())
    }

    fn tick(&mut self, delta_time: f64, manager: &mut IdManager) -> Result<(), GameErr> {
        {
            let world = self.world.clone();
            let delta_time = Arc::new(delta_time);
            self.thread_pool.scoped(|scope| {
                for entry in world.get_entity_data().iter() {
                    let entity = entry.1.clone();
                    let world = world.clone();
                    let delta_time = delta_time.clone();
                    scope.execute(move || {
                        entity.tick(delta_time, world);
                    });

                }

            });
        }
        match Arc::get_mut(&mut self.world)  {
            Some(world) => {
                match world.get_mut_entity_data() {
                    Ok(entity_data) => {
                        let mut keys = vec!();
                        for key in entity_data.keys() {
                            keys.push(key.clone());
                        }
                        for key in keys {
                            let mut entity = match entity_data.remove(&key) {
                                Some(entity) => entity,
                                None => return Err(GameErr::BadIndex),
                            };
                            match Arc::get_mut(&mut entity) {
                                Some(entity) => entity,
                                None => return Err(GameErr::GetMut),
                            }.tick_mut(manager);
                            entity_data.insert(key, entity);
                        }
                        Ok(())
                    },
                    Err(err) => Err(GameErr::World(err)),
                }
            },
            None => Err(GameErr::GetMut),
        }
    }
}

#[derive(Debug)]
pub enum GameErr {
    World(WorldErr),
    Game(Box<GameErr>),
    Entity(Box<Error>),
    GetMut,
    BadIndex,
}

impl fmt::Display for GameErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameErr::World(ref err) => err.fmt(f),
            GameErr::Game(ref err) => err.fmt(f),
            GameErr::Entity(ref err) => err.fmt(f),
            GameErr::GetMut => write!(f, "Get Mut was None"),
            GameErr::BadIndex => write!(f, "Index was None"),
        }
    }
}

impl Error for GameErr {
    fn description(&self) -> &str {
        match *self {
            GameErr::World(ref err) => err.description(),
            GameErr::Game(ref err) => err.description(),
            GameErr::Entity(ref err) => err.description(),
            GameErr::GetMut => "Get Mut was None",
            GameErr::BadIndex => "Index was None",
        }
    }
}
