use dxf::{
    entities::{Arc, Circle, DimensionBase, Entity, EntityType, Line, OrdinateDimension, Text},
    enums::{HorizontalTextJustification, VerticalTextJustification},
    tables::{DimStyle, Layer},
    Color, Drawing, Point,
};
use std::error::Error;

pub fn write_line(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let line = Line {
        p1: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        p2: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
        ..Default::default()
    };
    let mut line = Entity::new(dxf::entities::EntityType::Line(line));
    line.common.layer = layer.to_string();
    drawing.add_entity(line);
    Ok(())
}

pub fn write_cross(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    write_line(drawing, x - r, y, x + r, y, layer)?;
    write_line(drawing, x, y - r, x, y + r, layer)?;
    Ok(())
}

pub fn write_circle(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let circle = Circle {
        center: Point { x, y, z: 0.0 },
        radius: r,
        ..Default::default()
    };

    let mut circle = Entity::new(dxf::entities::EntityType::Circle(circle));

    circle.common.layer = layer.to_string();

    drawing.add_entity(circle);

    Ok(())
}

pub fn write_arc(
    drawing: &mut Drawing,
    x: f64,
    y: f64,
    r: f64,
    start_angle: f64,
    end_angle: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let arc = Arc {
        center: Point { x, y, z: 0.0 },
        radius: r,
        start_angle,
        end_angle,
        ..Default::default()
    };

    let mut arc = Entity::new(dxf::entities::EntityType::Arc(arc));

    arc.common.layer = layer.to_string();

    drawing.add_entity(arc);

    Ok(())
}

pub fn write_dimension(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    text_rotation_angle: f64,
    layer: String,
) -> Result<(), Box<dyn Error>> {
    let dim_style = DimStyle {
        name: "mydim".to_string(),
        dimensioning_text_height: 1000.0,
        dimensioning_arrow_size: 500.0,
        dimension_extension_line_offset: 2000.0,
        ..Default::default()
    };

    drawing.add_dim_style(dim_style);

    let gap = 5000.0;

    let theta = text_rotation_angle / 180.0 * std::f64::consts::PI;

    let dimension_base = DimensionBase {
        definition_point_1: Point {
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0 - gap * f64::cos(theta),
            z: 0.0,
        },
        text_mid_point: Point {
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0 - gap * f64::cos(theta),
            z: 0.0,
        },
        dimension_style_name: "mydim".to_string(),
        text_rotation_angle,
        ..Default::default()
    };

    let dimension = OrdinateDimension {
        dimension_base,
        definition_point_2: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        definition_point_3: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
    };

    let mut dimension = Entity::new(dxf::entities::EntityType::OrdinateDimension(dimension));

    dimension.common.layer = layer;

    drawing.add_entity(dimension);

    Ok(())
}
