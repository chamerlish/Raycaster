extern crate glium;
// Use the re-exported winit dependency to avoid version mismatches.
// Requires the `simple_window_builder` feature.
use glium::{Surface, winit};
use winit::application::ApplicationHandler;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

glium::implement_vertex!(Vertex, position, color);


struct App{
    window: Option<winit::window::Window>,
    display: Option<glium::Display<glium::glutin::surface::WindowSurface>>,
    vertex_buffer: Option<glium::VertexBuffer<Vertex>>,
    program: Option<glium::Program>
}

impl App {
    pub fn new() -> Self {
        Self { 
            window: None, 
            display: None, 
            vertex_buffer: None, 
            program: None 
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("triagles type shit")
            .with_inner_size(800, 600)
            .build(event_loop);

        let vertices = [
            Vertex {
                position: [-0.5, -0.5],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(
            &display,
            &vertices
        ).unwrap();

        

        let vertex_shader: &str = include_str!("vertex.glsl");
        let fragment_shader = include_str!("fragment.glsl");

        let program = glium::Program::from_source(
            &display, 
            vertex_shader, 
            fragment_shader, 
            None).unwrap();
        
        self.window = Some(window);
        self.display = Some(display);
        self.vertex_buffer = Some(vertex_buffer);
        self.program = Some(program);

    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            winit::event::WindowEvent::RedrawRequested => {
                let display = self.display.as_ref().unwrap();
                let vertex_buffer = self.vertex_buffer.as_ref().unwrap();
                let program = self.program.as_ref().unwrap();

                let mut frame = display.draw();
                
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                let indecies = glium::index::NoIndices(
                    glium::index::PrimitiveType::TrianglesList
                );
                frame.draw(
                    vertex_buffer, 
                    indecies, 
                    program, 
                    &glium::uniforms::EmptyUniforms,
                    &glium::DrawParameters::default()).unwrap();
                
                frame.finish().unwrap();
            }
        _ => {}
            }
        
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}


fn main() {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .unwrap();

    let mut app: App = App::new();

    event_loop.run_app(&mut app).unwrap();
    
}