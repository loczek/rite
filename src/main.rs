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
    bottom: f32,
    left: f32,
    height: f32,
    width: f32,
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().expect("Unable to create event loop");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(800, 400)
        .with_title("Rite")
        .build(&event_loop);

    let bitmap = BitmapFont::new(&display);

    let renderer = TextRenderer::new(&bitmap);

    let mut content = include_str!("./samples/sample.js")
        .to_string()
        .chars()
        .filter(|x| *x != '\r')
        .collect::<String>();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(
        &display,
        include_str!("./shaders/texture.vert"),
        include_str!("./shaders/texture.frag"),
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

    let padding = 16.0;

    let mut curr_cursor_x = 0.0;
    let mut curr_cursor_y = 0.0;

    let mut cursor_x = 0;
    let mut cursor_y = 0;

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
                                keyboard::NamedKey::ArrowRight => {
                                    let lines = content
                                        .lines()
                                        .map(|line| line.len())
                                        .collect::<Vec<usize>>();

                                    if cursor_x < lines[cursor_y] {
                                        cursor_x += 1
                                    }
                                }
                                keyboard::NamedKey::ArrowLeft => {
                                    if cursor_x > 0 {
                                        cursor_x -= 1
                                    }
                                }
                                keyboard::NamedKey::ArrowDown => {
                                    let lines = content.lines().count();
                                    if cursor_y < lines {
                                        cursor_y += 1
                                    }
                                }
                                keyboard::NamedKey::ArrowUp => {
                                    if cursor_y > 0 {
                                        cursor_y -= 1
                                    }
                                }
                                keyboard::NamedKey::Space => {
                                    content.insert(cursor_x, ' ');
                                    cursor_x += 1;
                                }
                                keyboard::NamedKey::Enter => {
                                    content.insert(cursor_x, '\n');
                                }
                                _ => return,
                            },
                            keyboard::Key::Character(characters) => {
                                let mut idx = 0;

                                let mut offset_y = cursor_y;
                                let chars = content.chars().collect::<Vec<_>>();

                                while offset_y > 0 {
                                    if chars[idx] == '\n' {
                                        offset_y -= 1;
                                    }

                                    idx += 1;
                                }

                                for char in characters.chars() {
                                    content.insert(idx + cursor_x, char)
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
                        window.inner_size().height as f32 - bitmap.ascent - padding,
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

                    curr_cursor_x = lerp(curr_cursor_x, cursor_x as f32, 0.1);
                    curr_cursor_y = lerp(curr_cursor_y, cursor_y as f32, 0.1);

                    let cursor_rect = Rectangle {
                        bottom: window.inner_size().height as f32 - bitmap.ascent + bitmap.descent
                            - padding
                            - (24.0 * curr_cursor_y as f32),
                        left: (12.0 * curr_cursor_x as f32) + padding,
                        height: 24.0,
                        width: 2.0,
                    };

                    let cursor_shape = ColorVertex::from(cursor_rect, [1.0, 1.0, 1.0]);

                    let ascent_rect: Rectangle = Rectangle {
                        bottom: window.inner_size().height as f32 - bitmap.ascent + bitmap.ascent
                            - padding,
                        left: padding,
                        height: 1.0,
                        width: 200.0,
                    };

                    let ascent_shape = ColorVertex::from(ascent_rect, [1.0, 0.0, 0.0]);

                    let baseline_rect: Rectangle = Rectangle {
                        bottom: window.inner_size().height as f32 - bitmap.ascent - padding,
                        left: padding,
                        height: 1.0,
                        width: 200.0,
                    };

                    let baseline_shape = ColorVertex::from(baseline_rect, [0.0, 1.0, 0.0]);

                    let descent_rect: Rectangle = Rectangle {
                        bottom: window.inner_size().height as f32 - bitmap.ascent + bitmap.descent
                            - padding,
                        left: padding,
                        height: 1.0,
                        width: 200.0,
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
