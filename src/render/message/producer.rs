use bounded_spsc_queue::Producer;
use cgmath::*;
use render::color::*;
use render::message::*;
use std::mem;
use utility::slotmap::*;

pub struct RenderProducer {
    render_producer: Producer<RenderFrame>,
    frame: RenderFrame,
    map_rect: IndexMap,
    map_triangle: IndexMap,
}

impl RenderProducer {
    pub fn new(render_producer: Producer<RenderFrame>) -> RenderProducer {
        RenderProducer {
            render_producer: render_producer,
            frame: RenderFrame::new(),
            map_rect: IndexMap::new(),
            map_triangle: IndexMap::new(),
        }
    }

    pub fn create_rect(&mut self, pos: Vector2<f32>, size: Vector2<f32>, color: Color) -> IndexToken {
        let message = QuadMessage::Create {
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.quads.push(message);
        self.map_rect.add()
    }

    pub fn update_rect(&mut self, token: &IndexToken, pos: Vector2<f32>, size: Vector2<f32>, color: Color) {
        let message = QuadMessage::Update {
            id: self.map_rect.get(token),
            pos: pos,
            size: size,
            color: color,
        };
        self.frame.quads.push(message);
    }

    pub fn remove_rect(&mut self, token: IndexToken) {
        let message = QuadMessage::Remove {
            id: self.map_rect.remove(token),
        };
        self.frame.quads.push(message);
    }

    pub fn create_triangle(&mut self, pos: Vector2<f32>, height: f32, color: Color) -> IndexToken {
        let message = TriangleMessage::Create {
            pos: pos,
            height: height,
            color: color,
        };
        self.frame.triangles.push(message);
        self.map_triangle.add()
    }

    pub fn update_triangle(&mut self, token: &IndexToken, pos: Vector2<f32>, height: f32, color: Color) {
        let message = TriangleMessage::Update {
            id: self.map_triangle.get(token),
            pos: pos,
            height: height,
            color: color,
        };
        self.frame.triangles.push(message);
    }

    pub fn remove_triangle(&mut self, token: IndexToken) {
        let message = TriangleMessage::Remove {
            id: self.map_triangle.remove(token),
        };
        self.frame.triangles.push(message);
    }

    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        let message = SetTranslationMessage {
            translation: translation,
        };
        self.frame.translation = Some(message);
    }

    pub fn send(&mut self) {
        let mut frame = RenderFrame::new();
        mem::swap(&mut frame, &mut self.frame);
        self.render_producer.push(frame);
    }
}
