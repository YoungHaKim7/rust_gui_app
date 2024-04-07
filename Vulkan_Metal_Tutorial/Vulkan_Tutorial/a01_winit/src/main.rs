use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone, Default)]
struct AppData {}

#[derive(Clone, Debug)]
struct App {}

impl App {
    unsafe fn create(window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {}
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    // Window
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Vulkan tutorial (Rust)")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    // App
    //
    let mut app = unsafe { App::create(&window)? };
    event_loop.run(move |event, elwt| match event {
        Event::AboutToWait => window.request_redraw(),
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::RedrawRequested if !elwt.exiting() => {
                unsafe { app.render(&window) }.unwrap()
            }
            WindowEvent::CloseRequested => {
                elwt.exit();
                unsafe {
                    app.destroy();
                }
            }
            _ => {}
        },
        _ => {}
    })?;

    Ok(())
}
