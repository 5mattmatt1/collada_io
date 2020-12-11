use std::io::Write;
use xml::writer::{EventWriter, Result};
use xml::attribute::Attribute;
use crate::util::*;
use crate::io::XmlWrite;

pub enum TransformationElement
{
    Lookat {

    },
    Matrix {
        sid: String,
        matrix: Vec<f64> // 4x4 suitable for matrix composition
    },
    Rotate {

    },
    Scale {

    },
    Skew {

    },
    Translate {

    }
}

pub struct BindVertexIndex
{
    pub semantic: String,
    pub input_semantic: String,
    pub input_set: Option<usize>
}

pub struct InstanceMaterial
{
    pub sid: Option<String>,
    pub name: Option<String>,
    pub target: String,
    pub symbol: String,

    // pub bind: Option<Bind>, // (fx)
    pub bind_vertex_indices: Vec<BindVertexIndex>
}

pub struct BindMaterial 
{
    pub instance_materials: Vec<InstanceMaterial>
}

pub enum Instance
{
    Geometry {
        url: String,
        name: Option<String>,
        sid: Option<String>,
        bind_material: Option<BindMaterial>
    }
}

pub struct Node
{
    pub id: String,
    pub name: String,
    pub transformation_elements: Vec<TransformationElement>,
    pub instances: Vec<Instance>
}

pub struct VisualScene
{
    pub id: String,
    pub name: String,
    pub nodes: Vec<Node>
}

pub struct Scene
{
    pub visual_scenes: Vec<String>
}

impl XmlWrite for TransformationElement
{
    fn write<W>(&self, w: &mut EventWriter<W>) -> Result<()> 
    where W: Write 
    {
        match self
        {
            Self::Matrix {
                sid, 
                matrix
            } => {
                let attributes: Vec<Attribute> = vec!{
                    Attribute {
                        name: "sid".into(),
                        value: &sid
                    },
                };
                write_vec_element(w, "matrix", &matrix, &attributes)?;
            },
            _ => {
                unimplemented!()
            }
        }
        Ok(())
    }
}

impl XmlWrite for Instance
{
    fn write<W>(&self, w: &mut EventWriter<W>) -> Result<()> 
    where W: Write 
    {
        match self
        {
            Self::Geometry {
                url,
                name: opt_name, 
                sid: _sid,
                bind_material: _bind_material
            } => {
                let mut attributes = vec!{
                    Attribute {
                        name: "url".into(),
                        value: url
                    }
                };

                match opt_name
                {
                    Some(name) => {
                        attributes.push(Attribute {
                            name: "name".into(),
                            value: name
                        });
                    }, None => {

                    }
                }
                write_start_element(w, "instance_geometry", &attributes)?;
                // TODO: Bind Material (fx)
                write_end_element(w, "instance_geometry")?;
            }
        }
        Ok(())
    }   
}

impl XmlWrite for Node
{
    fn write<W>(&self, w: &mut EventWriter<W>) -> Result<()> 
    where W: Write 
    {
        let attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "id".into(),
                value: &self.id
            },
            Attribute {
                name: "name".into(),
                value: &self.name
            },
            Attribute {
                name: "type".into(),
                value: "NODE"
            }
        };
        write_start_element(w, "node", &attributes)?;
        for transformation_element in &self.transformation_elements
        {
            transformation_element.write(w)?;
        }
        for instance in &self.instances
        {
            instance.write(w)?;
        }
        write_end_element(w, "node")?;
        Ok(()) 
    }   
}

impl XmlWrite for VisualScene
{
    fn write<W>(&self, w: &mut EventWriter<W>) -> Result<()> 
    where W: Write 
    { 
        let attributes: Vec<Attribute> = vec!{
            Attribute {
                name: "id".into(),
                value: &self.id
            },
            Attribute {
                name: "name".into(),
                value: &self.name
            }
        };
        write_start_element(w, "visual_scene", &attributes)?;
        for node in &self.nodes
        {
            node.write(w)?;
        }
        write_end_element(w, "visual_scene")?;

        Ok(())
    }
}

impl XmlWrite for Scene
{
    fn write<W>(&self, w: &mut EventWriter<W>) -> Result<()> 
    where W: Write 
    { 
        write_start_element(w, "scene", &Vec::new())?;
        for visual_scene in &self.visual_scenes
        {
            let attributes: Vec<Attribute> = vec!{
                Attribute {
                    name: "url".into(),
                    value: &visual_scene
                }
            };
            write_start_element(w, "instance_visual_scene", &attributes)?;
            write_end_element(w, "instance_visual_scene")?;
        }
        write_end_element(w, "scene")?;
        Ok(())
    }
}