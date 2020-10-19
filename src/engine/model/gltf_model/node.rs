use super::{data::InitData, primitive::Primitive, Matrix4, ModelRenderContext, ModelRenderData};
use crate::engine::prelude::*;

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

    pub fn render(&self, context: &mut ModelRenderContext, render_data: &ModelRenderData) {
        let render_data = ModelRenderData {
            model_matrix: &(render_data.model_matrix * self.transform),
            ..*render_data
        };
        for primitive in self.primitives.iter() {
            primitive.render(context, &render_data);
        }
        for child in self.children.iter() {
            child.render(context, &render_data);
        }
    }
}
