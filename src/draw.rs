/*
 * kitkat
 *
 * Copyright 2021 - Manos Pitsidianakis
 *
 * This file is part of kitkat.
 *
 * kitkat is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * kitkat is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with kitkat. If not, see <http://www.gnu.org/licenses/>.
 */

pub struct Buffer<'v> {
    pub vec: &'v mut Vec<u8>,
    pub row_width: usize,
    pub height: usize,
}

pub fn plot(buffer: &'_ mut Buffer<'_>, (x, y): (i64, i64)) {
    if x < 0 || y < 0 {
        eprintln!("invalid plot() coors: ({}, {})", x, y);
        return;
    }
    //eprintln!("plot() coors: ({}, {})", x, y);
    let (x, y) = (x as usize, y as usize);
    let row_width = buffer.row_width.wrapping_div(8)
        + if buffer.row_width.wrapping_rem(8) > 0 {
            1
        } else {
            0
        };
    //std::dbg!(x.wrapping_div(8));
    //std::dbg!(x.wrapping_rem(8));
    let byte_column = x.wrapping_div(8) + 1;
    let bit_column = x.wrapping_rem(8) as u32;
    //std::dbg!(byte_column);
    //std::dbg!(bit_column);
    if let Some(cell) = buffer.vec.get_mut(y * row_width + byte_column) {
        *cell |= 0x01_u8 << bit_column;
    }
}

pub fn plot_line_with_width(
    buffer: &'_ mut Buffer<'_>,
    point_a: (i64, i64),
    point_b: (i64, i64),
    wd: f64,
) {
    let width2 = wd / 2.0;
    plot_line_width(buffer, point_a, point_b, 1.0);
    for w in ((-1.0 * width2) as i64)..(width2 as i64) {
        plot_line_width(
            buffer,
            (point_a.0 + w, point_a.1),
            (point_b.0 + w, point_b.1),
            1.0,
        );
    }
}

pub fn plot_line_width(
    buffer: &'_ mut Buffer<'_>,
    (mut x0, mut y0): (i64, i64),
    (x1, y1): (i64, i64),
    wd: f64,
) {
    //eprintln!(
    //    "plot_line_width: ({}, {}), ({}, {}) width = {}",
    //    x0, y0, x1, y1, wd
    //);
    /* Bresenham's line algorithm */
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    /* error value e_xy */
    let mut e2: i64;
    let mut x2: i64;
    let mut y2: i64;
    let ed: f64 = if (dx + dy) == 0 {
        1.0
    } else {
        f64::sqrt((dx * dx) as f64 + (dy * dy) as f64)
    };
    let mut points = vec![];
    let wd = (wd + 1.0) / 2.0;
    //eprintln!("wd = {}, ed = {}", wd, ed);
    loop {
        points.push((x0, y0));
        plot(buffer, (x0, y0));
        e2 = err;
        x2 = x0;
        if 2 * e2 >= -dx {
            /* x step */
            //eprintln!(" x step ");
            e2 += dy;
            y2 = y0;
            while e2 < ((ed as f64 * wd) as i64) && (y1 != y2 || dx > dy) {
                y2 += sy;
                plot(buffer, (x0, y2));
                points.push((x0, y2));
                e2 += dx;
            }
            if x0 == x1 {
                break;
            };
            e2 = err;
            err -= dy;
            x0 += sx;
        }
        if 2 * e2 <= dy {
            /* y step */
            //eprintln!(" y step ");
            e2 = dx - e2;
            while e2 < ((ed as f64 * wd) as i64) && (x1 != x2 || dx < dy) {
                x2 += sx;
                plot(buffer, (x2, y0));
                points.push((x2, y0));
                e2 += dy;
            }
            if y0 == y1 {
                break;
            };
            err += dx;
            y0 += sy;
        }
    }
}

pub fn plot_ellipse(
    buffer: &'_ mut Buffer<'_>,
    (xm, ym): (i64, i64),
    (a, b): (i64, i64),
    quadrants: [bool; 4],
    _wd: f64,
) {
    let mut x = -a;
    let mut y = 0;
    let mut e2 = b;
    let mut dx = (1 + 2 * x) * e2 * e2;
    let mut dy = x * x;
    let mut err = dx + dy;
    loop {
        if quadrants[0] {
            plot(buffer, (xm - x, ym + y)); /*   I. Quadrant */
        }
        if quadrants[1] {
            plot(buffer, (xm + x, ym + y)); /*  II. Quadrant */
        }
        if quadrants[2] {
            plot(buffer, (xm + x, ym - y)); /* III. Quadrant */
        }
        if quadrants[3] {
            plot(buffer, (xm - x, ym - y)); /*  IV. Quadrant */
        }
        e2 = 2 * err;
        if e2 >= dx {
            x += 1;
            dx += 2 * b * b;
            err += dx;
            //err += dx += 2*(long)b*b; }    /* x step */
        }
        if e2 <= dy {
            y += 1;
            dy += 2 * a * a;
            err += dy;
            //err += dy += 2*(long)a*a; }    /* y step */
        }
        if x > 0 {
            break;
        }
    }
    while y < b {
        /* to early stop for flat ellipses with a=1, */
        y += 1;
        plot(buffer, (xm, ym + y)); /* -> finish tip of ellipse */
        plot(buffer, (xm, ym - y));
    }
}
