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

    pub fn default_window(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> winit::window::Window {
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

        pub fn width(&self) -> &u32 {
            &self.width
        }

        pub fn height(&self) -> &u32 {
            &self.height
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

        pub fn set<T: Color>(&mut self, width: usize, height: usize, color: &T) {
            let index = width + self.width as usize * height;
            let c = self.pixels.frame_mut().chunks_exact_mut(4).nth(index);
            match c {
                Some(v) => {
                    v[0] = color.red();
                    v[1] = color.green();
                    v[2] = color.blue();
                    v[3] = color.alpha();
                }
                None => panic!(),
            }
        }
    }

    pub struct FrameBuilder<'a> {
        width: u32,
        height: u32,
        pixels: Option<pixels::Pixels>,
        surface: Option<pixels::SurfaceTexture<'a, winit::window::Window>>,
        event_loop: Option<&'a EventLoop<()>>,
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

        pub fn build(self) -> Result<Frame, FrameBuilderErr> {
            let surface = self.surface.ok_or(FrameBuilderErr::MissingSurface)?;

            Ok(Frame {
                width: self.width,
                height: self.height,
                pixels: match self.pixels {
                    Some(v) => v,
                    None => pixels::PixelsBuilder::new(self.width, self.height, surface)
                        .clear_color(pixels::wgpu::Color::BLACK)
                        .wgpu_backend(pixels::wgpu::Backends::GL)
                        .build()
                        .unwrap(),
                },
            })
        }
    }
}
