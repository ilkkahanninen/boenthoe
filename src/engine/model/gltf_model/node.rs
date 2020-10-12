use super::{data::InitData, primitive::Primitive, Matrix4, ModelRenderContext};
use crate::engine::Engine;

pub struct Node {
    transform: Matrix4,
    primitives: Vec<Primitive>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(engine: &Engine, node: &gltf::Node, data: &InitData) -> Self {
        Self {
            transform: node.transform().matrix().into(),
            primitives: node
                .mesh()
                .map(|mesh| {
                    mesh.primitives()
                        .map(|primitive| Primitive::new(engine, &primitive, data))
                        .collect()
                })
                .unwrap_or(Vec::new()),
            children: node
                .children()
                .map(|child| Node::new(engine, &child, data))
                .collect(),
        }
    }

    pub fn render(&self, context: &mut ModelRenderContext, transform: &Matrix4) {
        let global_transform = transform * self.transform;
        for primitive in self.primitives.iter() {
            primitive.render(context, &global_transform);
        }
        for child in self.children.iter() {
            child.render(context, &global_transform);
        }
    }
}
