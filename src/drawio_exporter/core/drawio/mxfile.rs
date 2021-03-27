use anyhow::{Context, Result};
use flate2::read::DeflateDecoder;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct MxCell {
    pub value: Option<String>,
}

impl MxCell {
    pub fn value_as_link(&self) -> Option<(String, String)> {
        if let Some(value) = self.value.clone() {
            if value.contains("href=") {
                return MxCell::extract_link_data(value);
            }
        }
        None
    }

    fn extract_link_data(value: String) -> Option<(String, String)> {
        // TODO Clean <.*> text
        // TODO Clean &nbsp; or &amp;nbsp;
        if let Ok(re) = Regex::new(".*href=\"(.*)\".*>(.*)<.*") {
            if let Some(caps) = re.captures(value.as_str()) {
                let link_value = match caps.get(1) {
                    Some(link) => Some(link.as_str().to_string()),
                    None => None,
                };
                let link_text = match caps.get(2) {
                    Some(link) => Some(link.as_str().to_string()),
                    None => None,
                };

                match (link_value, link_text) {
                    (Some(link), Some(text)) => return Some((link, text)),
                    (_, _) => {}
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Element {
    MxCell(MxCell),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

fn deserialize_ignore_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(deserializer)?;
    Ok(())
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Root {
    #[serde(rename = "$value")]
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct MxGraphModel {
    #[serde(rename = "root", default)]
    pub root: Root,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct Diagram {
    pub id: String,
    pub name: String,
    #[serde(rename = "mxGraphModel", default)]
    pub mx_graph_model: MxGraphModel,
}

impl Diagram {
    pub fn get_links(&self) -> Vec<(String, String)> {
        self.mx_graph_model
            .root
            .elements
            .iter()
            .map(|element| match element {
                Element::MxCell(cell) => Some(cell),
                Element::Other => None,
            })
            .filter(|cell| cell.is_some())
            .map(|cell| cell.unwrap().value_as_link())
            .filter(|link| link.is_some())
            .map(|link| link.unwrap())
            .collect()
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Mxfile {
    #[serde(rename = "diagram", default)]
    pub diagrams: Vec<Diagram>,
}

#[derive(Debug, Deserialize, PartialEq, Default, Clone)]
pub struct CompressDiagram {
    pub id: String,
    pub name: String,
    #[serde(rename = "$value")]
    pub raw_diagram: String,
}

#[derive(Debug, Deserialize)]
pub struct MxfileWithCompressDiagrams {
    #[serde(rename = "diagram", default)]
    pub diagrams: Vec<CompressDiagram>,
}

pub fn read_file(path: &Path) -> Result<Mxfile> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("can read content of {}", path.display()))?;
    match content.is_empty() {
        true => Ok(Mxfile { diagrams: vec![] }),
        false => parse_compressed_content(path, content.clone())
            .or_else(|_| parse_uncompressed_content(path, content)),
    }
}

fn parse_compressed_content(path: &Path, content: String) -> Result<Mxfile> {
    let mxfile_with_compressed_diagrams: MxfileWithCompressDiagrams =
        serde_xml_rs::from_reader(content.as_bytes())
            .with_context(|| format!("can parse xml on {}", path.display()))?;
    let mxfile = decompress(mxfile_with_compressed_diagrams)
        .with_context(|| format!("can uncompress xml on {}", path.display()))?;
    Ok(mxfile)
}

fn decompress(mxfile_with_compressed_diagrams: MxfileWithCompressDiagrams) -> Result<Mxfile> {
    let mut diagrams: Vec<Diagram> = vec![];
    for compressed_diagram in mxfile_with_compressed_diagrams.diagrams {
        let base64_raw_diagram = base64::decode(compressed_diagram.raw_diagram)?;

        let mut raw_diagram_deflate_decoder = DeflateDecoder::new(&base64_raw_diagram[..]);
        let mut urlencoded_diagram = String::new();
        raw_diagram_deflate_decoder.read_to_string(&mut urlencoded_diagram)?;

        let xml_diagram = urlencoding::decode(urlencoded_diagram.as_str())?;

        let mx_graph_model: MxGraphModel = serde_xml_rs::from_reader(xml_diagram.as_bytes())?;

        diagrams.push(Diagram {
            id: compressed_diagram.id,
            name: compressed_diagram.name,
            mx_graph_model,
        })
    }

    Ok(Mxfile { diagrams })
}

fn parse_uncompressed_content(path: &Path, content: String) -> Result<Mxfile> {
    let mxfile: Mxfile = serde_xml_rs::from_reader(content.as_bytes())
        .with_context(|| format!("can parse xml on {}", path.display()))?;
    Ok(mxfile)
}
