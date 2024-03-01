mod cursor;
mod font;
mod render;
mod scalable;
mod text;
mod utils;
mod vertex;

use cursor::Cursor;
use font::BitmapFont;
use glium::{uniform, Surface, VertexBuffer};
use render::Application;
use text::TextRenderer;
use utils::interpolation::lerp;
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

    let mut cursor = Cursor::new();


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
                                    if cursor.idx > 0 {
                                        cursor.move_left(&content);
                                        content.remove(cursor.idx);
                                    }
                                }
                                keyboard::NamedKey::ArrowRight => {
                                    cursor.move_right(&content);
                                }
                                keyboard::NamedKey::ArrowLeft => {
                                    cursor.move_left(&content);
                                }
                                keyboard::NamedKey::ArrowDown => {
                                    cursor.move_down(&content);
                                }
                                keyboard::NamedKey::ArrowUp => {
                                    cursor.move_up(&content);
                                }
                                keyboard::NamedKey::Space => {
                                    content.insert(cursor.idx, ' ');
                                    cursor.move_right(&content);
                                }
                                keyboard::NamedKey::Enter => {
                                    content.insert(cursor.idx, '\n');
                                    cursor.move_right(&content);
                                }
                                _ => return,
                            },
                            keyboard::Key::Character(characters) => {
                                for char in characters.chars() {
                                    content.insert(cursor.idx, char);
                                    cursor.move_right(&content);
                                }
                            }
                            _ => return,
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    let mut target = display.draw();

                    target.clear_color(2.0 / 255.0, 2.0 / 255.0, 2.0 / 255.0, 1.0);
                    // target.clear_color(1.0, 1.0, 1.0, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0 ,0.0, 0.0, 1.0f32],
                        ],
                        tex: &bitmap.texture,
                    };

                    curr_cursor_x = lerp(curr_cursor_x, cursor.cursor_x as f32, 0.1);
                    curr_cursor_y = lerp(curr_cursor_y, cursor.cursor_y as f32, 0.1);

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

                    let counter_shape =
                        renderer.render(&mut content.len().to_string(), padding, padding, &window);

                    let counter_vertex_buffer =
                        VertexBuffer::new(&display, &counter_shape).unwrap();

                    target
                        .draw(
                            &counter_vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();

                    let cursor_rect = Rectangle {
                        bottom: window.inner_size().height as f32 - bitmap.ascent + bitmap.descent
                            - padding
                            - (curr_cursor_y * 24.0),
                        left: padding + (curr_cursor_x * 12.0),
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
