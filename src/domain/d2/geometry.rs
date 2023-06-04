use std::f32::consts::PI;

use glam::Vec4;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, IndexFormat, RenderPass,
};

use crate::context::Context;

pub struct GeometryBuffer {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl GeometryBuffer {
    pub fn draw<'a>(&'a self, rpass: &'a mut RenderPass<'a>) {
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
        rpass.draw_indexed(0..(self.index_buffer.size() as u32 / 4), 0, 0..1);
    }
}

pub struct Geometry {
    pub vertexes: Vec<Vec4>,
    pub indexes: Vec<u32>,
}

impl Geometry {
    pub fn buffer(&self, ctx: &Context) -> GeometryBuffer {
        let shared_context = ctx.0.read().unwrap();

        GeometryBuffer {
            vertex_buffer: shared_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.vertexes[..]),
                    usage: wgpu::BufferUsages::VERTEX,
                }),

            index_buffer: shared_context
                .device
                .create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.indexes[..]),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        }
    }
}

pub fn circle(res: u32) -> Geometry {
    let mut vertexes: Vec<Vec4> = vec![];
    let mut indexes: Vec<u32> = vec![];

    vertexes.push(Vec4::new(0., 0., 0., 1.));

    let ph = (PI * 2.) / (res as f32);
    let mut th = 0.;

    for i in 1..=res {
        // let x = f32::cos(th) * 0.5;
        // let y = f32::sin(th) * 0.5;

        let x = f32::cos(th) * 200.;
        let y = f32::sin(th) * 200.;
        vertexes.push(Vec4::new(x, y, 0., 1.));

        indexes.push(0);
        indexes.push(i);
        indexes.push(i + 1);

        th += ph;
    }

    indexes.pop();
    indexes.push(1);

    Geometry { vertexes, indexes }
}
