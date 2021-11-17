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

use super::*;
pub struct Image {
    pub bytes: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub x_offset: usize,
    pub y_offset: usize,
}

impl From<Bitmap<'_>> for Image {
    fn from(val: Bitmap<'_>) -> Image {
        Image {
            bytes: bits_to_bytes(val.bits, val.width),
            width: val.width,
            height: val.height,
            x_offset: val.x_offset,
            y_offset: val.y_offset,
        }
    }
}

impl Image {
    pub fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bytes[y * self.width + x] == BLACK {
                    buffer[(self.y_offset + y) * CAT_WIDTH + self.x_offset + x] = fg;
                } else if let Some(bg) = bg {
                    buffer[(self.y_offset + y) * CAT_WIDTH + self.x_offset + x] = bg;
                }
            }
        }
    }

    pub fn draw_outline(&mut self) {
        for i in 0..(self.height as _) {
            self.plot(0, i);
            self.plot(self.width as i32 - 1, i);
        }
        for i in 0..(self.width as _) {
            self.plot(i, self.height as i32 - 1);
            self.plot(i, 0);
        }
    }

    pub fn clear(&mut self) {
        for i in self.bytes.iter_mut() {
            *i = WHITE;
        }
    }

    pub fn plot(&mut self, x: i32, y: i32) {
        //std::dbg!((x, y));
        //std::dbg!(self.bytes.len());
        //std::dbg!(self.width);
        //std::dbg!(self.height);
        //std::dbg!(self.width * self.height);
        if x < 0 || y < 0 {
            eprintln!("invalid plot() coors: ({}, {})", x, y);
            return;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        self.bytes[y * self.width + x] = BLACK;
    }

    pub fn get(&mut self, x: i32, y: i32) -> u32 {
        //std::dbg!((x, y));
        //std::dbg!(self.bytes.len());
        //std::dbg!(self.width);
        //std::dbg!(self.height);
        //std::dbg!(self.width * self.height);
        if x < 0 || y < 0 {
            eprintln!("invalid plot() coors: ({}, {})", x, y);
            return WHITE;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        self.bytes[y * self.width + x]
    }

    pub fn plot_ellipse(
        &mut self,
        (xm, ym): (i32, i32),
        (a, b): (i32, i32),
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
                self.plot(xm - x, ym + y); /*   I. Quadrant */
            }
            if quadrants[1] {
                self.plot(xm + x, ym + y); /*  II. Quadrant */
            }
            if quadrants[2] {
                self.plot(xm + x, ym - y); /* III. Quadrant */
            }
            if quadrants[3] {
                self.plot(xm - x, ym - y); /*  IV. Quadrant */
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
            self.plot(xm, ym + y); /* -> finish tip of ellipse */
            self.plot(xm, ym - y);
        }
    }

    pub fn plot_line_width(&mut self, (mut x0, mut y0): (i32, i32), (x1, y1): (i32, i32), wd: f64) {
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
        let mut points = vec![];
        let wd = (wd + 1.0) / 2.0;
        //eprintln!("wd = {}, ed = {}", wd, ed);
        loop {
            points.push((x0, y0));
            self.plot(x0, y0);
            e2 = err;
            x2 = x0;
            if 2 * e2 >= -dx {
                /* x step */
                //eprintln!(" x step ");
                e2 += dy;
                y2 = y0;
                while e2 < ((ed as f64 * wd) as i32) && (y1 != y2 || dx > dy) {
                    y2 += sy;
                    self.plot(x0, y2);
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
                    self.plot(x2, y0);
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

    pub fn flood_fill(&mut self, mut x: i32, y: i32) {
        eprintln!("flood fill x, y {:?}", (x, y));
        if self.get(x, y) != WHITE {
            return;
        }
        let mut s = Vec::new();
        s.push((x, x, y + 1, 1));
        s.push((x, x, y, -1));
        while let Some((mut x1, x2, y, dy)) = s.pop() {
            x = x1;
            if self.get(x, y) == WHITE {
                //Inside(x, y):
                while self.get(x - 1, y) == WHITE {
                    // Inside(x - 1, y):
                    self.plot(x - 1, y);
                    x = x - 1;
                }
            }
            if x < x1 {
                s.push((x, x1 - 1, y - dy, -dy));
            }
            while x1 < x2 {
                while self.get(x1, y) == WHITE {
                    //Inside(x1, y):
                    self.plot(x1, y);
                    x1 = x1 + 1;
                    s.push((x, x1 - 1, y + dy, dy));
                    if x1 - 1 > x2 {
                        s.push((x2 + 1, x1 - 1, y - dy, -dy));
                    }
                    while x1 < x2 && self.get(x1, y) != WHITE {
                        x1 = x1 + 1;
                    }
                    x = x1;
                }
            }
        }
    }
}
