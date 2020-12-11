use std::borrow::Cow;
use std::io::Write;
use xml::writer::{EventWriter, XmlEvent, Result};

use xml::attribute::Attribute;
use xml::namespace::Namespace;

pub fn write_text_element<W: Write>( w: &mut EventWriter<W>, name: &str, text: &str, attributes: &Vec<Attribute>) -> Result<()>
{
    write_start_element(w, name, attributes)?;

    w.write(XmlEvent::Characters {
        0: text
    })?;

    write_end_element(w, name)?;
    
    Ok(())
}

pub fn write_vec_element<W: Write, T: std::fmt::Display>( w: &mut EventWriter<W>, name: &str, vec: &Vec<T>, attributes: &Vec<Attribute>) -> Result<()>
{
    let text = vec.iter().map(|idx| idx.to_string()).collect::<Vec<_>>().join(" ");
    write_text_element(w, name, &text, attributes)?;

    Ok(())
}

pub fn write_start_element<W: Write>( w: &mut EventWriter<W>, name: &str, attributes: &Vec<Attribute>) -> Result<()>
{
    w.write(XmlEvent::StartElement {
        name: name.into(),
        attributes: Cow::Borrowed(attributes),
        namespace: Cow::Owned(Namespace::empty())
    })?;
    Ok(())
}

pub fn write_end_element<W: Write>( w: &mut EventWriter<W>, name: &str) -> Result<()>
{
    w.write(XmlEvent::EndElement {
        name: Some(name.into()),
    })?;
    Ok(())
}