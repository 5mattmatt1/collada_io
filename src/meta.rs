use std::io::Write;
use xml::writer::{EventWriter, Result};
use xml::attribute::Attribute;
use crate::util::*;

pub struct Contributor
{
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub author_website: Option<String>,   
    pub authoring_tool: Option<String>,
    pub comments: Option<String>,
    pub copyright: Option<String>,
    pub source_data: Option<String>
}

pub struct Unit
{
    pub name: String,
    pub meter: f64
}

#[derive(Copy, Clone)]
pub enum UpAxis
{
    XUp,
    YUp,
    ZUp
}

pub struct Asset
{
    pub contributors: Vec<Contributor>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub keywords: Option<String>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub revision: Option<String>,
    pub subject: Option<String>,
    pub title: Option<String>,
    pub unit: Unit,
    pub up_axis: Option<UpAxis>,
}

impl Default for Unit
{
    fn default() -> Unit
    {
        Unit {
            name: "meter".to_string(),
            meter: 1.0
        }
    }
}

impl Default for UpAxis
{
    fn default() -> UpAxis
    {
        UpAxis::YUp
    }
}

impl<'a> Into<&'a str> for UpAxis
{
    fn into(self) -> &'a str
    {
        match self
        {
            Self::XUp => "X_UP",
            Self::YUp => "Y_UP",
            Self::ZUp => "Z_UP"
        }
    }
}

impl UpAxis
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        let up_axis_str: &str = (*self).into();
        write_text_element(w, "up_axis", up_axis_str, &Vec::new())?;

        Ok(())
    }   
}


impl Contributor
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        write_start_element(w, "contributor", &Vec::new())?;

        match &self.author
        {
            Some(author) => {
                write_text_element(w, "author", &author, &Vec::new())?;
            }, None => {}
        }

        match &self.authoring_tool
        {
            Some(authoring_tool) => {
                write_text_element(w, "authoring_tool", &authoring_tool, &Vec::new())?;
            }, None => {}
        }

        write_end_element(w, "contributor")?;
        Ok(())
    }
}

impl Unit
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        let meter = self.meter.to_string();
        let attributes = vec!{
            Attribute {
                name: "unit".into(),
                value: &self.name
            },
            Attribute {
                name: "meter".into(),
                value: &meter
            }
        };
        write_start_element(w, "unit", &attributes)?;
        write_end_element(w, "unit")?;
        Ok(())
    }   
}

impl Asset
{
    pub fn write<W: Write>(&self, w: &mut EventWriter<W>) -> Result<()>
    {
        write_start_element(w, "asset", &Vec::new())?;

        for contributor in &self.contributors
        {
            contributor.write(w)?;
        }

        write_text_element(w, "created", &self.created.to_rfc2822(), &Vec::new())?;
        write_text_element(w, "modified", &self.modified.to_rfc2822(), &Vec::new())?;

        self.unit.write(w)?;
        match &self.up_axis
        {
            Some(up_axis) => {
                up_axis.write(w)?;
            },
            None => {

            }
        }

        write_end_element(w, "asset")?;

        Ok(())
    }
}