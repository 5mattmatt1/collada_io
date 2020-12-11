use crate::geometry;
use crate::meta;
use crate::scene;

pub static COLLADA_XMLNS: &'static str = "http://www.collada.org/2005/11/COLLADASchema";
pub static COLLADA_XMLNS_XSI: &'static str = "http://www.w3.org/2001/XMLSchema-instance";
pub static COLLADA_VERSION: &'static str = "1.4.1";

use std::io::Write;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent, Result};
use xml::attribute::Attribute;
use xml::common::XmlVersion;
use crate::io::XmlWrite;
use crate::util::*;


pub struct Collada
{
    pub asset: meta::Asset,
    pub geometries: Option<Vec<geometry::Geometry>>,
    pub visual_scenes: Option<Vec<scene::VisualScene>>,
    pub scene: Option<scene::Scene>
}

impl Collada
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        w.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("utf-8"),
            standalone: None
        })?;

        let attributes = vec!{
            Attribute {
                name: "xmlns".into(),
                value: COLLADA_XMLNS
            },
            Attribute {
                name: "version".into(),
                value: COLLADA_VERSION
            },
            Attribute {
                name: "xmlns:xsi".into(),
                value: COLLADA_XMLNS_XSI
            }
        };
        write_start_element(w, "COLLADA", &attributes)?;

        self.asset.write(w)?;

        match &self.geometries
        {
            Some(geometries) => {
                write_start_element(w, "library_geometries", &Vec::new())?;
                
                for it in geometries 
                { 
                    it.write(w)?;
                }

                write_end_element(w, "library_geometries")?;
            },
            None => {

            }
        }

        match &self.visual_scenes
        {
            Some(visual_scenes) => {
                write_start_element(w, "library_visual_scenes", &Vec::new())?;
                
                for visual_scene in visual_scenes
                {
                    visual_scene.write(w)?;
                }
                
                write_end_element(w, "library_visual_scenes")?;
            }, None => {

            }
        }

        match &self.scene
        {
            Some(scene) => {
                scene.write(w)?;
            }, None => {

            }
        }

        write_end_element(w, "COLLADA")?;

        Ok(())
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> Result<()>
    {
        let mut writer = EmitterConfig::new().perform_indent(true).create_writer(w);   
        
        self.write(&mut writer)?;

        return Ok(());
    }
}