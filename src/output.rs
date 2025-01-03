use crate::input::SteelColumnDrawing;
use crate::output_util::*;
use anyhow::Result;
use dxf::{tables::Layer, Color, Drawing};

fn set_layer(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<()> {
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

    let layer = Layer {
        name: input.layer_name.text.clone(),
        color: Color::from_index(3),
        ..Default::default()
    };

    drawing.add_layer(layer);

    Ok(())
}

fn write_column(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<()> {
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

fn write_base_plate(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<()> {
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

fn write_anchor_bolts(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<()> {
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

fn write_texts(drawing: &mut Drawing, input: &SteelColumnDrawing) -> Result<()> {
    let text_height = input.layout.text_height;
    let x = 0.0;
    let mut y = -1000.0;
    let dy = 2.0 * text_height;

    let values = [
        input.column_name.clone(),
        format!(
            "H-{}x{}x{}x{}({})",
            input.h_section.h,
            input.h_section.b,
            input.h_section.tw,
            input.h_section.tf,
            input.h_section.material,
        ),
        format!(
            "BPL-{}x{}x{}({})",
            input.base_plate.t, input.base_plate.lx, input.base_plate.ly, input.base_plate.material,
        ),
        format!(
            "{}-M{}(L={},{})",
            2 * (input.anchor_bolt.nx - 1) + 2 * (input.anchor_bolt.ny - 1),
            input.anchor_bolt.d,
            input.anchor_bolt.l,
            input.anchor_bolt.material,
        ),
        format!("({})", input.anchor_bolt.note),
        format!(
            "PL-{}x{}x{}({})",
            input.anchor_plate.t,
            input.anchor_plate.d,
            input.anchor_plate.d,
            input.anchor_plate.material,
        ),
    ];

    for value in values {
        write_text(drawing, x, y, text_height, &value, &input.layer_name.text)?;
        y -= dy;
    }

    Ok(())
}

pub fn write(input: SteelColumnDrawing, output_file: &str) -> Result<()> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_column(&mut drawing, &input)?;

    write_base_plate(&mut drawing, &input)?;

    write_anchor_bolts(&mut drawing, &input)?;

    write_texts(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
