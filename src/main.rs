mod font;
mod rope;
mod scalable;
mod text;

use font::BitmapFont;
use glium::{implement_vertex, uniform, Surface};
use text::TextRenderer;
use winit::{
    event::{ElementState, Event, WindowEvent},
    keyboard,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

extern crate glium;
extern crate winit;

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    bottom: i32,
    left: i32,
    height: u32,
    width: u32,
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    fn from(rect: Rectangle, texture_rect: Rectangle) -> Vec<Vertex> {
        let bottom = rect.bottom as f32;
        let left = rect.left as f32;
        let height = rect.height as f32;
        let width = rect.width as f32;

        let texture_bottom = texture_rect.bottom as f32;
        let texture_left = texture_rect.left as f32;
        let texture_width = texture_rect.width as f32;

        let shape = vec![
            Vertex {
                // top left
                position: [left, bottom + height],
                tex_coords: [texture_left, 0.0],
            },
            Vertex {
                // top right
                position: [left + width, bottom + height],
                tex_coords: [texture_left + texture_width, 0.0],
            },
            Vertex {
                // bottom right
                position: [left + width, bottom],
                tex_coords: [texture_left + texture_width, texture_bottom],
            },
            Vertex {
                // bottom right
                position: [left + width, bottom],
                tex_coords: [texture_left + texture_width, texture_bottom],
            },
            Vertex {
                // bottom left
                position: [left, bottom],
                tex_coords: [texture_left, texture_bottom],
            },
            Vertex {
                // top left
                position: [left, bottom + height],
                tex_coords: [texture_left, 0.0],
            },
        ];
        shape
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Unable to create event loop");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(800, 400)
        .with_title("Rite")
        .build(&event_loop);

    let bitmap = BitmapFont::new(&display);

    let renderer = TextRenderer::new(&bitmap);

    let mut content = include_str!("./samples/sample.js").to_string();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(
        &display,
        include_str!("./shaders/vertex.vert"),
        include_str!("./shaders/fragment.frag"),
        None,
    )
    .unwrap();

    let mut cursor_x = 0;

    event_loop
        .run(|ev, control_flow| match ev {
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.key_without_modifiers().as_ref() {
                            keyboard::Key::Named(key) => match key {
                                keyboard::NamedKey::Backspace => {
                                    if cursor_x > 0 {
                                        cursor_x -= 1;
                                        content.remove(cursor_x);
                                    }
                                }
                                keyboard::NamedKey::ArrowRight => cursor_x += 1,
                                keyboard::NamedKey::ArrowLeft => {
                                    if cursor_x > 0 {
                                        cursor_x -= 1
                                    }
                                }
                                keyboard::NamedKey::Space => {
                                    content.insert(cursor_x, ' ');
                                    cursor_x += 1;
                                }
                                _ => return,
                            },
                            keyboard::Key::Character(characters) => {
                                for char in characters.chars() {
                                    content.insert(cursor_x, char)
                                }
                                cursor_x += 1
                            }
                            _ => return,
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    let mut target = display.draw();

                    target.clear_color(0.005, 0.005, 0.005, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0 ,0.0, 0.0, 1.0f32],
                        ],
                        tex: &bitmap.texture,
                    };

                    let shape = renderer.render(
                        &mut content,
                        10,
                        (window.inner_size().height - 48) as i32,
                        &window,
                    );

                    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();

                    target.finish().unwrap();
                }
                WindowEvent::Resized(window_size) => display.resize(window_size.into()),
                WindowEvent::CloseRequested => control_flow.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
