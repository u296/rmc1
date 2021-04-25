mod block;
mod camera;
mod chunk;
mod graphics;
mod input;
mod terraingen;
mod world;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use log::*;

use glium::{glutin, implement_vertex, Frame, Surface};

use glutin::event::Event;

use world::World;

#[derive(Clone, Copy, Debug)]
struct FbVert {
    position: [f32; 2],
}

implement_vertex!(FbVert, position);

fn draw(mut frame: Frame, world: &mut World) -> Result<(), glium::SwapBuffersError> {
    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    world.render(&mut frame);

    frame.finish()
}

fn main() {
    env_logger::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let world_generator = terraingen::opensimplex::OpensimplexGenerator::new(
        Some(453209875342987),
        (0.03, 0.03),
        30.0,
        30.0,
    );

    let world = Rc::new(RefCell::new(world::World::generate(
        &display,
        &world_generator,
        256,
        256,
    )));

    let mut inputhandler =
        input::InputHandler::new(world.clone(), display.gl_window().window(), true);

    let mut last_frame_printout_time = 0;
    let start = Rc::new(RefCell::new(Instant::now()));
    let clock1 = Rc::new(RefCell::new(Instant::now()));
    let clock2 = Rc::new(RefCell::new(Instant::now() + Duration::from_millis(16)));
    let last_draw = Rc::new(RefCell::new(Instant::now()));

    // we only render when there are no incoming events
    let mut render_count = 0;
    let mut last_frame_time = Duration::from_secs(0);
    let mut worst_frame_time = Duration::from_secs(0);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        let last_loop_time = clock2.borrow().duration_since(*clock1.borrow());
        last_frame_time += last_loop_time;

        {
            *clock1.borrow_mut() = Instant::now();
        }

        if clock1.borrow().duration_since(*start.borrow()).as_secs() != last_frame_printout_time {
            last_frame_printout_time = clock1.borrow().duration_since(*start.borrow()).as_secs();
            info!(
                "[FPS: {}] [frame time: {}]",
                render_count,
                (1.0 / worst_frame_time.as_secs_f64()) as i32
            );
            render_count = 0;
            worst_frame_time = Duration::from_secs(0);
        }

        match inputhandler.handle_event(&display, &event) {
            Some(action) => {
                *control_flow = action;
                return;
            }
            _ => (),
        }

        // we don't want to render if there are still events
        // waiting to be processed, so we will only render the
        // frame when we have flushed out all the events, which
        // will cause Event::MainEventsCleared to be emitted,
        // then we know that there are no events waiting
        let mut render = false;

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                glutin::event::WindowEvent::Resized(_) => {
                    render = true;
                }
                _ => (),
            },
            Event::RedrawRequested(_) => render = true,
            Event::MainEventsCleared => render = true,
            _ => (),
        };

        if render {
            *last_draw.borrow_mut() = Instant::now();

            inputhandler.flush_updates(last_frame_time.as_secs_f32());
            world
                .borrow_mut()
                .update(&display, last_frame_time.as_secs_f32());

            draw(display.draw(), &mut world.borrow_mut()).unwrap();

            if last_frame_time > worst_frame_time {
                worst_frame_time = last_frame_time;
            }

            last_frame_time = Duration::from_secs(0);
            render_count += 1;
        }

        {
            *clock2.borrow_mut() = Instant::now();
        }
    });
}
