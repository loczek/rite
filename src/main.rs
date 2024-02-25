mod font;
mod rope;
mod scalable;
mod text;
mod vertex;

use font::BitmapFont;
use glium::{implement_vertex, uniform, Surface};
use text::TextRenderer;
use vertex::ColorVertex;
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

    let color_program = glium::Program::from_source(
        &display,
        include_str!("./shaders/color.vert"),
        include_str!("./shaders/color.frag"),
        None,
    )
    .unwrap();

    let padding = 32;

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
                        padding,
                        (window.inner_size().height - bitmap.ascent as u32) as i32 - padding,
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

                    let cursor_rect = Rectangle {
                        bottom: (window.inner_size().height - bitmap.ascent as u32) as i32
                            + bitmap.descent
                            - padding,
                        left: (12 * cursor_x as i32) + padding,
                        height: 24,
                        width: 1,
                    };

                    let cursor_shape = ColorVertex::from(cursor_rect, [1.0, 1.0, 1.0]);

                    let ascent_rect: Rectangle = Rectangle {
                        bottom: (window.inner_size().height - bitmap.ascent as u32) as i32
                            + bitmap.ascent
                            - padding,
                        left: padding,
                        height: 1,
                        width: 200,
                    };

                    let ascent_shape = ColorVertex::from(ascent_rect, [1.0, 0.0, 0.0]);

                    let baseline_rect: Rectangle = Rectangle {
                        bottom: (window.inner_size().height - bitmap.ascent as u32) as i32
                            - padding,
                        left: padding,
                        height: 1,
                        width: 200,
                    };

                    let baseline_shape = ColorVertex::from(baseline_rect, [0.0, 1.0, 0.0]);

                    let descent_rect: Rectangle = Rectangle {
                        bottom: (window.inner_size().height - bitmap.ascent as u32) as i32
                            + bitmap.descent
                            - padding,
                        left: padding,
                        height: 1,
                        width: 200,
                    };

                    let descent_shape = ColorVertex::from(descent_rect, [0.0, 0.0, 1.0]);

                    let mut combined_shape =
                        [cursor_shape, ascent_shape, baseline_shape, descent_shape].concat();

                    let cursor_shape_iter = combined_shape.iter_mut();

                    for vert in cursor_shape_iter {
                        scalable::rescale_position(
                            vert,
                            window.inner_size().height as f32,
                            window.inner_size().width as f32,
                        );
                    }

                    let vertex_buffer_cursor =
                        glium::VertexBuffer::new(&display, &combined_shape).unwrap();

                    target
                        .draw(
                            &vertex_buffer_cursor,
                            &indices,
                            &color_program,
                            &glium::uniforms::EmptyUniforms,
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
