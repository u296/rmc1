use std::error::Error;

use glium::{Display, Surface};

use crate::graphics::Renderable;

mod crosshair;
mod hotbar;


pub struct Hud {
    crosshair: crosshair::Crosshair,
}

impl Hud {
    pub fn new(display: &Display) -> Self {
        Hud {
            crosshair: crosshair::Crosshair::new(display)
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.crosshair.set_aspect_ratio(aspect_ratio);
    }
}

impl<S: Surface> Renderable<S> for Hud {
    fn render(&self, surface: &mut S) -> Result<(), Box<dyn Error>> {
        let components = &[&self.crosshair];

        for component in components.iter() {
            component.render(surface)?;
        }

        Ok(())
    }
}