use std::cell::{RefCell};

use glium::{Display, Surface};

use crate::graphics::Renderable;

mod crosshair;
mod hotbar;

pub struct Hud {
    crosshair: RefCell<crosshair::Crosshair>,
}

impl Hud {
    

    pub fn new(display: &Display) -> Self {
        Hud {
            crosshair: RefCell::new(crosshair::Crosshair::new(display))
        }
    }

    pub fn render(&self, frame: &mut impl Surface, aspect_ratio: f32) {
        let mut crosshair = self.crosshair.borrow_mut();
        crosshair.set_aspect_ratio(aspect_ratio);
        drop(crosshair);

        let components = &[self.crosshair.borrow()];

        for component in components.iter() {
            component.render(frame);
        }
    }
}
