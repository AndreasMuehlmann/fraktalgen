extern crate sdl2;

use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;


mod fraktal;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("fraktalgen", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut mandel_brot_set = fraktal::MandelBrotSet::new();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vertical_interval = mandel_brot_set.get_vertical_interval();
    let mut horizontal_interval = mandel_brot_set.get_horizontal_interval();

    let mut change = true;
    'running: loop {
        let vertical_travel_distance = (vertical_interval[1] - vertical_interval[0]) / 10.0;
        let horizontal_travel_distance = (horizontal_interval[1] - horizontal_interval[0]) / 10.0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },


                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    vertical_interval[0] -= vertical_travel_distance;
                    vertical_interval[1] -= vertical_travel_distance; 
                    change = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    vertical_interval[0] += vertical_travel_distance;
                    vertical_interval[1] += vertical_travel_distance;
                    change = true;
                },

                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    horizontal_interval[0] -= horizontal_travel_distance;
                    horizontal_interval[1] -= horizontal_travel_distance;
                    change = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    horizontal_interval[0] += horizontal_travel_distance;
                    horizontal_interval[1] += horizontal_travel_distance;
                    change = true;
                },
                Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                    horizontal_interval[0] += horizontal_travel_distance;
                    horizontal_interval[1] -= horizontal_travel_distance;
                    vertical_interval[0] += vertical_travel_distance;
                    vertical_interval[1] -= vertical_travel_distance;
                    change = true;
                },
                Event::KeyDown { keycode: Some(Keycode::O), .. } => {
                    horizontal_interval[0] -= horizontal_travel_distance;
                    horizontal_interval[1] += horizontal_travel_distance;
                    vertical_interval[0] -= vertical_travel_distance;
                    vertical_interval[1] += vertical_travel_distance;
                    change = true;
                },

                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    let max_iterations: u64 = mandel_brot_set.get_max_iterations();
                    mandel_brot_set.set_max_iterations(max_iterations + 500);
                    change = true;
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    let max_iterations: u64 = mandel_brot_set.get_max_iterations();
                    mandel_brot_set.set_max_iterations(max_iterations - 500 );
                    change = true;
                },

                _ => {}
            }

            mandel_brot_set.set_vertical_interval(vertical_interval);
            mandel_brot_set.set_horizontal_interval(horizontal_interval);
        }
        if change {
            change = false;
            mandel_brot_set.draw(&mut canvas);
            canvas.present();
        }
    }
}
