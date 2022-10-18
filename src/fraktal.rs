use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;


pub struct MandelBrotSet {
    max_iterations: u64,
    vertical_interval: [f32; 2],
    horizontal_interval: [f32; 2]
}


impl MandelBrotSet {
    pub fn new() -> Self {
        return Self {
            max_iterations: 100,
            vertical_interval: [-2.0, 2.0],
            horizontal_interval: [-2.0, 2.0]
        };
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let window_size: (u32, u32) = canvas.output_size().unwrap();
        for x in 0..window_size.0 {
            for y in 0..window_size.1 {
                let rgb: Color = self.give_color(
                    MandelBrotSet::number_from_interval_to_interval(x as f32, [0.0, window_size.0 as f32], self.horizontal_interval),
                    MandelBrotSet::number_from_interval_to_interval(y as f32, [0.0, window_size.1 as f32], self.vertical_interval)
                    );
                canvas.set_draw_color(rgb);
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }
    }

    fn give_color(&self, x: f32, y: f32) -> Color {
        let iterations: f32 = self.give_iterations_mandelbrot(x, y) as f32;
        
        let rgb_value: f32 = MandelBrotSet::number_from_interval_to_interval(iterations, [0.0, self.max_iterations as f32], [0.0, 255.0]);
        let smoothed_rgb_value: f32 = 255.0 * rgb_value.log(10.0) / 255.0_f32.log(10.0);

        // black red
        // return Color::RGB(rgb_value as u8, 0, 0);

        // black white purple
        // return Color::RGB(smoothed_rgb_value as u8, rgb_value as u8, smoothed_rgb_value as u8);
         
        // black white green
        return Color::RGB(smoothed_rgb_value as u8, smoothed_rgb_value as u8, rgb_value as u8);
         
        // blue yellow red
        // return Color::RGB(smoothed_rgb_value as u8, rgb_value as u8, (80) as u8);
    }

    fn number_from_interval_to_interval(number: f32, interval_from: [f32; 2], interval_to: [f32; 2]) -> f32 {
        return (number - interval_from[0]) * (interval_to[1] - interval_to[0]) / (interval_from[1] - interval_from[0]) + interval_to[0];
    }

    fn give_iterations_mandelbrot(&self, x: f32, y: f32) -> u64 {
        let mut real_part: f32 = 0.0;
        let mut imaginary_part: f32 = 0.0;

        let mut iteration: u64 = 0;
        while real_part * real_part + imaginary_part * imaginary_part <= 4.0 && iteration < self.max_iterations {
            let new_real_part: f32 = real_part.powf(2.0) - imaginary_part.powf(2.0) + x;
            imaginary_part = 2.0 * real_part * imaginary_part + y;
            real_part = new_real_part;
             
            iteration += 1;
        }
        return iteration;
    }

    pub fn get_max_iterations(&self) -> u64 {
        return self.max_iterations;
    }

    pub fn set_max_iterations(&mut self, max_iterations: u64) {
        if max_iterations < 10 {
            self.max_iterations = 10;
            return;
        }
        self.max_iterations = max_iterations;

    }

    pub fn get_vertical_interval(&mut self) -> [f32; 2] {
        return self.vertical_interval;
    }

    pub fn get_horizontal_interval(&self) -> [f32; 2] {
        return self.horizontal_interval;
    }
    
    pub fn set_vertical_interval(&mut self, interval: [f32; 2]) {
        self.vertical_interval = interval;
    }

    pub fn set_horizontal_interval(&mut self, interval: [f32; 2]) {
        self.horizontal_interval = interval;
    }
}
