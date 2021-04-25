

use std::rc::Rc;
use std::cell::RefCell;

use log::*;

use glium::glutin;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event::{self, VirtualKeyCode, Event, WindowEvent, MouseButton, ElementState, DeviceEvent};
use glutin::window::Window;
use glium::Display;

use crate::world::World;
use crate::chunk::{Chunk, CHUNK_SIZE_U8};
use crate::block::{Block, types::*};
use crate::camera;

mod raycast;

const MOUSE_SENSITIVITY: f64 = 0.07;

const FORWARD_KEY: VirtualKeyCode =  VirtualKeyCode::Comma;
const LEFT_KEY: VirtualKeyCode =     VirtualKeyCode::A;
const BACKWARD_KEY: VirtualKeyCode = VirtualKeyCode::O;
const RIGHT_KEY: VirtualKeyCode =    VirtualKeyCode::E;
const UP_KEY: VirtualKeyCode =       VirtualKeyCode::Space;
const DOWN_KEY: VirtualKeyCode =     VirtualKeyCode::LControl;

const RELOAD_KEY: VirtualKeyCode =   VirtualKeyCode::F5;

pub struct InputHandler {
    world: Rc<RefCell<World>>,
    capturing_mouse: bool,
    camera_controller: camera::controller::Controller,
}

impl InputHandler {
    pub fn new(world: Rc<RefCell<World>>, window: &glutin::window::Window, capturing_mouse: bool) -> Self {
        if capturing_mouse {
            Self::set_mouse_capture_state(window, true);
        }

        InputHandler {
            world: world,
            capturing_mouse: capturing_mouse,
            camera_controller: Default::default()
        }
    }

    /// invalidates the correct chunks when the specified block
    /// changes
    fn invalidate_block_chunkmeshes(world: &mut World, global_coords: [i32; 3]) {
        let (chunk_coords, block_coords) = Chunk::get_local_coord_from_world_coord(global_coords);

        // the chunk in which the block changed will always be invalid
        world.flag_chunkmesh_dirty(chunk_coords);

        if block_coords[0] == 0 {
            world.flag_chunkmesh_dirty([chunk_coords[0] - 1, chunk_coords[1], chunk_coords[2]]);
        }
        else if block_coords[0] == CHUNK_SIZE_U8 - 1 {
            world.flag_chunkmesh_dirty([chunk_coords[0] + 1, chunk_coords[1], chunk_coords[2]]);
        }

        if block_coords[1] == 0 {
            world.flag_chunkmesh_dirty([chunk_coords[0], chunk_coords[1] - 1, chunk_coords[2]]);
        }
        else if block_coords[1]== CHUNK_SIZE_U8 - 1 {
            world.flag_chunkmesh_dirty([chunk_coords[0], chunk_coords[1] + 1, chunk_coords[2]]);
        }

        if block_coords[2] == 0 {
            world.flag_chunkmesh_dirty([chunk_coords[0], chunk_coords[1], chunk_coords[2] - 1]);
        }
        else if block_coords[2] == CHUNK_SIZE_U8 - 1 {
            world.flag_chunkmesh_dirty([chunk_coords[0], chunk_coords[1], chunk_coords[2] + 1]);
        }
    }

    fn handle_keyboard_event(&mut self, display: &Display, input: &glutin::event::KeyboardInput) -> Option<ControlFlow> {
        let pressed = input.state == event::ElementState::Pressed;
        let key = input.virtual_keycode?;

        match key {
            UP_KEY => self.camera_controller.moving_up = pressed,
            DOWN_KEY => self.camera_controller.moving_down = pressed,
            FORWARD_KEY => self.camera_controller.moving_forward = pressed,
            LEFT_KEY => self.camera_controller.moving_left = pressed,
            BACKWARD_KEY => self.camera_controller.moving_back = pressed,
            RIGHT_KEY => self.camera_controller.moving_right = pressed,

            RELOAD_KEY => {
                match self.world.borrow_mut().reload_assets(display) {
                    Ok(_) => (),
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
            VirtualKeyCode::Escape => {
                self.capturing_mouse = false;
                Self::set_mouse_capture_state(display.gl_window().window(), false);
            },
            _ => (),
        }
        None
    }

    fn handle_mouse_movement(&mut self, delta: &(f64, f64)) -> Option<ControlFlow> {
        if self.capturing_mouse {
            

            let mut world = self.world.borrow_mut();

            world.camera.rotate_yaw(-(delta.0 * MOUSE_SENSITIVITY) as f32);
            world.camera.rotate_pitch(-(delta.1 * MOUSE_SENSITIVITY) as f32);
        }
        None
    }

    fn handle_mouse_button(&mut self, display: &Display, state: &ElementState, button: &MouseButton) -> Option<ControlFlow> {
        if !self.capturing_mouse && *button == MouseButton::Left && *state == ElementState::Pressed {
            self.capturing_mouse = true;
            Self::set_mouse_capture_state(display.gl_window().window(), true);
        }
        else if self.capturing_mouse {
            let mut world = self.world.borrow_mut();

            let block_occupation_checker = |coord: [i32; 3]| -> bool {
                let (chunk_coords, block_coords) = Chunk::get_local_coord_from_world_coord(coord);
                
                match world.chunks.iter()
                    .find(|c| c.coordinates == chunk_coords) {
                        Some(chunk) => match chunk.get_block(block_coords) {
                                Some(_) => true,
                                None => false
                            },
                        None => false
                        
                    }
            };

            if *button == MouseButton::Left && *state == ElementState::Pressed {
                match raycast::raycast(block_occupation_checker, world.camera.get_position(), world.camera.get_view_dir(), 6.0) {
                    Some(coordinates) => {
                        let (chunk_coords, block_coords) = Chunk::get_local_coord_from_world_coord(coordinates);
                        let chunk = world.chunks.iter_mut().find(|c| c.coordinates == chunk_coords).unwrap();
                        let block = chunk.get_block_mut(block_coords);
                        *block = None;
                        debug!("destroyed block: chunk: {:?} block: {:?}", chunk_coords, block_coords);
                        
                        Self::invalidate_block_chunkmeshes(&mut world, coordinates);
                    },
                    None => ()
                }
            } else if *button == MouseButton::Right && *state == ElementState::Pressed {
                let (hit_block, path) = raycast::raycast_path(block_occupation_checker, world.camera.get_position(), world.camera.get_view_dir(), 6.0);

                if hit_block {
                    if path.len() >= 2 { // check that the player isn't inside a block when trying to place
                        let place_location = path[path.len() - 2]; // second last block in the path is the block before collision
                        let (chunk_coords, block_coordinates) = Chunk::get_local_coord_from_world_coord(place_location);

                        match world.chunks.iter_mut().find(|c| c.coordinates == chunk_coords) {
                            Some(chunk) => {
                                let block = chunk.get_block_mut(block_coordinates);
                                *block = Some(Block::new(block_coordinates, &GLASS_BLOCK));
                                Self::invalidate_block_chunkmeshes(&mut world, place_location);
                            },
                            None => () // too bad, the chunk doesn't exist
                        }
                    }
                }
            }
        }
        
        

        None
    }

    fn handle_window_resize(&mut self, newsize: (u32, u32)) -> Option<ControlFlow> {
        let mut world = self.world.borrow_mut();
        world.camera.set_aspect_ratio(newsize.0 as f32 / newsize.1 as f32);
        None
    }
    
    pub fn handle_event(&mut self, display: &Display, event: &Event<()>) -> Option<ControlFlow> {
        
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::Resized(newsize) => self.handle_window_resize((newsize.width, newsize.height)),
                WindowEvent::KeyboardInput { input, .. } => self.handle_keyboard_event(display, input),
                WindowEvent::MouseInput { state, button, .. } => self.handle_mouse_button(display, state, button),
                _ => None,
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion{delta} => self.handle_mouse_movement(delta),
                _ => None
            },
            _ => None
        }

        
    }

    pub fn flush_updates(&mut self, seconds: f32) {
        let mut world = self.world.borrow_mut();
        self.camera_controller.update_camera(&mut world.camera, seconds);
    }

    fn set_mouse_capture_state(window: &Window, capture: bool) {
        if capture {
            window.set_cursor_visible(false);
            window.set_cursor_grab(true).unwrap();
        } else {
            window.set_cursor_visible(true);
            window.set_cursor_grab(false).unwrap();
        }
    }
}