use std::sync::Arc;

use nes_rs::cpu::{mem::Memory, CPU};
use pixels::{wgpu::TextureFormat, Pixels, PixelsBuilder, SurfaceTexture};
use rand::{rng, rngs::ThreadRng, Rng};
use tracing::{error, trace};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::Window,
};

use crate::game;

pub struct Emulator {
    #[allow(dead_code)]
    window: Arc<Window>,
    cpu: CPU,
    screen: Pixels<'static>,
    rng: ThreadRng,
}

impl Emulator {
    pub fn new(window: Arc<Window>) -> Self {
        let screen = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            PixelsBuilder::new(32, 32, surface_texture)
                .texture_format(TextureFormat::Rgba8Unorm)
                .build()
                .unwrap()
        };
        let rng = rng();
        let mut cpu = CPU::new();
        cpu.load(&game::GAME_CODE);
        cpu.reset();
        Self {
            window,
            screen,
            rng,
            cpu,
        }
    }

    pub fn window(&self) -> Arc<Window> {
        self.window.clone()
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>, event_loop: &ActiveEventLoop) {
        if let Err(err) = self.screen.resize_surface(size.width, size.height) {
            error!(target: "pixels.resize_surface", "Error: {}", err);
            event_loop.exit();
        }
    }

    pub fn render(&mut self, event_loop: &ActiveEventLoop) {
        self.update();

        if self.draw() {
            if let Err(err) = self.screen.render() {
                error!(target: "pixels.render", "Error: {}", err);
                event_loop.exit();
            }
        }
    }

    pub fn input(&mut self, event: KeyEvent, event_loop: &ActiveEventLoop) {
        if let KeyEvent {
            logical_key,
            state: ElementState::Pressed,
            ..
        } = event
        {
            match logical_key.as_ref() {
                Key::Named(NamedKey::Escape) => event_loop.exit(),
                Key::Character("w") | Key::Named(NamedKey::ArrowUp) => {
                    self.cpu.mem_write(0xFF, 0x77)
                }
                Key::Character("a") | Key::Named(NamedKey::ArrowLeft) => {
                    self.cpu.mem_write(0xFF, 0x61)
                }
                Key::Character("s") | Key::Named(NamedKey::ArrowDown) => {
                    self.cpu.mem_write(0xFF, 0x73)
                }
                Key::Character("d") | Key::Named(NamedKey::ArrowRight) => {
                    self.cpu.mem_write(0xFF, 0x64)
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {
        self.cpu.mem_write(0xfe, self.rng.random_range(1..16));
        self.cpu.tick();
        trace!("tick happened");
    }

    fn draw(&mut self) -> bool {
        let frame = self.screen.frame_mut();
        let mut frame_idx = 0;
        let mut update = false;
        for i in 0x0200..0x0600 {
            let (r, g, b, a) = Self::color(self.cpu.mem_read(i));
            if frame[frame_idx] != r
                || frame[frame_idx + 1] != g
                || frame[frame_idx + 2] != b
                || frame[frame_idx + 3] != a
            {
                frame[frame_idx] = r;
                frame[frame_idx + 1] = g;
                frame[frame_idx + 2] = b;
                frame[frame_idx + 3] = a;
                update = true;
            }
            frame_idx += 4
        }
        update
    }

    fn color(byte: u8) -> (u8, u8, u8, u8) {
        match byte {
            0 => (0, 0, 0, 255),
            1 => (255, 255, 255, 255),
            2 | 9 => (128, 128, 128, 255),
            3 | 10 => (255, 0, 0, 255),
            4 | 11 => (0, 255, 0, 255),
            5 | 12 => (0, 0, 255, 255),
            6 | 13 => (255, 0, 255, 255),
            7 | 14 => (255, 255, 0, 255),
            _ => (0, 255, 255, 255),
        }
    }
}
