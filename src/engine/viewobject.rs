pub trait ViewObject {
    fn get_bind_group(&self) -> &wgpu::BindGroup;

    fn get_layout(&self) -> &wgpu::BindGroupLayout;

    fn get_indexed_bind_group(&self, index: u32) -> Option<&wgpu::BindGroup> {
        if index == 0 {
            Some(self.get_bind_group())
        } else {
            None
        }
    }
}
