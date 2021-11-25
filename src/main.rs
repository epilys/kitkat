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

use minifb::{Key, Window, WindowOptions};
use std::f64;
use std::f64::consts::{FRAC_PI_2, PI};
use std::time::{Duration, Instant, SystemTime};

mod image;
pub use image::*;
mod draw;
pub use draw::*;
mod date;
mod hands;
mod moonphase;

pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub(crate) const AZURE_BLUE: u32 = from_u8_rgb(0, 127, 255);
pub(crate) const _RED: u32 = from_u8_rgb(157, 37, 10);
pub(crate) const WHITE: u32 = from_u8_rgb(255, 255, 255);
pub(crate) const MOON: u32 = from_u8_rgb(0xd4, 0xc6, 0xa8);
pub(crate) const MOONDARK: u32 = from_u8_rgb(0x59, 0x53, 0x45);
pub(crate) const SUN: u32 = from_u8_rgb(0xff, 0xeb, 0x3b);
pub(crate) const SUNDARK: u32 = from_u8_rgb(0xff, 0xa3, 0x01);
pub(crate) const BLACK: u32 = 0;

#[derive(Clone, Copy)]
struct Bitmap<'bits> {
    bits: &'bits [u8],
    width: usize,
    height: usize,
    x_offset: usize,
    y_offset: usize,
}

#[inline(always)]
pub fn pixel_width_to_bits_width(i: usize) -> usize {
    i.wrapping_div(8) + if i.wrapping_rem(8) > 0 { 1 } else { 0 }
}

pub fn bits_to_bytes(bits: &[u8], width: usize) -> Vec<u32> {
    let mut ret = Vec::with_capacity(bits.len() * 8);
    let mut current_row_count = 0;
    for byte in bits {
        for n in 0..8 {
            if byte.rotate_right(n) & 0x01 > 0 {
                ret.push(BLACK);
            } else {
                ret.push(WHITE);
            }
            current_row_count += 1;
            if current_row_count == width {
                current_row_count = 0;
                break;
            }
        }
    }
    ret
}

impl<'bits> Bitmap<'bits> {
    /*
    fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>) {
        let row_width = pixel_width_to_bits_width(self.width);
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
            i += self.width + self.x_offset;
        }
        //debug_assert_eq!(i, self.width * self.height);
    }
    */
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
    x_offset: CAT_WIDTH / 2,
    y_offset: 30,
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

fn create_eye_pixmap(t: f64) -> Image {
    macro_rules! tr {
        ($cond:expr ,? $then:expr ,: $else:expr) => {
            if $cond {
                $then
            } else {
                $else
            }
        };
    }
    let mut ret = Image {
        bytes: vec![WHITE; 30 * 60],
        width: 60,
        height: 30,
        x_offset: 47,
        y_offset: 30,
    };

    //ret.draw_outline();

    const A: f64 = 0.7;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut u: f64;
    let w: f64 = FRAC_PI_2;
    /*  Sphere parameters    */
    /*  Radius               */
    let r: f64 = 1.0;
    /*  Center of sphere     */
    let x0: f64 = 0.0;
    let y0: f64 = 0.0;
    let z0: f64 = 2.0;

    let angle: f64 = A * f64::sin(omega * t + phi) + w;
    let mut points: Vec<(i64, i64)> = Vec::with_capacity(100);

    let mut i = 0;
    u = -1.0 * FRAC_PI_2;
    while u < FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle + PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle + PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i64;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i64;
        points.push((a, b));
        u += 0.25;
        i += 1;
    }

    u = FRAC_PI_2;
    while u > -1.0 * FRAC_PI_2 {
        let x = x0 + r * f64::cos(u) * f64::cos(angle - PI / 7.0);
        let z = z0 + r * f64::cos(u) * f64::sin(angle - PI / 7.0);
        let y = y0 + r * f64::sin(u);

        let a = ((tr!(z == 0.0 ,? x ,: x / z) * 23.0) + 12.0) as i64;
        let b = ((tr!(z == 0.0 ,? y ,: y / z) * 23.0) + 11.0) as i64;
        points.push((a, b));
        u -= 0.25;
        i += 1;
    }

    let (mut cx, mut cy) = points[0]; // calculate centroid of points
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        cx += point_b.0;
        cy += point_b.1;
        ret.plot_line_width(point_a, point_b, 0.);
    }
    let n = points.len() as i64;
    ret.flood_fill(cx / n, cy / n);
    for j in 0..i {
        points[j].0 += 31;
    }
    let (mut cx, mut cy) = points[0]; // calculate centroid of points
    for window in points.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        cx += point_b.0;
        cy += point_b.1;
        ret.plot_line_width(point_a, point_b, 0.);
    }
    ret.flood_fill(cx / n, cy / n);

    ret
}

fn create_tail_image(t: f64) -> Image {
    /*  Pendulum parameters */
    let sin_theta: f64;
    let cos_theta: f64;
    const A: f64 = 0.4;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let angle: f64;

    //    static XPoint tailOffset = { 74, -15 };
    const TAIL_WIDTH: usize = 90;
    const TAIL_HEIGHT: usize = 80;
    const TAIL_OFFSET: (i64, i64) = ((TAIL_WIDTH / 2) as i64, 0);
    let mut center_tail: Vec<(i64, i64)> = vec![(0, 0); 3]; /* center tail    */
    let mut new_tail: Vec<(i64, i64)> = vec![(0, 0); 3]; /*  Tail at time "t"  */

    {
        /*
         *  Create an "center" tail.
         */
        center_tail[0] = (0, 0);
        center_tail[1] = (3, (TAIL_HEIGHT - 15) as i64);
        center_tail[2] = (20, (TAIL_HEIGHT - 15) as i64);
    }

    /*
     *  Compute pendulum function.
     */
    angle = A * f64::sin(omega * t + phi);
    sin_theta = f64::sin(angle);
    cos_theta = f64::cos(angle);

    let mut buf = Image::new(
        TAIL_WIDTH,
        TAIL_HEIGHT,
        CAT_WIDTH / 2 - TAIL_WIDTH / 2,
        TAIL.y_offset,
    );
    /*
     *  Rotate the center tail about its origin by "angle" degrees.
     */
    for i in 0..3 {
        new_tail[i].0 = ((center_tail[i].0 as f64) * cos_theta
            + ((center_tail[i].1 as f64) * sin_theta)) as i64;
        new_tail[i].1 = ((center_tail[i].0 as f64 * -1.0) * sin_theta
            + ((center_tail[i].1 as f64) * cos_theta)) as i64;

        new_tail[i].0 += TAIL_OFFSET.0;
        new_tail[i].1 += TAIL_OFFSET.1;
    }

    const WIDTH: f64 = 17.0;
    const WIDTH2: f64 = WIDTH / 2.0;
    buf.plot_line_width(new_tail[0], new_tail[1], 0.0);
    buf.plot_line_width(new_tail[1], new_tail[2], 0.0);
    buf.plot_line_width(new_tail[2], new_tail[0], 0.0);

    let center = (
        (new_tail[0].0 + new_tail[1].0 + new_tail[2].0) / 3,
        (new_tail[0].1 + new_tail[1].1 + new_tail[2].1) / 3,
    );
    buf.flood_fill(center.0, center.1);

    let (xa, ya) = new_tail[1];
    let (xb, yb) = new_tail[2];
    let last_point = ((xa + xb) / 2, (ya + yb) / 2);
    buf.plot_ellipse(
        last_point,
        (WIDTH2 as i64, WIDTH2 as i64),
        [true, true, true, true],
        1.0,
    );
    buf.flood_fill(last_point.0 + 5, last_point.1 + 5);
    buf
}

fn create_tail_image_hook(t: f64) -> Image {
    /*  Pendulum parameters */
    let mut sin_theta: f64;
    let mut cos_theta: f64;
    const A: f64 = 0.4;
    let omega: f64 = 1.0;
    let phi: f64 = 3.0 * FRAC_PI_2;
    let mut angle: f64;

    //    static XPoint tailOffset = { 74, -15 };
    const TAIL_OFFSET: (i64, i64) = (72, 0);
    const CENTER_TAIL: [(i64, i64); N_TAIL_PTS] = [
        /*  "Center" tail points definition */
        (0, 0),
        (0, 76),
        (3, 82),
        (10, 84),
        (18, 82),
        (21, 76),
        (21, 70),
    ];

    let mut off_center_tail: Vec<(i64, i64)> = vec![(0, 0); N_TAIL_PTS]; /* off center tail    */
    let mut new_tail: Vec<(i64, i64)> = vec![(0, 0); N_TAIL_PTS]; /*  Tail at time "t"  */

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
                as i64;
            off_center_tail[i].1 = ((-1.0 * (CENTER_TAIL[i].0 as f64)) * sin_theta
                + ((CENTER_TAIL[i].1 as f64) * cos_theta))
                as i64;
        }
    }

    /*
     *  Compute pendulum function.
     */
    angle = A * f64::sin(omega * t + phi);
    sin_theta = f64::sin(angle);
    cos_theta = f64::cos(angle);

    let mut ret = TAIL.bits.to_vec();
    let mut buf = Buffer {
        vec: &mut ret,
        row_width: TAIL.width,
        height: TAIL.height,
    };
    /*
     *  Rotate the center tail about its origin by "angle" degrees.
     */
    for i in 0..N_TAIL_PTS {
        new_tail[i].0 = ((off_center_tail[i].0 as f64) * cos_theta
            + ((off_center_tail[i].1 as f64) * sin_theta)) as i64;
        new_tail[i].1 = ((off_center_tail[i].0 as f64 * -1.0) * sin_theta
            + ((off_center_tail[i].1 as f64) * cos_theta)) as i64;

        new_tail[i].0 += TAIL_OFFSET.0;
        new_tail[i].1 += TAIL_OFFSET.1;
    }

    const WIDTH: f64 = 15.0;
    const WIDTH2: f64 = WIDTH / 2.0;
    for window in new_tail.as_slice().windows(2) {
        let point_a = window[0];
        let point_b = window[1];
        plot_line_with_width(&mut buf, point_a, point_b, WIDTH as _);
    }

    let mut last_point = *new_tail.last().unwrap();
    last_point.1 += 1;
    for b in 0..=((0.8 * WIDTH2) as i64) {
        plot_ellipse(
            &mut buf,
            last_point,
            (WIDTH2 as i64, b),
            [false, false, true, true],
            1.0,
        );
    }

    Image::from(Bitmap { bits: &ret, ..TAIL })
}

const HELP: &str = r#"Usage: kitkat [--hook|--crazy|--offset OFFSET|--borderless|--resize|--sunmoon|--moon|--date]

Displays a kit kat clock with the system time, or the system time with given offset if the --offset
argument is provided.

      --hook                 show a hooked tail instead of the default drop shaped one
      --crazy                go faster for each time this argument is invoked
      --offset OFFSET        add OFFSET to current system time (only the first given
                             offset will be used)
      --borderless
      --resize
      --sunmoon              show sun or moon phase depending on the hour
      --moon                 show only moon phase
      --date                 show month date

      OFFSET format is [+-]{0,1}\d\d:\d\d, e.g: 02:00 or -03:45 or +00:00
"#;

fn main() {
    let time = unsafe { libc::time(std::ptr::null_mut()) };
    let mut tm = std::mem::MaybeUninit::<libc::tm>::uninit();
    unsafe {
        libc::localtime_r(&time as *const _, tm.as_mut_ptr());
    }
    let tm = unsafe { tm.assume_init() };
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    if !args.is_empty() && args.iter().any(|s| s == "--help") {
        if args.iter().any(|s| s != "--help") {
            eprintln!(
                "WARNING: Ignoring other arguments and startup because --help was specified."
            );
        }
        println!("{}", HELP);
        return;
    }

    let borderless = !args.is_empty() && args.iter().any(|s| s == "--borderless");
    let resize = !args.is_empty() && args.iter().any(|s| s == "--resize");
    let sunmoon = !args.is_empty() && args.iter().any(|s| s == "--sunmoon");
    let moon = !args.is_empty() && args.iter().any(|s| s == "--moon");
    if sunmoon && moon {
        eprintln!("ERROR: You can't use both --sunmoon and --moon.");
        return;
    }
    let show_date = !args.is_empty() && args.iter().any(|s| s == "--date");

    let mut tail_kind: fn(_) -> _ = create_tail_image;
    let crazy: usize = args.iter().filter(|s| *s == "--crazy").count();
    if !args.is_empty() && args.iter().any(|s| s == "--hook") {
        tail_kind = create_tail_image_hook;
    }
    let mut offset_sign = true;
    let mut offset_hour = 0;
    let mut offset_min = 0;
    if let Some(pos) = args.iter().position(|s| s == "--offset") {
        if let Some(offset) = args.iter().nth(pos + 1) {
            let mut offset = offset.as_str();
            if offset.starts_with("+") {
                offset = &offset[1..];
            } else if offset.starts_with("-") {
                offset_sign = false;
                offset = &offset[1..];
            }

            if offset.len() != "00:00".len()
                || offset.as_bytes()[2] != b':'
                || !offset.as_bytes()[0..2]
                    .iter()
                    .chain(offset.as_bytes()[3..].iter())
                    .all(|b| b.is_ascii_digit())
            {
                eprintln!("Wrong format for time offset, must be [+-]{{0,1}}\\d\\d:\\d\\d, e.g. 02:00 or -03:45 or +00:00");
                return;
            }
            offset_hour = offset[0..2].parse::<usize>().unwrap();
            offset_min = offset[3..].parse::<usize>().unwrap();
            if offset_min >= 60 {
                eprintln!("Wrong format for time offset, minute must be < 60");
                return;
            }
            if offset_hour >= u8::MAX as usize {
                eprintln!(
                    "Wrong format for time offset, hour must be a reasonable value, i.e. < {}",
                    u8::MAX
                );
                return;
            }
        } else {
            eprintln!("--offset requires a value with a certain format: [+-]{{0,1}}\\d\\d:\\d\\d");
            return;
        }
    }
    let mut buffer: Vec<u32> = vec![WHITE; CAT_WIDTH * CAT_HEIGHT];
    let mut tails_frames: Vec<Image> = Vec::with_capacity(NUM_TAILS);
    let mut eyes_frames: Vec<Image> = Vec::with_capacity(NUM_TAILS);

    for i in 0..NUM_TAILS {
        tails_frames.push(tail_kind(i as f64 * PI / (NUM_TAILS as f64)));
        eyes_frames.push(create_eye_pixmap(i as f64 * PI / (NUM_TAILS as f64)));
    }

    let mut window = Window::new(
        "kitkat - ESC or q to exit",
        CAT_WIDTH,
        CAT_HEIGHT,
        WindowOptions {
            title: true,
            borderless,
            resize,
            transparency: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let catwhite = Image::from(CATWHITE);
    catwhite.draw(&mut buffer, WHITE, Some(WHITE));
    let catback = Image::from(CATBACK);
    catback.draw(&mut buffer, BLACK, None);
    let cattie = Image::from(CATTIE);
    cattie.draw(&mut buffer, AZURE_BLUE, None);
    let tail = Image::from(TAIL);
    tail.draw(&mut buffer, BLACK, None);
    let eyes = Image::from(EYES);
    eyes.draw(&mut buffer, BLACK, None);

    //CATTIE.draw(&mut buffer, AZURE_BLUE, None);
    //TAIL.draw(&mut buffer, black, None);
    //EYES.draw(&mut buffer, black, None);
    /*
     */

    let _blank_face = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut hour_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut minute_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };
    let mut second_hand = Image {
        bytes: vec![WHITE; hands::FACE_WIDTH * hands::FACE_HEIGHT],
        width: hands::FACE_WIDTH,
        height: hands::FACE_HEIGHT,
        x_offset: hands::FACE_OFFSET_X,
        y_offset: hands::FACE_OFFSET_Y,
    };

    const SECOND_HAND_WIDTH: i64 = 23;
    const SECOND_HAND_HEIGHT: i64 = 1;

    const MINUTE_HAND_WIDTH: i64 = 22;
    const MINUTE_HAND_HEIGHT: i64 = 3;

    const HOUR_HAND_WIDTH: i64 = 16;
    const HOUR_HAND_HEIGHT: i64 = 2;

    let mut i: usize = 0;
    let mut prev_i = i;
    let mut up = true;
    let mut system_now_second;
    let mut now_second = Instant::now();
    let mut seconds;
    let _now = SystemTime::now();
    let mut hour: u8 = tm.tm_hour as _;
    let mut minutes: u8 = tm.tm_min as _;
    add_time_offset(
        &mut hour,
        &mut minutes,
        offset_sign,
        offset_hour,
        offset_min,
    );
    let mut passed_seconds = tm.tm_sec as _;
    hands::draw_second(
        &mut hour_hand,
        HOUR_HAND_WIDTH,
        HOUR_HAND_HEIGHT,
        -5,
        (hour as f64) / 12.0,
    );
    hour_hand.draw(&mut buffer, BLACK, None);
    hands::draw_second(
        &mut second_hand,
        SECOND_HAND_WIDTH,
        SECOND_HAND_HEIGHT,
        -5,
        (passed_seconds as f64) / 60.0,
    );
    second_hand.draw(&mut buffer, BLACK, None);
    hands::draw_second(
        &mut minute_hand,
        MINUTE_HAND_WIDTH,
        MINUTE_HAND_HEIGHT,
        -5,
        (minutes as f64) / 60.0,
    );
    minute_hand.draw(&mut buffer, BLACK, None);

    let sun: Image = moonphase::sun();
    let sun_bg: Image = moonphase::sun_background();

    let moon_corners: Image = moonphase::corner_fill();
    let full_moon: Image = moonphase::MoonPosition::FullMoon.into();
    let moon_phase: Image = moonphase::phase(moonphase::position(None)).into();

    let date: Image = date::make_date(tm.tm_mday as i64);

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        let cur_tail = &tails_frames[i];
        tail.draw(&mut buffer, BLACK, Some(WHITE));
        cur_tail.draw(&mut buffer, BLACK, None);
        let cur_eyes = &eyes_frames[i];
        eyes_frames[prev_i].draw(&mut buffer, WHITE, None);
        prev_i = i;
        cur_eyes.draw(&mut buffer, BLACK, None);

        let new_now_second = Instant::now();

        if crazy > 0 || new_now_second - now_second >= Duration::from_secs(1) {
            passed_seconds += 1;
            now_second = new_now_second;
            system_now_second = SystemTime::now();
            seconds = system_now_second
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            //blank_face.draw(&mut buffer, WHITE, Some(WHITE));
            second_hand.draw(&mut buffer, WHITE, None);
            second_hand.clear();
            if crazy > 0 {
                passed_seconds += 6 * (crazy as u64);
                seconds = passed_seconds;
            }
            hands::draw_second(
                &mut second_hand,
                SECOND_HAND_WIDTH,
                SECOND_HAND_HEIGHT,
                -5,
                (seconds as f64) / 60.0,
            );
            second_hand.draw(&mut buffer, BLACK, None);
        }
        if crazy > 0 || passed_seconds >= 60 {
            passed_seconds = 0;
            minutes += 1;
            if minutes == 60 {
                minutes = 0;
                hour += 1;
                if hour == 24 {
                    hour = 0;
                }
            }
            minute_hand.draw(&mut buffer, WHITE, None);
            minute_hand.clear();
            hands::draw_second(
                &mut minute_hand,
                MINUTE_HAND_WIDTH,
                MINUTE_HAND_HEIGHT,
                -5,
                (minutes as f64) / 60.0,
            );
        }
        minute_hand.draw(&mut buffer, BLACK, None);
        //blank_face.draw(&mut buffer, WHITE, Some(WHITE));
        hour_hand.draw(&mut buffer, WHITE, None);
        hour_hand.clear();
        if show_date {
            date.draw(&mut buffer, BLACK, None);
        }
        hands::draw_second(
            &mut hour_hand,
            HOUR_HAND_WIDTH,
            HOUR_HAND_HEIGHT,
            -5,
            (hour as f64) / 12.0,
        );
        hour_hand.draw(&mut buffer, BLACK, None);

        if moon || sunmoon {
            // FIXME: use the https://en.wikipedia.org/wiki/Sunrise_equation to calc sunrise times
            // by having user provide latitude
            if moon || hour < 8 || hour > 18 {
                moon_corners.draw(&mut buffer, BLACK, Some(WHITE));
                full_moon.draw(&mut buffer, MOONDARK, None);
                moon_phase.draw(&mut buffer, MOON, None);
            } else if sunmoon {
                sun_bg.draw(&mut buffer, SUNDARK, None);
                sun.draw(&mut buffer, SUN, None);
            }
        }

        if crazy > 0 {
            for _ in i..(crazy + i) {
                if up {
                    if i + 1 == tails_frames.len() {
                        up = false;
                    } else {
                        i = (i + 1).wrapping_rem(tails_frames.len());
                    }
                } else {
                    if i == 0 {
                        up = true;
                    } else {
                        i -= 1;
                    }
                }
            }
        } else {
            if up {
                if i + 1 == tails_frames.len() {
                    up = false;
                } else {
                    i = (i + 1).wrapping_rem(tails_frames.len());
                }
            } else {
                if i == 0 {
                    up = true;
                } else {
                    i -= 1;
                }
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

fn add_time_offset(
    hour: &mut u8,
    minutes: &mut u8,
    offset_sign: bool,
    offset_hour: usize,
    offset_min: usize,
) {
    match (offset_sign, (offset_hour, offset_min)) {
        (false, (h, m)) if h > 0 || m > 0 => {
            let mut ih = *hour as i64;
            ih -= h as i64;
            let mut im = *minutes as i64;
            im -= m as i64;
            if im < 0 {
                ih -= if (im.abs() % 59) > 0 { 1 } else { 0 };
                ih %= 23;
            }
            if im < 0 {
                *minutes = (60 + im) as u8;
            } else {
                *minutes = im as u8;
            }
            if ih < 0 {
                *hour = (24 + ih) as u8;
            } else {
                *hour = ih as u8;
            }
        }
        (true, (h, m)) if h > 0 || m > 0 => {
            *hour += h as u8;
            *hour %= 24;
            *minutes += m as u8;
            if *minutes >= 60 {
                *hour += *minutes / 60;
                *hour %= 24;
            }
            *minutes %= 60;
        }
        _ => {}
    }
}

#[test]
fn test_time_offset() {
    let mut hour: u8 = 14;
    let mut minutes: u8 = 32;

    add_time_offset(&mut hour, &mut minutes, true, 0, 0);
    assert_eq!((hour, minutes), (14, 32));

    add_time_offset(&mut hour, &mut minutes, false, 0, 0);
    assert_eq!((hour, minutes), (14, 32));

    add_time_offset(&mut hour, &mut minutes, true, 2, 0);
    assert_eq!((hour, minutes), (16, 32));

    hour = 14;
    minutes = 32;

    add_time_offset(&mut hour, &mut minutes, true, 2, 32);
    assert_eq!((hour, minutes), (17, 04));

    hour = 14;
    minutes = 32;

    add_time_offset(&mut hour, &mut minutes, false, 2, 0);
    assert_eq!((hour, minutes), (12, 32));

    hour = 14;
    minutes = 32;

    add_time_offset(&mut hour, &mut minutes, false, 2, 33);
    assert_eq!((hour, minutes), (11, 59));

    hour = 14;
    minutes = 32;

    add_time_offset(&mut hour, &mut minutes, true, 24, 59);
    assert_eq!((hour, minutes), (15, 31));
}
