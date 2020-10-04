#[derive(Debug, Clone)]
pub struct Asset {
    pub data: Vec<u8>,
    pub name: String,
    pub asset_type: AssetType,
}

#[derive(Debug, Copy, Clone)]
pub enum AssetType {
    GlslVertexShader,
    GlslFragmentShader,
    PngImage,
}
