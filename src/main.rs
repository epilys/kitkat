#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use std::f32;
use std::f32::consts::{FRAC_PI_2, PI};

macro_rules! tr {
    ($cond:expr ,? $then:expr ,: $else:expr) => {
        if $cond {
            $then
        } else {
            $else
        }
    };
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[derive(Clone, Copy)]
struct Bitmap<'bits> {
    bits: &'bits [u8],
    width: usize,
    height: usize,
    x_offset: usize,
    y_offset: usize,
}

impl<'bits> Bitmap<'bits> {
    fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>) {
        let row_width =
            self.width.wrapping_div(8) + if self.width.wrapping_rem(8) > 0 { 1 } else { 0 };
        //std::dbg!(row_width);
        debug_assert_eq!(row_width * self.height, self.bits.len());
        let mut bits = self.bits.iter();
        let mut i = self.y_offset * (CAT_WIDTH);
        for _ in 0..self.height {
            let mut c = 0;
            'byte_row: for byte in bits.by_ref().take(row_width) {
                for n in 0..8 {
                    if self.x_offset + c == self.width || i + self.x_offset + c >= buffer.len() {
                        break 'byte_row;
                    }
                    if byte.rotate_right(n) & 0x01 > 0 {
                        buffer[i + self.x_offset + c] = fg;
                    } else if let Some(bg) = bg {
                        buffer[i + self.x_offset + c] = bg;
                    };
                    c += 1;
                }
            }
            i += CAT_WIDTH;
        }
        //debug_assert_eq!(i, self.width * self.height);
    }
}

include!("catback.rs");
const CATBACK: Bitmap<'static> = Bitmap {
    bits: CAT_BITS,
    width: CAT_WIDTH,
    height: CAT_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("cattie.rs");
const CATTIE: Bitmap<'static> = Bitmap {
    bits: CATTIE_BITS,
    width: CATTIE_WIDTH,
    height: CATTIE_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("catwhite.rs");
const CATWHITE: Bitmap<'static> = Bitmap {
    bits: CATWHITE_BITS,
    width: CATWHITE_WIDTH,
    height: CATWHITE_HEIGHT,
    x_offset: 0,
    y_offset: 0,
};

include!("eyes.rs");
const EYES: Bitmap<'static> = Bitmap {
    bits: EYES_BITS,
    width: EYES_WIDTH,
    height: EYES_HEIGHT,
    x_offset: 15,
    y_offset: 0,
};

include!("tail.rs");
const TAIL: Bitmap<'static> = Bitmap {
    bits: TAIL_BITS,
    width: TAIL_WIDTH,
    height: TAIL_HEIGHT,
    x_offset: 0,
    y_offset: TAIL_OFFSET_Y,
};

const NUM_TAILS: usize = 16;

const N_TAIL_PTS: usize = 7;
const CENTER_TAIL: [(i32, i32); N_TAIL_PTS] = [
    /*  "Center" tail points definition */
    (0, 0),
    (0, 76),
    (3, 82),
    (10, 84),
    (18, 82),
    (21, 76),
    (21, 70),
];

fn plot(buffer: &mut Vec<u8>, x: i32, y: i32) {
    if x < 0 || y < 0 {
        eprintln!("invalid plot() coors: ({}, {})", x, y);
        return;
    }
    //eprintln!("plot() coors: ({}, {})", x, y);
    let (x, y) = (x as usize, y as usize);
    let row_width = CAT_WIDTH.wrapping_div(8) + if CAT_WIDTH.wrapping_rem(8) > 0 { 1 } else { 0 };
    //std::dbg!(x.wrapping_div(8));
    //std::dbg!(x.wrapping_rem(8));
    let byte_column = x.wrapping_div(8) + 1;
    let bit_column = x.wrapping_rem(8) as u32;
    //std::dbg!(byte_column);
    //std::dbg!(bit_column);
    if let Some(cell) = buffer.get_mut(y * row_width + byte_column) {
        *cell |= 0x01_u8 << bit_column;
    }
}

fn plot_line_width(
    buffer: &mut Vec<u8>,
    (mut x0, mut y0): (i32, i32),
    (x1, y1): (i32, i32),
    wd: f32,
) {
    eprintln!(
        "plot_line_width: ({}, {}), ({}, {}) width = {}",
        x0, y0, x1, y1, wd
    );
    /* Bresenham's line algorithm */
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    /* error value e_xy */
    let mut e2: i32;
    let mut x2: i32;
    let mut y2: i32;
    let ed: f32 = if (dx + dy) == 0 {
        1.0
    } else {
        f32::sqrt((dx * dx) as f32 + (dy * dy) as f32)
    };
    let mut loop_ctr = 0;
    let mut points = vec![];
    let wd = (wd + 1.0) / 2.0;
    loop {
        points.push((x0, y0));
        plot(buffer, x0, y0);
        e2 = err;
        x2 = x0;
        if 2 * e2 >= -dx {
            /* x step */
            eprintln!(" x step ");
            e2 += dy;
            y2 = y0;
            while e2 < ((ed as f32 * wd) as i32) && (y1 <= y2 || dx > dy) {
                y2 += sy;
                plot(buffer, x0, y2);
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
            eprintln!(" y step ");
            e2 = dx - e2;
            while e2 < ((ed as f32 * wd) as i32) && (x1 != x2 || dx < dy) {
                x2 += sx;
                plot(buffer, x2, y0);
                points.push((x2, y0));
                e2 += dy;
            }
            if y0 == y1 {
                break;
            };
            err += dx;
            y0 += sy;
        }
        loop_ctr += 1;
    }
}

fn create_tail_pixmap(t: f32) -> Vec<u8> {
    /*  Pendulum parameters */
    let mut sin_theta: f32;
    let mut cos_theta: f32;
    const A: f32 = 0.4;
    let omega: f32 = 1.0;
    let phi: f32 = 3.0 * FRAC_PI_2;
    let mut angle: f32;

    //    static XPoint tailOffset = { 74, -15 };
    const TAIL_OFFSET: (i32, i32) = (74, -15);

    let mut off_center_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /* off center tail    */
    let mut new_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /*  Tail at time "t"  */

    {
        /*
         *  Create an "off-center" tail to deal with the fact that
         *  the tail has a hook to it.  A real pendulum so shaped would
         *  hang a bit to the left (as you look at the cat).
         */
        angle = -0.08;
        sin_theta = f32::sin(angle);
        cos_theta = f32::cos(angle);

        for i in 0..N_TAIL_PTS {
            off_center_tail[i].0 = ((CENTER_TAIL[i].0 as f32) * cos_theta
                + ((CENTER_TAIL[i].1 as f32) * sin_theta))
                as i32;
            off_center_tail[i].1 = ((-1.0 * (CENTER_TAIL[i].0 as f32)) * sin_theta
                + ((CENTER_TAIL[i].1 as f32) * cos_theta))
                as i32;
        }
    }

    /*
     *  Compute pendulum function.
     */
    angle = A * f32::sin(omega * t + phi);
    sin_theta = f32::sin(angle);
    cos_theta = f32::cos(angle);

    let mut ret = TAIL.bits.to_vec();
    /*
     *  Rotate the center tail about its origin by "angle" degrees.
     */
    for i in 0..N_TAIL_PTS {
        new_tail[i].0 = ((off_center_tail[i].0 as f32) * cos_theta
            + ((off_center_tail[i].1 as f32) * sin_theta)) as i32;
        new_tail[i].1 = ((off_center_tail[i].0 as f32 * -1.0) * sin_theta
            + ((off_center_tail[i].1 as f32) * cos_theta)) as i32;

        new_tail[i].0 += TAIL_OFFSET.0;
        new_tail[i].1 += TAIL_OFFSET.1;
    }

    for window in new_tail.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        plot_line_width(&mut ret, point_a, point_b, 25.0);
    }

    ret
}

fn right_turn(points: &[(i32, i32)]) -> bool {
    //Again, determining whether three points constitute a "left turn" or a "right turn" does not
    //require computing the actual angle between the two line segments, and can actually be
    //achieved with simple arithmetic only.
    //
    //For three points P 1 = ( x 1 , y 1 ) {\displaystyle P_{1}=(x_{1},y_{1})} P_{1}=(x_{1},y_{1}),
    //P 2 = ( x 2 , y 2 ) {\displaystyle P_{2}=(x_{2},y_{2})} P_{2}=(x_{2},y_{2}) and P 3 = ( x 3 ,
    //y 3 ) {\displaystyle P_{3}=(x_{3},y_{3})} P_{3}=(x_{3},y_{3}), compute the z-coordinate of
    //the cross product of the two vectors P 1 P 2 → {\displaystyle {\overrightarrow {P_{1}P_{2}}}}
    //\overrightarrow {P_{1}P_{2}} and P 1 P 3 → {\displaystyle {\overrightarrow {P_{1}P_{3}}}}
    //\overrightarrow {P_{1}P_{3}}, which is given by the expression ( x 2 − x 1 ) ( y 3 − y 1 ) −
    //( y 2 − y 1 ) ( x 3 − x 1 ) {\displaystyle
    //(x_{2}-x_{1})(y_{3}-y_{1})-(y_{2}-y_{1})(x_{3}-x_{1})}
    //(x_{2}-x_{1})(y_{3}-y_{1})-(y_{2}-y_{1})(x_{3}-x_{1}).
    //
    //If the result is 0, the points are collinear; if it is positive, the three points constitute
    //a "left turn" or counter-clockwise orientation, otherwise a "right turn" or clockwise
    //orientation (for counter-clockwise numbered points).
    let p_1 = &points[points.len() - 1];
    let p_2 = &points[points.len() - 2];
    let p_3 = &points[points.len() - 3];
    let z_coordinate = (p_2.0 - p_1.0) * (p_3.1 - p_1.1) - (p_2.1 - p_1.1) * (p_3.0 - p_1.0);
    z_coordinate < 0
}

fn convex_hull(points: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut pts = points.to_vec();
    pts.sort();

    let mut _L_ano = vec![pts[0], pts[1]];
    for i in 3..pts.len() {
        _L_ano.push(pts[i]);
        while _L_ano.len() > 2 && !right_turn(&_L_ano) {
            let middle_of_last_three = _L_ano.len() - 2;
            _L_ano.remove(middle_of_last_three);
        }
    }
    let mut _L_kato = vec![pts[pts.len() - 1], pts[pts.len() - 2]];
    for i in (pts.len() - 2)..1 {
        _L_kato.push(pts[i]);
        while _L_kato.len() > 2 && !right_turn(&_L_kato) {
            let middle_of_last_three = _L_kato.len() - 2;
            _L_kato.remove(middle_of_last_three);
        }
    }
    // Remove first and last vertex of _L_kato to avoid duplication of common vertices
    _L_kato.remove(0);
    _L_kato.pop();
    _L_ano.extend(_L_kato.into_iter());
    _L_ano
}
type Point = (i32, i32);

fn fill_polygon(buffer: &mut Vec<u8>, points: &[(i32, i32)]) {
    let mut y_min = points[0].1;
    let mut y_max = y_min;
    for (_, y) in points {
        if *y < y_min {
            y_min = *y;
        }
        if *y > y_max {
            y_max = *y;
        }
    }

    let mut pts = convex_hull(points);
    struct EdgeInfo {
        ab: (Point, Point),
        y_max: i32,
        x_max: i32,
        y_min: i32,
        x_min: i32,
        slope: f32,
    }
    let mut edge_info = vec![];
    for window in pts.as_slice().windows(2) {
        let ((x_a, y_a), (x_b, y_b)) = (window[0], window[1]);
        let slope = (y_a - y_b) as f32 / ((x_a - x_b) as f32);
        edge_info.push(EdgeInfo {
            ab: ((x_a, y_a), (x_b, y_b)),
            y_max: std::cmp::max(y_a, y_b),
            x_max: std::cmp::max(x_a, x_b),
            y_min: std::cmp::min(y_a, y_b),
            x_min: std::cmp::min(x_a, x_b),
            slope,
        });
    }

    for y in y_min..=y_max {
        for point in points {}
        //plot(buffer, x0, y0);
    }
}

fn create_eye_pixmap(t: f32) -> Vec<u8> {
    const A: f32 = 0.7;
    let omega: f32 = 1.0;
    let phi: f32 = 3.0 * FRAC_PI_2;
    let mut u: f32;
    let mut w: f32 = FRAC_PI_2;
    /*  Sphere parameters    */
    /*  Radius               */
    let mut r: f32 = 1.0;
    /*  Center of sphere     */
    let mut x0: f32 = 0.0;
    let mut y0: f32 = 0.0;
    let mut z0: f32 = 2.0;

    let mut angle: f32 = A * f32::sin(omega * t + phi) + w;
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(100);

    let mut i = 0;
    u = -1.0 * FRAC_PI_2;
    while u < FRAC_PI_2 {
        let x = x0 + r * f32::cos(u) * f32::cos(angle + PI / 7.0);
        let z = z0 + r * f32::cos(u) * f32::sin(angle + PI / 7.0);
        let y = y0 + r * f32::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u += 0.25;
        i += 1;
    }

    u = FRAC_PI_2;
    while u > -1.0 * FRAC_PI_2 {
        let x = x0 + r * f32::cos(u) * f32::cos(angle - PI / 7.0);
        let z = z0 + r * f32::cos(u) * f32::sin(angle - PI / 7.0);
        let y = y0 + r * f32::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u -= 0.25;
        i += 1;
    }
    /*
        /*
         *  Create pixmap for drawing eye (and stippling on update)
         */
        XFillPolygon(dpy, eyeBitmap, bitmapGC, pts, i, Nonconvex, CoordModeOrigin);

        for (j = 0; j < i; j++) {
            pts[j].x += 31;
        }
        XFillPolygon(dpy, eyeBitmap, bitmapGC, pts, i, Nonconvex, CoordModeOrigin);

        XFreeGC(dpy, bitmapGC);

        return (eyeBitmap);
    }
    */
    let mut ret = EYES.bits.to_vec();
    ret
}

fn main() {
    let azure_blue = from_u8_rgb(0, 127, 255);
    let white = from_u8_rgb(255, 255, 255);
    let black = 0;

    let mut buffer: Vec<u32> = vec![white; CAT_WIDTH * CAT_HEIGHT];
    let mut tails: Vec<Vec<u8>> = Vec::with_capacity(NUM_TAILS);
    let mut eyes: Vec<Vec<u8>> = Vec::with_capacity(NUM_TAILS);

    for i in 0..NUM_TAILS {
        tails.push(create_tail_pixmap(i as f32 * PI / (NUM_TAILS as f32)));
        eyes.push(create_eye_pixmap(i as f32 * PI / (NUM_TAILS as f32)));
    }

    let mut window = Window::new(
        "Test - ESC to exit",
        CAT_WIDTH,
        CAT_HEIGHT,
        WindowOptions {
            title: true,
            //borderless: true,
            //resize: false,
            //transparency: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    CATWHITE.draw(&mut buffer, white, Some(white));
    CATBACK.draw(&mut buffer, black, None);
    CATTIE.draw(&mut buffer, azure_blue, None);
    TAIL.draw(&mut buffer, black, None);
    /*
    let mut bits = CATBACK.bits.to_vec();
    for i in 0..bits.len() {
        bits[i] = 0;
    }
    let test: Bitmap<'_> = Bitmap {
        bits: &bits,
        width: CAT_WIDTH,
        height: CAT_HEIGHT,
        x_offset: 0,
        y_offset: 0,
    };
    test.draw(&mut buffer, black, Some(white));
    //plot_line_width(&mut bits, (25, 0), (25, 40), 5.0);
    for i in 0..4 {
        plot_line_width(
            &mut bits,
            (30, 150 + i * 10),
            (55, 150 + i * 10),
            1.0 + i as f32 * 1.0,
        );
    }
    //plot_line_width(&mut bits, (30, 150), (35, 150), 1.0);
    //plot_line_width(&mut bits, (35, 160), (30, 160), 1.0);
    let test: Bitmap<'_> = Bitmap {
        bits: &bits,
        width: CAT_WIDTH,
        height: CAT_HEIGHT,
        x_offset: 0,
        y_offset: 0,
    };
    test.draw(&mut buffer, black, Some(white));
    */

    let mut i: usize = 0;
    let mut up = true;
    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        let tail: Bitmap<'_> = Bitmap {
            bits: &tails[i],
            width: TAIL_WIDTH,
            height: TAIL_HEIGHT,
            x_offset: 0,
            y_offset: TAIL_OFFSET_Y,
        };
        TAIL.draw(&mut buffer, black, Some(white));
        tail.draw(&mut buffer, black, None);
        if up {
            if i + 1 == tails.len() {
                up = false;
            } else {
                i = (i + 1).wrapping_rem(tails.len());
            }
        } else {
            if i == 0 {
                up = true;
            } else {
                i -= 1;
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, CAT_WIDTH, CAT_HEIGHT)
            .unwrap();

        let millis = std::time::Duration::from_millis(100);

        std::thread::sleep(millis);
    }
}
