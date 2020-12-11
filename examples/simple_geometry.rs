extern crate collada_io;

use std::fs::File;

fn main()
{
    println!("Simple Geometry Example!");

    let mut file = File::create("simple-geometry.dae").unwrap();
    let collada: collada_io::collada::Collada = collada_io::collada::Collada {
        scene: None,
        visual_scenes: None,
        asset: collada_io::meta::Asset::default(),
        geometries: Some(vec!{
            collada_io::geometry::Geometry {
                id: Some("Cube-mesh".to_string()),
                name: Some("Cube".to_string()),
                mesh: collada_io::geometry::Mesh {
                    triangles: collada_io::geometry::Triangles {
                        vertices: "#Cube-mesh-vertices".to_string(),
                        normals: Some("#Cube-mesh-normals".to_string()),
                        tex_vertices: Some("#Cube-mesh-map-0".to_string()),
                        primitive: Some(vec! {
                            4, 0, 0, 2, 0, 1, 0, 0, 2, 
                            2, 1, 3, 7, 1, 4, 3, 1, 5,
                            6, 2, 6, 5, 2, 7, 7, 2, 8, 
                            1, 3, 9, 7, 3, 10, 5, 3, 11, 
                            0, 4, 12, 3, 4, 13, 1, 4, 14, 
                            4, 5, 15, 1, 5, 16, 5, 5, 17, 
                            4, 0, 18, 6, 0, 19, 2, 0, 20, 
                            2, 1, 21, 6, 1, 22, 7, 1, 23, 
                            6, 2, 24, 4, 2, 25, 5, 2, 26, 
                            1, 3, 27, 3, 3, 28, 7, 3, 29, 
                            0, 4, 30, 2, 4, 31, 3, 4, 32, 
                            4, 5, 33, 0, 5, 34, 1, 5, 35
                        }),
                        material: None
                    },
                    vertices: collada_io::geometry::Vertices {
                        id: "Cube-mesh-vertices".to_string(),
                        name: None,
                        source: "#Cube-mesh-positions".to_string()
                    },
                    sources: vec! {
                        collada_io::geometry::Source {
                            id: "Cube-mesh-positions".to_string(),
                            float_array: collada_io::geometry::FloatArray {
                                id: "Cube-mesh-positions-array".to_string(),    
                                data: vec!{
                                    1.0, 1.0, 1.0, 
                                    1.0, 1.0, -1.0, 
                                    1.0, -1.0, 1.0, 
                                    1.0, -1.0, -1.0, 
                                    -1.0, 1.0, 1.0, 
                                    -1.0, 1.0, -1.0, 
                                    -1.0, -1.0, 1.0, 
                                    -1.0, -1.0, -1.0
                                }
                            },
                            accessor: collada_io::geometry::Accessor {
                                params: vec! { "X".to_string(), "Y".to_string(), "Z".to_string() }
                            }
                        },
                        collada_io::geometry::Source {
                            id: "Cube-mesh-normals".to_string(),
                            float_array: collada_io::geometry::FloatArray {
                                id: "Cube-mesh-normals-array".to_string(),    
                                data: vec!{
                                    0.0, 0.0, 1.0, 
                                    0.0, -1.0, 0.0,
                                    -1.0, 0.0, 0.0, 
                                    0.0, 0.0, -1.0, 
                                    1.0, 0.0, 0.0, 
                                    0.0, 1.0, 0.0,
                                }
                            },
                            accessor: collada_io::geometry::Accessor {
                                params: vec! { "X".to_string(), "Y".to_string(), "Z".to_string() }
                            }
                        },
                        collada_io::geometry::Source {
                            id: "Cube-mesh-map-0".into(),
                            float_array: collada_io::geometry::FloatArray {
                                id: "Cube-mesh-map-0-array".into(),
                                data: vec!{
                                    0.875, 0.5,
                                    0.625, 0.75, 
                                    0.625, 0.5,
                                    0.625, 0.75, 
                                    0.375, 1.0, 
                                    0.375, 0.75, 
                                    0.625, 0.0, 
                                    0.375, 0.25, 
                                    0.375, 0.0, 
                                    0.375, 0.5,
                                    0.125, 0.75, 
                                    0.125, 0.5, 
                                    0.625, 0.5, 
                                    0.375, 0.75, 
                                    0.375, 0.5, 
                                    0.625, 0.25, 
                                    0.375, 0.5, 
                                    0.375, 0.25, 
                                    0.875, 0.5, 
                                    0.875, 0.75, 
                                    0.625, 0.75, 
                                    0.625, 0.75, 
                                    0.625, 1.0, 
                                    0.375, 1.0, 
                                    0.625, 0.0, 
                                    0.625, 0.25, 
                                    0.375, 0.25, 
                                    0.375, 0.5, 
                                    0.375, 0.75, 
                                    0.125, 0.75,
                                    0.625, 0.5, 
                                    0.625, 0.75, 
                                    0.375, 0.75, 
                                    0.625, 0.25, 
                                    0.625, 0.5, 
                                    0.375, 0.5,
                                }
                            },
                            accessor: collada_io::geometry::Accessor {
                                params: vec! { "S".to_string(), "T".to_string()}
                            }
                        }
                    }
                }
            }
        })
    };
    collada.write_to(&mut file).unwrap();
}