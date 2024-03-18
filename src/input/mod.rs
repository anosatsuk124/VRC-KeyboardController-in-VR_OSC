use crate::osc::{self, send_packet};
use anyhow::Result;
use rosc::OscType;
use std::sync::Arc;

pub static INPUT_HANDLER: once_cell::sync::OnceCell<Arc<std::sync::Mutex<InputHandler>>> =
    once_cell::sync::OnceCell::new();

#[derive(Debug, Clone, Copy, Default)]
struct Movement {
    vertical: f32,
    horizontal: f32,
}

#[derive(Debug, Clone, Copy, Default)]
struct Looking(f32);

#[derive(Debug, Clone, Copy, Default)]
pub struct InputHandler {
    movement: Movement,
    looking_to: Looking,
}

impl InputHandler {
    const MOV_VERTICALLY: &'static str = "/Vertical";
    const MOV_HORIZONTALLY: &'static str = "/Horizontal";

    const LOOKING_HORIZONTALLY: &'static str = "/LookHorizontal";

    fn new() -> Self {
        Self::default()
    }

    fn update_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }

    fn update_looking(&mut self, looking_to: Looking) {
        self.looking_to = looking_to;
    }

    fn reset_all(&mut self) {
        self.movement = Movement::default();
        self.looking_to = Looking::default();
    }
}

impl InputHandler {
    pub fn init() -> Result<()> {
        match INPUT_HANDLER.set(Arc::new(std::sync::Mutex::new(Self::new()))) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!("failed to init input handler: {:?}", e),
        }
    }

    pub fn mov_right(&mut self) {
        self.update_movement(Movement {
            horizontal: 1f32,
            vertical: self.movement.vertical,
        })
    }

    pub fn mov_left(&mut self) {
        self.update_movement(Movement {
            horizontal: -1f32,
            vertical: self.movement.vertical,
        })
    }

    pub fn mov_forward(&mut self) {
        self.update_movement(Movement {
            horizontal: self.movement.horizontal,
            vertical: 1f32,
        })
    }

    pub fn mov_backward(&mut self) {
        self.update_movement(Movement {
            horizontal: self.movement.horizontal,
            vertical: -1f32,
        })
    }

    pub fn look_right(&mut self) {
        self.update_looking(Looking(1f32));
    }

    pub fn look_left(&mut self) {
        self.update_looking(Looking(-1f32));
    }

    pub fn eval(&mut self) -> Result<()> {
        send_packet(
            Self::MOV_VERTICALLY,
            vec![OscType::Float(self.movement.vertical)],
        )?;
        send_packet(
            Self::MOV_HORIZONTALLY,
            vec![OscType::Float(self.movement.horizontal)],
        )?;
        send_packet(
            Self::LOOKING_HORIZONTALLY,
            vec![OscType::Float(self.looking_to.0)],
        )?;

        self.reset_all();
        Ok(())
    }
}
