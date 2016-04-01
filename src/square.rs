extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::PrimitiveType::TrianglesList;

use cell::Cell;

pub fn geometry(display: &Display) -> (VertexBufferAny, IndexBuffer<u16>) {
    (vertices(display), indices(display))
}

fn vertices(display: &Display) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        vertex_position: [f32; 2],
        vertex_color: [f32; 3],
    }

    implement_vertex!(Vertex, vertex_position, vertex_color);

    let colour = [0.2, 0.2, 0.2];

    VertexBuffer::new(display,
        &vec![
            Vertex { vertex_position: [ -0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5, -0.5], vertex_color: colour },
            Vertex { vertex_position: [ -0.5, -0.5], vertex_color: colour },
        ]
    ).unwrap().into_vertex_buffer_any()
}

fn indices(display: &Display) -> IndexBuffer<u16> {
    IndexBuffer::new(display, TrianglesList,&vec![0u16, 1, 2, 0, 2, 3]).unwrap()
}

#[derive(Copy, Clone)]
struct ModelTransform {
    model_position: [f32; 2],
    model_scale: f32,
}

pub fn instances(display: &Display, grid: &Vec<Cell>) -> VertexBufferAny {
    implement_vertex!(ModelTransform, model_position, model_scale);

    let mut data = Vec::new();
    data.reserve_exact(grid.len());
    for cell in grid.iter() {
        if cell.alive {            
            data.push(ModelTransform {
                model_position: [cell.x, cell.y],
                model_scale: cell.scale
            });
        }
    }
    VertexBuffer::new(display, &data).unwrap().into_vertex_buffer_any()
}
