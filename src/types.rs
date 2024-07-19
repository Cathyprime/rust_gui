pub mod color {
    use std::str::FromStr;
    use thiserror::Error;

    pub trait Color {
        fn red(&self) -> u8;
        fn green(&self) -> u8;
        fn blue(&self) -> u8;
        fn alpha(&self) -> u8;
    }

    pub struct Rgba {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
        pub alpha: u8,
    }

    impl Rgba {
        pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
            Rgba {
                red,
                green,
                blue,
                alpha,
            }
        }
    }

    pub struct Grayscale {
        scale: u8,
    }

    impl Grayscale {
        pub fn new(scale: u8) -> Self {
            Grayscale { scale }
        }
    }

    impl Color for Grayscale {
        fn red(&self) -> u8 {
            self.scale
        }

        fn green(&self) -> u8 {
            self.scale
        }

        fn blue(&self) -> u8 {
            self.scale
        }

        fn alpha(&self) -> u8 {
            255
        }
    }

    #[derive(Debug, Error)]
    pub enum ColorErr {
        #[error("invalid format")]
        Format,
        #[error(transparent)]
        ParseIntErr(#[from] std::num::ParseIntError),
    }

    impl Color for Rgba {
        fn red(&self) -> u8 {
            self.red
        }

        fn green(&self) -> u8 {
            self.green
        }

        fn blue(&self) -> u8 {
            self.blue
        }

        fn alpha(&self) -> u8 {
            self.alpha
        }
    }

    impl std::convert::From<Rgba> for pixels::wgpu::Color {
        fn from(val: Rgba) -> Self {
            pixels::wgpu::Color {
                r: val.red as f64,
                g: val.green as f64,
                b: val.blue as f64,
                a: val.alpha as f64,
            }
        }
    }

    impl FromStr for Rgba {
        type Err = ColorErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.len() {
                4 => {
                    let r = u8::from_str_radix(&s[1..2], 16)?;
                    let g = u8::from_str_radix(&s[2..3], 16)?;
                    let b = u8::from_str_radix(&s[3..4], 16)?;

                    Ok(Rgba {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: 255,
                    })
                }
                7 | 9 => {
                    let r = u8::from_str_radix(&s[1..3], 16)?;
                    let g = u8::from_str_radix(&s[3..5], 16)?;
                    let b = u8::from_str_radix(&s[5..7], 16)?;
                    let a = {
                        if s.len() == 9 {
                            u8::from_str_radix(&s[7..9], 16).unwrap_or(255)
                        } else {
                            255
                        }
                    };

                    Ok(Rgba {
                        red: r,
                        green: g,
                        blue: b,
                        alpha: a,
                    })
                }
                _ => Err(ColorErr::Format),
            }
        }
    }
}

pub mod frame {
    use std::str::FromStr;

    use super::color::Color;
    use super::color::Rgba;
    use pixels::Pixels;
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    pub struct Frame {
        width: u32,
        height: u32,
        pixels: Pixels,
    }

    pub fn default_window(
        width: u32,
        height: u32,
        title: &str,
        event_loop: &EventLoop<()>,
    ) -> winit::window::Window {
        let size = winit::dpi::LogicalSize::new(width, height);
        WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    pub fn default_event_loop() -> EventLoop<()> {
        EventLoop::new()
    }

    impl Frame {
        pub fn new(width: u32, height: u32, window: &winit::window::Window) -> Self {
            let surface = pixels::SurfaceTexture::new(width, height, &window);
            let pixels = pixels::PixelsBuilder::new(width, height, surface)
                .clear_color(pixels::wgpu::Color::BLACK)
                .wgpu_backend(pixels::wgpu::Backends::GL)
                .build()
                .unwrap();
            Frame {
                width,
                height,
                pixels,
            }
        }

        pub fn render(&self) -> Result<(), pixels::Error> {
            self.pixels.render()
        }

        pub fn resize_surface(
            &mut self,
            width: u32,
            height: u32,
        ) -> Result<(), pixels::TextureError> {
            self.pixels.resize_surface(width, height)
        }

        pub fn resize_buffer(
            &mut self,
            width: u32,
            height: u32,
        ) -> Result<(), pixels::TextureError> {
            self.pixels.resize_buffer(width, height)?;
            self.width = width;
            self.height = height;
            Ok(())
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

        pub fn pixels(&self) -> &Pixels {
            &self.pixels
        }

        pub fn pixels_mut(&mut self) -> &mut Pixels {
            &mut self.pixels
        }

        pub fn get_rgba(&self, idx: (usize, usize)) -> Option<Rgba> {
            let index = idx.0 + self.width as usize * idx.1;
            let c = self.pixels.frame().chunks_exact(4).nth(index - 1);
            c.map(|v| Rgba {
                red: v[0],
                green: v[1],
                blue: v[2],
                alpha: v[3],
            })
        }

        pub fn set(&mut self, width: usize, height: usize, color: &impl Color) {
            let index = width + self.width as usize * height;
            let c = self.pixels.frame_mut().chunks_exact_mut(4).nth(index);
            match c {
                Some(v) => {
                    v[0] = color.red();
                    v[1] = color.green();
                    v[2] = color.blue();
                    v[3] = color.alpha();
                }
                None => panic!("out of bounds"),
            }
        }

        pub fn set_by_index(&mut self, index: usize, color: impl Color) {
            let c = self.pixels.frame_mut().chunks_exact_mut(4).nth(index);
            match c {
                Some(v) => {
                    v[0] = color.red();
                    v[1] = color.green();
                    v[2] = color.blue();
                    v[3] = color.alpha();
                }
                None => panic!("out of bounds"),
            }
        }
    }

    pub struct FrameBuilder<'a> {
        width: u32,
        height: u32,
        pixels: Option<pixels::Pixels>,
        surface: Option<pixels::SurfaceTexture<'a, winit::window::Window>>,
        event_loop: Option<&'a EventLoop<()>>,
        background: Option<Rgba>,
    }

    #[derive(thiserror::Error, Debug)]
    #[non_exhaustive]
    pub enum FrameBuilderErr {
        #[error("missing surface")]
        MissingSurface,
    }

    impl<'a> FrameBuilder<'a> {
        pub fn new(width: u32, height: u32) -> Self {
            FrameBuilder {
                width,
                height,
                pixels: None,
                surface: None,
                event_loop: None,
                background: None,
            }
        }

        pub fn with_event_loop(mut self, event_loop: &'a EventLoop<()>) -> Self {
            self.event_loop = Some(event_loop);
            self
        }

        pub fn with_surface(
            mut self,
            surface: pixels::SurfaceTexture<'a, winit::window::Window>,
        ) -> Self {
            self.surface = Some(surface);
            self
        }

        pub fn with_background(mut self, color: impl Color) -> Self {
            self.background = Some(Rgba {
                red: color.red(),
                green: color.green(),
                blue: color.blue(),
                alpha: color.alpha(),
            });
            self
        }

        pub fn build(self) -> Result<Frame, FrameBuilderErr> {
            let surface = self.surface.ok_or(FrameBuilderErr::MissingSurface)?;

            Ok(Frame {
                width: self.width,
                height: self.height,
                pixels: match self.pixels {
                    Some(v) => v,
                    None => pixels::PixelsBuilder::new(self.width, self.height, surface)
                        .clear_color(
                            self.background
                                .unwrap_or_else(|| Rgba::from_str("#000").unwrap())
                                .into(),
                        )
                        .wgpu_backend(pixels::wgpu::Backends::GL)
                        .build()
                        .unwrap(),
                },
            })
        }
    }
}

pub mod utils {
    use std::ops::{Add, Div, Mul, Sub};

    pub fn remap_value<T>(value: T, from: (T, T), to: (T, T)) -> T
    where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Copy
    {
        to.0 + (value - from.0) * (to.1 - to.0) / (from.1 - from.0)
    }
}
