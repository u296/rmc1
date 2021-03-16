mod block;
mod chunk;
mod camera;
mod world;
mod graphics;
mod input;
mod terraingen;

use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};

use log::*;

use glium::{
    glutin,
    Surface,
    Display,
    Frame,
    Program,
    DrawParameters,
    VertexBuffer,
    IndexBuffer,
    index,
    uniforms,
    framebuffer,
    texture,
    implement_vertex,
    uniform, program
};
use texture::{Texture2d, SrgbTexture2d, DepthTexture2d, DepthFormat, UncompressedFloatFormat};
use framebuffer::{SimpleFrameBuffer, DepthRenderBuffer};
use index::PrimitiveType;
use uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler};
use glutin::event::{self, Event, WindowEvent};

use world::World;
use graphics::Mesh;

#[derive(Clone, Copy, Debug)]
struct FbVert {
    position: [f32; 2]
}

implement_vertex!(FbVert, position);

fn draw(
    mut frame: Frame,
    world: &World,
    fbquad: &Mesh<FbVert>,
    post_prog: &Program,
    fb: &mut SimpleFrameBuffer,
    fb_color_tex: &texture::SrgbTexture2d,
    fb_depth_tex: &texture::DepthTexture2d,
    ) -> Result<(), glium::SwapBuffersError> {

    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    
    fb.clear_depth(1.0);
    world.render(fb);

    let uniforms = uniform! {
        color: Sampler::new(fb_color_tex)
            .minify_filter(MinifySamplerFilter::Nearest)
            .magnify_filter(MagnifySamplerFilter::Nearest),
        depth: Sampler::new(fb_depth_tex)
            .minify_filter(MinifySamplerFilter::Nearest)
            .magnify_filter(MagnifySamplerFilter::Nearest)
        
    };

    let params = DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::Overwrite,
            write: false,
            .. Default::default()
        },
        .. Default::default()
    };

    frame.draw(&fbquad.vertices, &fbquad.indices, post_prog, &uniforms, &params).unwrap();

    frame.finish()
}

fn main() {
    env_logger::init();

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    

    let fbquad_mesh = {
        let fbquad_verts_data = &[
            FbVert{ position: [-1.0, -1.0]},
            FbVert{ position: [ 1.0, -1.0]},
            FbVert{ position: [-1.0,  1.0]},
            FbVert{ position: [ 1.0,  1.0]},
        ];

        let fbquad_indices_data = &[
            0, 1, 2,
            1, 2, 3
        ];
        
        Mesh {
            vertices: VertexBuffer::new(&display, fbquad_verts_data).unwrap(),
            indices: IndexBuffer::new(&display, PrimitiveType::TrianglesList, fbquad_indices_data).unwrap(),
        }
    };

    let fbquad_shader = match program!(&display,
        420 => {
            vertex: include_str!("shaders/fbquad_shader.vert"),
            fragment: include_str!("shaders/fbquad_shader.frag")
    }) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
    };



    let world_generator = terraingen::opensimplex::OpensimplexGenerator::new(
        Some(74324),
        (0.03, 0.03),
        30.0,
        30.0,
    );

    let world = Rc::new(RefCell::new(world::World::generate(
        &display,
        &world_generator,
        256,
        256)
    ));

    
    

    let mut inputhandler = input::InputHandler::new(world.clone(), display.gl_window().window(), true);
    
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
            info!("[FPS: {}] [frame time: {}]", render_count, (1.0 / worst_frame_time.as_secs_f64()) as i32);
            render_count = 0;
            worst_frame_time = Duration::from_secs(0);
        }

        

        match inputhandler.handle_event(&display, &event) {
            Some(action) => {
                *control_flow = action;
                return
            }
            _ => ()
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
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::Resized(s) => {
                    //*framebuffer_quad_texture.borrow_mut() = texture::Texture2d::empty(&display, s.width, s.height).unwrap();
                }
                _ => (),
            },
            Event::RedrawRequested(_) => render = true,
            Event::MainEventsCleared => render = true,
            _ => ()
        };

        
        if render {
            *last_draw.borrow_mut() = Instant::now();

            inputhandler.flush_updates(last_frame_time.as_secs_f32());
            world.borrow_mut().update(&display, last_frame_time.as_secs_f32());



            let s = display.gl_window().window().inner_size();
            let framebuffer_quad_depth_texture = glium::texture::DepthTexture2d::empty(&display, s.width, s.height).unwrap();
            let framebuffer_quad_color_texture = glium::texture::SrgbTexture2d::empty(&display, s.width, s.height).unwrap();

            let mut fb = SimpleFrameBuffer::with_depth_buffer(&display, &framebuffer_quad_color_texture, &framebuffer_quad_depth_texture).unwrap();


            draw(
                display.draw(),
                &world.borrow(),
                &fbquad_mesh,
                &fbquad_shader,
                &mut fb,
                &framebuffer_quad_color_texture,
                &framebuffer_quad_depth_texture,
            ).unwrap();

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