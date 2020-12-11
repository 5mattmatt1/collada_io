use std::io::Write;
use xml::writer::{EventWriter, Result};
use xml::attribute::Attribute;
use crate::util::*;

pub struct Vertex
{

}

// Maybe more of a XML Triangles object and the more library friendly version just called Triangles???
pub struct Triangles
{
    pub vertices: String,
    pub tex_vertices: Option<String>,
    pub normals: Option<String>,
    pub primitive: Option<Vec<usize>>,
    pub material: Option<String>,
}

// Used by Param element
pub enum ArrayType
{
    Int,
    Float,
    Name,
    Bool,
    Idref,
    Sidref
}

// Does source even matter?
// It seems like it would always refer to it's parent source...
pub struct Accessor
{
    // Assuming that the param types would be
    // the same as the data array in the Parent Source
    // and that source will be the same as the ID of the parent source...
    pub params: Vec<String>
}

pub struct FloatArray
{
    pub id: String, 
    pub data: Vec<f64>,
}

// Int, Float, Name, Sidref, Idref
pub enum DataArray
{

}

pub enum Technique
{
    Common
}

pub struct Source
{
    pub id: String,
    pub float_array: FloatArray,
    pub accessor: Accessor
}

pub struct Vertices
{
    pub id: String,
    pub name: Option<String>,
    pub source: String
}

pub struct ControlVertices
{
    pub id: String,
    pub name: Option<String>,
    pub source: String    
}

pub struct Spline
{
    pub sources: Vec<Source>,
    // Need to change this to just use vertices and have the start and end element of vertices
    // to be written up above it by either spline or Mesh
    // and maybe even do the same thing Mesh and Spline themselves because the data they store is the same
    // structure...
    pub control_vertices: ControlVertices 
}

pub struct Mesh
{
    pub sources: Vec<Source>,
    pub vertices: Vertices,  
    pub triangles: Triangles  
}

pub enum GeometricElement
{
    Mesh(Mesh),
    Spline(Spline)
}

pub struct Geometry
{
    pub id: Option<String>,
    pub name: Option<String>,
    pub mesh: Mesh,
}

impl FloatArray
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {   
        let count_str = self.data.len().to_string();
        let attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "id".into(),
                value: &self.id
            },
            Attribute {
                name: "count".into(),
                value: &count_str
            }
        };
        
        write_vec_element(w, "float_array", &self.data, &attributes)?;
        Ok(())
    } 
}

impl Source
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        let attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "id".into(),
                value: &self.id
            }
        };
        let source_str = "#".to_string() + &self.float_array.id;
        let stride = self.accessor.params.len();
        let accessor_count = self.float_array.data.len() / stride;
        let stride_str = stride.to_string();
        let accessor_count_str = accessor_count.to_string();
        let accessor_attrib: Vec<Attribute> = vec!{
            Attribute {
                name: "source".into(),
                value: &source_str
            },
            Attribute {
                name: "count".into(),
                value: &accessor_count_str
            },
            Attribute {
                name: "stride".into(),
                value: &stride_str
            },
        };
        write_start_element(w, "source", &attributes)?;
        self.float_array.write(w)?;
        // Only supporting technique common at the moment
        write_start_element(w, "technique_common", &Vec::new())?;
        write_start_element(w, "accessor", &accessor_attrib)?;
        for param in &self.accessor.params
        {
            let param_attrib: Vec<Attribute> = vec!{
                Attribute {
                    name: "name".into(),
                    value: &param 
                },
                Attribute {
                    name: "type".into(),
                    value: "float" // Currently only using floats
                }
            };
            write_start_element(w, "param", &param_attrib)?;
            write_end_element(w, "param")?;
        }
        write_end_element(w, "accessor")?;
        write_end_element(w, "technique_common")?;
        write_end_element(w, "source")?;
        Ok(())
    }
}

impl Vertices
{

    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {        
        let attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "id".into(),
                value: &self.id
            }
        };
        let input_attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "semantic".into(),
                value: "POSITION"
            },
            Attribute {
                name: "source".into(),
                value: &self.source
            }
        };
        write_start_element(w, "vertices", &attributes)?;
        write_start_element(w, "input", &input_attributes)?;
        write_end_element(w, "input")?;
        write_end_element(w, "vertices")?;
        Ok(())
    }
}

impl Triangles
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {      
        let mut attributes: Vec<Attribute> = Vec::new();  
        match &self.material
        {
            Some(material) => {
                attributes.push(Attribute {
                    name: "material".into(),
                    value: material
                });
            }, None => {}
        }
        
        let mut num_params = 1;
        let mut normals_offset = 0;
        let mut tex_offset = 0;
        let mut count = 0;        

        match &self.normals
        {
            Some(_) => {
                normals_offset = num_params;
                num_params += 1;
            }, None => {}
        }

        match &self.tex_vertices
        {
            Some(_) => {
                tex_offset = num_params;
                num_params += 1;
            }, None => {}
        }

        match &self.primitive
        {
            Some(primitive) => {
                count = primitive.len() / (3 * num_params);

            }, None => {}
        }

        let count_str = count.to_string();
        attributes.push(Attribute {
            name: "count".into(),
            value: &count_str
        });

        write_start_element(w, "triangles", &attributes)?;
        let vertices_attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "semantic".into(),
                value: "VERTEX"
            },
            Attribute {
                name: "source".into(),
                value: &self.vertices
            },
            Attribute {
                name: "offset".into(),
                value: "0"
            }
        };
        write_start_element(w, "input", &vertices_attributes)?;
        write_end_element(w, "input")?;
        match &self.normals
        {
            Some(normals) => {
                let offset_str = normals_offset.to_string();
                let input_attributes: Vec<Attribute> = vec!{
                    Attribute {
                        name: "semantic".into(),
                        value: "NORMAL"
                    },
                    Attribute {
                        name: "source".into(),
                        value: &normals
                    },
                    Attribute {
                        name: "offset".into(),
                        value: &offset_str
                    }
                };
                write_start_element(w, "input", &input_attributes)?;
                write_end_element(w, "input")?;
            }, None => {}
        }

        match &self.tex_vertices
        {
            Some(tex_vertices) => {
                let offset_str = tex_offset.to_string();
                let input_attributes: Vec<Attribute> = vec!{
                    Attribute {
                        name: "semantic".into(),
                        value: "TEXCOORD"
                    },
                    Attribute {
                        name: "source".into(),
                        value: &tex_vertices
                    },
                    Attribute {
                        name: "offset".into(),
                        value: &offset_str
                    },
                    Attribute {
                        name: "set".into(),
                        value: "0"
                    }
                };
                write_start_element(w, "input", &input_attributes)?;
                write_end_element(w, "input")?;
            }, None => {}
        }

        match &self.primitive
        {
            Some(primitive) => {
                write_vec_element(w, "p", primitive, &Vec::new())?;

            }, None => {}
        }

        write_end_element(w, "triangles")?;

        Ok(())
    }
}

impl Mesh
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {        
        write_start_element(w, "mesh", &Vec::new())?;

        for source in &self.sources 
        { 
            source.write(w)?;
        }
        self.vertices.write(w)?;
        self.triangles.write(w)?;
        write_end_element(w, "mesh")?;

        Ok(())
    }
}

impl Geometry
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        let mut attributes: Vec<Attribute> = Vec::new();

        match &self.id
        {
            Some(id) => {
                attributes.push(Attribute {
                    name: "id".into(),
                    value: &id
                });
            }, None => {}
        }

        match &self.name
        {
            Some(name) => {
                attributes.push(Attribute {
                    name: "name".into(),
                    value: &name
                });
            }, None => {}
        }

        write_start_element(w, "geometry", &attributes)?;
        self.mesh.write(w)?;
        write_end_element(w, "geometry")?;

        Ok(())
    }
}