extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use std::f32;
use std::f32::consts::{FRAC_PI_2, PI};

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
        debug_assert_eq!(row_width * self.height, self.bits.len());
        let mut bits = self.bits.iter();
        let mut i = self.y_offset * (row_width * 8);
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

/*
include!("eyes.rs");
const EYES: Bitmap<'static> = Bitmap {
    bits: EYES_BITS,
    width: EYES_WIDTH,
    height: EYES_HEIGHT,
    x_offset: 15,
    y_offset: 0,
};
*/

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
    let (x, y) = (x as usize, y as usize);
    let row_width = CAT_WIDTH.wrapping_div(8) + if CAT_WIDTH.wrapping_rem(8) > 0 { 1 } else { 0 };
    let byte_column = x.wrapping_div(8) + 1;
    let bit_column = x.wrapping_rem(8) as u32;
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
    let wd = (wd + 1.0) / 2.0;
    loop {
        plot(buffer, x0, y0);
        e2 = err;
        x2 = x0;
        if 2 * e2 >= -dx {
            /* x step */
            e2 += dy;
            y2 = y0;
            while e2 < ((ed as f32 * wd) as i32) && (y1 <= y2 || dx > dy) {
                y2 += sy;
                plot(buffer, x0, y2);
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
            e2 = dx - e2;
            while e2 < ((ed as f32 * wd) as i32) && (x1 != x2 || dx < dy) {
                x2 += sx;
                plot(buffer, x2, y0);
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

fn create_tail_pixmap(t: f32) -> Vec<u8> {
    /*  Pendulum parameters */
    let mut sin_theta: f32;
    let mut cos_theta: f32;
    const A: f32 = 0.4;
    let omega: f32 = 1.0;
    let phi: f32 = 3.0 * FRAC_PI_2;
    let mut angle: f32;

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

fn main() {
    let azure_blue = from_u8_rgb(0, 127, 255);
    let white = from_u8_rgb(255, 255, 255);
    let black = 0;

    let mut buffer: Vec<u32> = vec![white; CAT_WIDTH * CAT_HEIGHT];
    let mut tails: Vec<Vec<u8>> = vec![];

    for i in 0..NUM_TAILS {
        tails.push(create_tail_pixmap(i as f32 * PI / (NUM_TAILS as f32)));
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
