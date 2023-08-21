use crate::input::SteelColumnDrawing;
use crate::output_util::*;
use dxf::{
    entities::{Circle, Entity, Line, Polyline},
    tables::Layer,
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<(), Box<dyn Error>> {
    let layer = Layer {
        name: input.layer_name.s_column.clone(),
        color: Color::from_index(4),
        ..Default::default()
    };

    drawing.add_layer(layer);

    let layer = Layer {
        name: input.layer_name.bolt.clone(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(layer);

    let layer = Layer {
        name: input.layer_name.plate.clone(),
        color: Color::from_index(1),
        ..Default::default()
    };

    drawing.add_layer(layer);

    Ok(())
}

fn write_column(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<(), Box<dyn Error>> {
    let h = input.h_section.h;
    let b = input.h_section.b;
    let tw = input.h_section.tw;
    let tf = input.h_section.tf;
    let r = input.h_section.r;

    let layer = &input.layer_name.s_column;

    write_line(drawing, -b / 2.0, h / 2.0, b / 2.0, h / 2.0, layer)?;
    write_line(drawing, -b / 2.0, -h / 2.0, b / 2.0, -h / 2.0, layer)?;

    write_line(drawing, -b / 2.0, h / 2.0 - tf, -b / 2.0, h / 2.0, layer)?;
    write_line(drawing, b / 2.0, h / 2.0 - tf, b / 2.0, h / 2.0, layer)?;
    write_line(drawing, -b / 2.0, -h / 2.0 + tf, -b / 2.0, -h / 2.0, layer)?;
    write_line(drawing, b / 2.0, -h / 2.0 + tf, b / 2.0, -h / 2.0, layer)?;

    write_line(
        drawing,
        -b / 2.0,
        h / 2.0 - tf,
        -tw / 2.0 - r,
        h / 2.0 - tf,
        layer,
    )?;
    write_line(
        drawing,
        b / 2.0,
        h / 2.0 - tf,
        tw / 2.0 + r,
        h / 2.0 - tf,
        layer,
    )?;
    write_line(
        drawing,
        -b / 2.0,
        -h / 2.0 + tf,
        -tw / 2.0 - r,
        -h / 2.0 + tf,
        layer,
    )?;
    write_line(
        drawing,
        b / 2.0,
        -h / 2.0 + tf,
        tw / 2.0 + r,
        -h / 2.0 + tf,
        layer,
    )?;

    write_line(
        drawing,
        -tw / 2.0,
        -h / 2.0 + tf + r,
        -tw / 2.0,
        h / 2.0 - tf - r,
        layer,
    )?;
    write_line(
        drawing,
        tw / 2.0,
        -h / 2.0 + tf + r,
        tw / 2.0,
        h / 2.0 - tf - r,
        layer,
    )?;

    if r > 0.0 {
        write_arc(
            drawing,
            -tw / 2.0 - r,
            h / 2.0 - tf - r,
            r,
            0.0,
            90.0,
            layer,
        )?;
        write_arc(
            drawing,
            tw / 2.0 + r,
            h / 2.0 - tf - r,
            r,
            90.0,
            180.0,
            layer,
        )?;
        write_arc(
            drawing,
            tw / 2.0 + r,
            -h / 2.0 + tf + r,
            r,
            180.0,
            270.0,
            layer,
        )?;
        write_arc(
            drawing,
            -tw / 2.0 - r,
            -h / 2.0 + tf + r,
            r,
            270.0,
            0.0,
            layer,
        )?;
    }

    Ok(())
}

fn write_base_plate(
    drawing: &mut Drawing,
    input: &SteelColumnDrawing,
) -> Result<(), Box<dyn Error>> {
    let lx = input.base_plate.lx;
    let ly = input.base_plate.ly;
    let layer = &input.layer_name.plate;

    write_line(drawing, -lx / 2.0, -ly / 2.0, lx / 2.0, -ly / 2.0, layer)?;
    write_line(drawing, -lx / 2.0, ly / 2.0, lx / 2.0, ly / 2.0, layer)?;
    write_line(drawing, -lx / 2.0, -ly / 2.0, -lx / 2.0, ly / 2.0, layer)?;
    write_line(drawing, lx / 2.0, -ly / 2.0, lx / 2.0, ly / 2.0, layer)?;

    Ok(())
}

fn get_bolt_coords(input: &SteelColumnDrawing) -> Vec<(f64, f64)> {
    let mut coords = Vec::new();

    let nx = input.anchor_bolt.nx;
    let ny = input.anchor_bolt.ny;

    let jx = input.anchor_bolt.jx;
    let jy = input.anchor_bolt.jy;

    let px = if nx == 1 { 0.0 } else { jy / (nx - 1) as f64 };
    let py = if ny == 1 { 0.0 } else { jx / (ny - 1) as f64 };

    let y_start = -jy / 2.0;

    for i in 0..nx {
        let y = y_start + i as f64 * px;

        let x = -jx / 2.0;
        coords.push((x, y));

        let x = jx / 2.0;
        coords.push((x, y));
    }

    let x_start = -jx / 2.0;

    for i in 0..ny {
        let x = x_start + i as f64 * py;

        let y = -jy / 2.0;
        coords.push((x, y));

        let y = jy / 2.0;
        coords.push((x, y));
    }

    coords
}

fn write_anchor_bolts(
    drawing: &mut Drawing,
    input: &SteelColumnDrawing,
) -> Result<(), Box<dyn Error>> {
    let d = input.anchor_bolt.d;
    let bolt_layer = &input.layer_name.bolt;
    let plate_layer = &input.layer_name.plate;

    for coord in get_bolt_coords(input) {
        let x = coord.0;
        let y = coord.1;
        let r = (d + 5.0) / 2.0;
        write_circle(drawing, x, y, r, plate_layer)?;
        write_cross(drawing, x, y, r + 1.0, bolt_layer)?;
    }

    Ok(())
}

pub fn write(input: SteelColumnDrawing, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_column(&mut drawing, &input)?;

    write_base_plate(&mut drawing, &input)?;

    write_anchor_bolts(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
