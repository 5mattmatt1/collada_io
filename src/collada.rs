struct Collada
{
    effects: Option<Vec<fx::effect::Effect>>;
    materials: Option<Vec<fs::material::Material>>;
    geometries: Option<Vec<geometry::Geometry>>; 
}