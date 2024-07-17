mod types;

use std::str::FromStr;

use types::frame::{default_event_loop, default_window, FrameBuilder};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
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
    let event_loop = default_event_loop();
    let window = default_window(WIDTH, HEIGHT, "Transflag", &event_loop);

    let surface = pixels::SurfaceTexture::new(window.inner_size().width, window.inner_size().width, &window);

    let frame_builder = FrameBuilder::new(WIDTH, HEIGHT)
        .with_surface(surface)
        .with_event_loop(&event_loop);

    let mut frame = frame_builder.build().unwrap();

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
