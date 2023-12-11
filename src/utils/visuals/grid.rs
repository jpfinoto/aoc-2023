use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::utils::grid::Cellular;

pub struct WindowOptions {
    pub width: u32,
    pub height: u32,
    pub title: &'static str,
    pub background_color: Color,
}

pub struct GridOptions {
    pub window: WindowOptions,
    pub grid_scale: f32,
}

pub trait GridRenderer<T> {
    fn render(&self, tile: &T) -> Color;
}

pub struct StaticGrid<'a, T, R> where T: Cellular, R: GridRenderer<T> {
    options: &'a GridOptions,
    renderer: &'a R,
    data: &'a [T],
}

pub fn plot_grid<T, R>(options: &GridOptions, renderer: &R, data: &[T])
    where T: Cellular, R: GridRenderer<T>
{
    base_loop(&options.window, &mut StaticGrid {
        options,
        renderer,
        data,
    });
}

impl<'a, T, R> RenderCallback for StaticGrid<'a, T, R> where T: Cellular, R: GridRenderer<T> {
    fn on_render(&self, canvas: &mut WindowCanvas) {
        for item in self.data {
            canvas.set_draw_color(self.renderer.render(&item));
            let grid_cell = item.cell();
            canvas.fill_rect(Rect::new(
                grid_cell.left,
                grid_cell.top,
                (((grid_cell.right - grid_cell.left) as f32) * self.options.grid_scale) as u32,
                (((grid_cell.bottom - grid_cell.top) as f32) * self.options.grid_scale) as u32,
            )).expect("fill rect");
        }
    }
}

pub trait RenderCallback {
    fn on_render(&self, canvas: &mut WindowCanvas);
}

pub fn base_loop<C: RenderCallback>(options: &WindowOptions, renderer: &mut C) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(options.title, options.width, options.height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(options.background_color);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.clear();

        renderer.on_render(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
