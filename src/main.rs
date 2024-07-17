mod types;

use std::str::FromStr;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::types::frame::Frame;
use crate::types::color::Rgba;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 480;
const FIFTH: u32 = HEIGHT / 5;

fn color_line(frame: &mut Frame, stripe: u32, color: Rgba) {
    for line in 0..FIFTH {
        for pixel in 0..WIDTH {
            frame.set(pixel as usize, (line + stripe) as usize, &color);
        }
    }
}

fn draw(frame: &mut Frame) {
    color_line(frame, 0,         Rgba::from_str("#5bcefa").unwrap());
    color_line(frame, FIFTH,     Rgba::from_str("#f5a9b8").unwrap());
    color_line(frame, FIFTH * 2, Rgba::from_str("#ffffff").unwrap());
    color_line(frame, FIFTH * 3, Rgba::from_str("#f5a9b8").unwrap());
    color_line(frame, FIFTH * 4, Rgba::from_str("#5bcefa").unwrap());
}

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let size = winit::dpi::LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("Hello, World!")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut frame = Frame::new(WIDTH, HEIGHT, &window);
    draw(&mut frame);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: e, .. } => match e {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => frame
                    .resize_surface(new_size.width, new_size.height)
                    .unwrap(),
                _ => {}
            },
            Event::RedrawRequested(_) => {
                if frame.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        };

        window.request_redraw();
    });
}
