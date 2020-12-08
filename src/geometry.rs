pub type VertexIndex = usize;
pub type TextureIndex = usize;
pub type NormalIndex = usize;

struct Vertex
{

}

struct Triangles
{
    vertices: Vec<(VertexIndex, VertexIndex, VertexIndex)>;
    tex_vertices: Option<Vec<(TextureIndex, TextureIndex, TextureIndex)>>;
    normals: Option<Vec<(NormalIndex, NormalIndex, NormalIndex)>>;
    material: Option<String>;
}