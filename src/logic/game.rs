use std::sync::{Arc};
use scoped_threadpool::{Pool};
use time::{precise_time_s};
use glium::glutin::Event as WindowEvent;

use input::{Keyboard, Mouse, Display, KeyCode, ButtonState, MouseButton, Button};
use logic::{World, WorldErr, EntityData};
use math::{Vec2};
use graphics::{Window, Transforms};

pub struct Game<T: EntityData<T>> {
    world: Arc<World<T>>,
    transforms: Arc<Transforms>,
    thread_pool: Pool,
}

impl<T: EntityData<T>> Game<T> {
    pub fn new(thread_count: u32, resolution: Vec2) -> Game<T> {
        let keyboard = Arc::new(Keyboard::new());
        let mouse = Arc::new(Mouse::new());
        let display = Arc::new(Display::new(resolution));
        let transforms = Arc::new(Transforms::new());
        Game {
            world: Arc::new(World::new(keyboard.clone(), mouse.clone(), display.clone())),
            transforms: transforms,
            thread_pool: Pool::new(thread_count),
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

    fn update_keyboard(&mut self, tick_number: u64, key_code: KeyCode, element_state: ButtonState) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_key(key_code, Button::new(tick_number, element_state)) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(GameErr::UpdateKeyboardWorld(err)),
                }
            },
            None => Err(GameErr::UpdateKeyboard("Unable to Get Mut World")),
        }
    }

    fn update_mouse_button(&mut self, tick_number: u64, mouse_button: MouseButton, element_state: ButtonState) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_button(mouse_button, Button::new(tick_number, element_state)) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::UpdateMouseButtonWorld(err)),
                }
            }
            None => Err(GameErr::UpdateMouseButton("Unable to Get Mut World")),
        }
    }

    fn update_mouse_pos(&mut self, mouse_pos: (i32, i32)) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_mouse_position(Vec2::from([mouse_pos.0 as f32, mouse_pos.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::UpdateMousePositionWorld(err)),
                }
            },
            None => Err(GameErr::UpdateMousePosition("Unable to Get Mut World")),
        }
    }

    fn update_resolution(&mut self, resolution: (u32, u32)) -> Result<(), GameErr> {
        match Arc::get_mut(&mut self.world) {
            Some(world) => {
                match world.set_resolution(Vec2::from([resolution.0 as f32, resolution.1 as f32])) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(GameErr::UpdateResolutionWorld(err)),
                }
            },
            None => Err(GameErr::UpdateResolution("Unable to Get Mut World")),
        }
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
                        WindowEvent::Resized(width, height) => self.update_resolution((width, height)).unwrap(),
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
                            Some(virtual_key_code) => self.update_keyboard(tick_number, virtual_key_code, element_state).unwrap(),
                            None => (),
                        },
                        WindowEvent::MouseMoved(pos) => self.update_mouse_pos(pos).unwrap(),
                        // WindowEvent::MouseWheel(mouse_scroll_data) => {
                        //
                        // },
                        WindowEvent::MouseInput(element_state, mouse_button) => self.update_mouse_button(tick_number, mouse_button, element_state).unwrap(),
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
                self.tick(tps_s).unwrap();
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
        let mut world = Arc::get_mut(&mut self.world).unwrap();
        for entry in world.get_mut_entity_data().unwrap().iter_mut() {
            // entry.1.render(window);
        }
        let mut frame = window.frame();
        for entry in world.get_mut_entity_data().unwrap().iter_mut() {
            match entry.1.get_renderable(){
                Some(data) => {
                    frame.draw_entity(data, self.transforms.clone());
                },
                None => (),
            }
        }
        frame.end();
    }

    fn tick(&mut self, delta_time: f64) -> Result<(), GameErr> {
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
                        for entry in entity_data.iter_mut() {
                            // entity.tick_mut();
                        }
                        Ok(())
                    },
                    Err(err) => Err(GameErr::TickWorld(err)),
                }
            },
            None => Err(GameErr::Tick("Unable to Get Mut World")),
        }
    }
}

#[derive(Debug)]
pub enum GameErr {
    UpdateKeyboard(&'static str),
    UpdateKeyboardWorld(WorldErr),
    UpdateMouseButton(&'static str),
    UpdateMouseButtonWorld(WorldErr),
    UpdateMousePosition(&'static str),
    UpdateMousePositionWorld(WorldErr),
    UpdateResolution(&'static str),
    UpdateResolutionWorld(WorldErr),
    Tick(&'static str),
    TickWorld(WorldErr),
}
