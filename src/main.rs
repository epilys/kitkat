#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use std::f64;
use std::f64::consts::{FRAC_PI_2, PI};

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
        let mut i = self.y_offset * (self.width);
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
    x_offset: 5,
    y_offset: 56,
};

include!("tail.rs");
const TAIL: Bitmap<'static> = Bitmap {
    bits: TAIL_BITS,
    width: TAIL_WIDTH,
    height: TAIL_HEIGHT,
    x_offset: 0,
    y_offset: TAIL_OFFSET_Y,
};

const NUM_TAILS: usize = 10;

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

struct Buffer {
    vec: Vec<u8>,
    row_width: usize,
    height: usize,
}

fn plot(buffer: &mut Buffer, (x, y): (i32, i32)) {
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

fn plot_line_with_width(buffer: &mut Buffer, point_a: (i32, i32), point_b: (i32, i32), wd: f64) {
    let width2 = wd / 2.0;
    plot_line_width(buffer, point_a, point_b, 1.0);
    for w in ((-1.0 * width2) as i32)..(width2 as i32) {
        plot_line_width(
            buffer,
            (point_a.0 + w, point_a.1),
            (point_b.0 + w, point_b.1),
            1.0,
        );
    }
}

fn plot_line_width(
    buffer: &mut Buffer,
    (mut x0, mut y0): (i32, i32),
    (x1, y1): (i32, i32),
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
    let mut e2: i32;
    let mut x2: i32;
    let mut y2: i32;
    let ed: f64 = if (dx + dy) == 0 {
        1.0
    } else {
        f64::sqrt((dx * dx) as f64 + (dy * dy) as f64)
    };
    let mut loop_ctr = 0;
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
            while e2 < ((ed as f64 * wd) as i32) && (y1 != y2 || dx > dy) {
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
            while e2 < ((ed as f64 * wd) as i32) && (x1 != x2 || dx < dy) {
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
        loop_ctr += 1;
    }
}

fn plot_ellipse(
    buffer: &mut Buffer,
    (xm, ym): (i32, i32),
    (a, b): (i32, i32),
    quadrants: [bool; 4],
    wd: f64,
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

fn create_eye_pixmap2(t: f64) -> Vec<u8> {
    let mut ret = Buffer {
        vec: EYES.bits.to_vec(),
        row_width: EYES.width,
        height: EYES.height,
    };
    let mut points: Vec<Vec<bool>> = vec![vec![false; EYES.width + 1]; EYES.height + 1];
    let top_point: (i32, i32) = ((EYES.width / 2) as i32, 0);
    let bottom_point: (i32, i32) = ((EYES.width / 2) as i32, EYES.height as i32);

    let center_point = (((3 * EYES.width) / 2) as i32, (EYES.height / 2) as i32);
    //println!("center_point: {:?}", center_point);

    let mut plot_point = move |points: &mut Vec<Vec<bool>>, point| {
        //eprintln!("plot_point: {:?}", point);
        let (x, y) = point;
        if y as usize >= points.len() || x as usize >= points[y as usize].len() {
            return;
        }
        points[y as usize][x as usize] = true;
    };

    //plot(&mut ret, top_point);
    plot_point(&mut points, top_point);
    plot_point(&mut points, bottom_point);

    let W64 = EYES.width as f64;
    let H64 = EYES.height as f64;
    let H64_2 = H64 / 2.0;

    let mut cos_theta_i; //= f64::cos(FRAC_PI_2 + std::dbg!(f64::acos((W64/r))));
    let mut sin_theta_i; //= f64::sin(FRAC_PI_2 + f64::acos((W64/r)));

    let mut r: f64 = f64::sqrt(W64.powi(2) + (H64_2).powi(2));
    //std::dbg!(H64_2);
    //std::dbg!(r);
    //std::dbg!(H64_2 / r);
    //let mut theta_i = -1.*(FRAC_PI_2 + f64::acos(H64_2/r));
    //   println!("theta_i: {:?}", theta_i);

    for y_k in top_point.1..=bottom_point.1 {
        sin_theta_i = ((y_k - center_point.1) as f64 / r);
        cos_theta_i = f64::sqrt(1. - sin_theta_i.powi(2));

        let x_k = (center_point.0 + (r * cos_theta_i) as i32) - center_point.0 - EYES.width as i32;
        plot_point(&mut points, (x_k, y_k));
        plot(&mut ret, (x_k, y_k));
    }
    //eprintln!("got points:");
    //for row in points {
    //    for p in row {
    //        print!("{}", if p { "❖" } else { "." });
    //    }
    //    println!("");
    //}
    //for b in ret.iter_mut() {
    //    *b = 0b10101010;
    //}

    ret.vec
}

fn create_tail_pixmap(t: f64) -> Vec<u8> {
    /*  Pendulum parameters */
    let mut sin_theta: f64;
    let mut cos_theta: f64;
    const A: f64 = 0.4;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut angle: f64;

    //    static XPoint tailOffset = { 74, -15 };
    const TAIL_OFFSET: (i32, i32) = (72, 0);

    let mut off_center_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /* off center tail    */
    let mut new_tail: Vec<(i32, i32)> = vec![(0, 0); N_TAIL_PTS]; /*  Tail at time "t"  */

    {
        /*
         *  Create an "off-center" tail to deal with the fact that
         *  the tail has a hook to it.  A real pendulum so shaped would
         *  hang a bit to the left (as you look at the cat).
         */
        angle = -0.08;
        sin_theta = f64::sin(angle);
        cos_theta = f64::cos(angle);

        for i in 0..N_TAIL_PTS {
            off_center_tail[i].0 = ((CENTER_TAIL[i].0 as f64) * cos_theta
                + ((CENTER_TAIL[i].1 as f64) * sin_theta))
                as i32;
            off_center_tail[i].1 = ((-1.0 * (CENTER_TAIL[i].0 as f64)) * sin_theta
                + ((CENTER_TAIL[i].1 as f64) * cos_theta))
                as i32;
        }
    }

    /*
     *  Compute pendulum function.
     */
    angle = A * f64::sin(omega * t + phi);
    sin_theta = f64::sin(angle);
    cos_theta = f64::cos(angle);

    let mut ret = Buffer {
        vec: TAIL.bits.to_vec(),
        row_width: TAIL.width,
        height: TAIL.height,
    };
    /*
     *  Rotate the center tail about its origin by "angle" degrees.
     */
    for i in 0..N_TAIL_PTS {
        new_tail[i].0 = ((off_center_tail[i].0 as f64) * cos_theta
            + ((off_center_tail[i].1 as f64) * sin_theta)) as i32;
        new_tail[i].1 = ((off_center_tail[i].0 as f64 * -1.0) * sin_theta
            + ((off_center_tail[i].1 as f64) * cos_theta)) as i32;

        new_tail[i].0 += TAIL_OFFSET.0;
        new_tail[i].1 += TAIL_OFFSET.1;
    }

    const WIDTH: f64 = 15.0;
    const WIDTH2: f64 = WIDTH / 2.0;
    for window in new_tail.as_slice().windows(2) {
        let mut point_a = window[0];
        let mut point_b = window[1];
        plot_line_with_width(&mut ret, point_a, point_b, WIDTH as _);
    }

    let mut last_point = *new_tail.last().unwrap();
    last_point.1 += 1;
    for b in 0..=((0.8 * WIDTH2) as i32) {
        plot_ellipse(
            &mut ret,
            last_point,
            (WIDTH2 as i32, b),
            [false, false, true, true],
            1.0,
        );
    }

    ret.vec
}

fn create_eye_pixmap(t: f64) -> Vec<u8> {
    /*
    const A: f64 = 0.7;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut u: f64;
    let mut w: f64 = FRAC_PI_2;
    /*  Sphere parameters    */
    /*  Radius               */
    let mut r: f64 = 1.0;
    /*  Center of sphere     */
    let mut x0: f64 = 0.0;
    let mut y0: f64 = 0.0;
    let mut z0: f64 = 2.0;

    let mut angle: f64 = A * f64::sin(omega * t + phi) + w;
    let mut points: Vec<(i32, i32)> = Vec::with_capacity(100);

    let mut i = 0;
    u = -1.0 * FRAC_PI_2;
    while u < FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle + PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle + PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u += 0.25;
        i += 1;
    }

    u = FRAC_PI_2;
    while u > -1.0 * FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle - PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle - PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i32;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i32;
        points.push((a, b));
        u -= 0.25;
        i += 1;
    }
    */
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
    let red = from_u8_rgb(157, 37, 10);
    let white = from_u8_rgb(255, 255, 255);
    let black = 0;

    let mut buffer: Vec<u32> = vec![white; CAT_WIDTH * CAT_HEIGHT];
    let mut tails: Vec<Vec<u8>> = Vec::with_capacity(NUM_TAILS);
    let mut eyes: Vec<Vec<u8>> = Vec::with_capacity(NUM_TAILS);

    for i in 0..NUM_TAILS {
        tails.push(create_tail_pixmap(i as f64 * PI / (NUM_TAILS as f64)));
        eyes.push(create_eye_pixmap2(i as f64 * PI / (NUM_TAILS as f64)));
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
    //EYES.draw(&mut buffer, black, None);
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
            1.0 + i as f64 * 1.0,
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
            ..TAIL
        };
        TAIL.draw(&mut buffer, black, Some(white));
        tail.draw(&mut buffer, black, None);
        let eye: Bitmap<'_> = Bitmap {
            bits: &eyes[i],
            ..EYES
        };
        EYES.draw(&mut buffer, black, Some(white));
        eye.draw(&mut buffer, red, None);
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
