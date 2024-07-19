use types::{
    color::Grayscale,
    frame::{default_event_loop, default_window, Frame, FrameBuilder},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

mod types;

fn draw(frame: &mut Frame) {
    let w = 4f64;
    let h = (w * frame.height() as f64) / frame.width() as f64;

    let xmin = -w / 2.0;
    let ymin = -h / 2.0;

    let max_iterations = 100;
    let xmax = xmin + w;
    let ymax = ymin + h;

    let dx = (xmax - xmin) / frame.width() as f64;
    let dy = (ymax - ymin) / frame.height() as f64;

    let mut y = ymin;
    for j in 0..frame.height() {
        let mut x = xmin;
        for i in 0..frame.width() {
            let mut a = x;
            let mut b = y;
            let mut n = 0;
            let max = 4.0;
            let mut abs_old = 0.0;
            let mut converge_number = max_iterations;
            while n < max_iterations {
                let aa = a * a;
                let bb = b * b;
                let abs = (aa + bb).sqrt();
                if abs > max {
                    let diff_to_last = abs - abs_old;
                    let diff_to_max = max - abs_old;
                    converge_number = (n as f64 + diff_to_max / diff_to_last) as i32;
                    break;
                }
                let twoab = 2.0 * a * b;
                a = aa - bb + x;
                b = twoab + y;
                n += 1;
                abs_old = abs;
            }

            if n == max_iterations {
                frame.set_by_index((i + j * frame.width()) as usize, Grayscale::new(0));
            } else {
                let norm = types::utils::remap_value(converge_number as f64, (0.0, max_iterations as f64), (0.0, 1.0));
                frame.set_by_index(
                    (i + j * frame.width()) as usize,
                    Grayscale::new(types::utils::remap_value(
                        norm.sqrt(),
                        (0.0, 1.0),
                        (0.0, 255.0),
                    ) as u8),
                )
            }
            x += dx;
        }
        y += dy;
    }
}

fn main() {
    let event_loop = default_event_loop();
    let window = default_window(1920, 1080, "Rust Mandelbrot!!!", &event_loop);

    let surface = pixels::SurfaceTexture::new(
        window.inner_size().height,
        window.inner_size().width,
        &window,
    );

    let mut frame = FrameBuilder::new(window.inner_size().width, window.inner_size().height)
        .with_event_loop(&event_loop)
        .with_surface(surface)
        .with_background(Grayscale::new(255))
        .build()
        .unwrap();

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
