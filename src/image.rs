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
use std::collections::VecDeque;
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
    pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
        Image {
            bytes: vec![WHITE; width * height],
            width,
            height,
            x_offset,
            y_offset,
        }
    }
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
            self.plot(self.width as i64 - 1, i);
        }
        for i in 0..(self.width as _) {
            self.plot(i, self.height as i64 - 1);
            self.plot(i, 0);
        }
    }

    pub fn clear(&mut self) {
        for i in self.bytes.iter_mut() {
            *i = WHITE;
        }
    }

    pub fn copy(&mut self, other: &Image, x_offset: usize, y_offset: usize) {
        for y in y_offset..self.height {
            for x in x_offset..self.width {
                if let Some(p) = other.get((x - x_offset) as i64, (y - y_offset) as i64) {
                    if p == BLACK {
                        self.plot(x as i64, y as i64);
                    }
                }
            }
        }
    }
    pub fn plot(&mut self, x: i64, y: i64) {
        //std::dbg!((x, y));
        //std::dbg!(self.bytes.len());
        //std::dbg!(self.width);
        //std::dbg!(self.height);
        //std::dbg!(self.width * self.height);
        if x < 0 || y < 0 || y >= (self.height as i64) || x >= (self.width as i64) {
            if cfg!(debug_assertions) {
                eprintln!("invalid plot() coors: ({}, {})", x, y);
            }
            return;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        self.bytes[y * self.width + x] = BLACK;
    }

    pub fn get(&self, x: i64, y: i64) -> Option<u32> {
        //std::dbg!((x, y));
        //std::dbg!(self.bytes.len());
        //std::dbg!(self.width);
        //std::dbg!(self.height);
        //std::dbg!(self.width * self.height);
        if x < 0 || y < 0 || y >= (self.height as i64) || x >= (self.width as i64) {
            return None;
        }
        let (x, y): (usize, usize) = (x as _, y as _);
        Some(self.bytes[y * self.width + x])
    }

    pub fn plot_ellipse(
        &mut self,
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

    pub fn plot_line_width(&mut self, (x1, y1): (i64, i64), (x2, y2): (i64, i64), wd: f64) {
        /* Bresenham's line algorithm */
        let mut d;
        let mut x: i64;
        let mut y: i64;
        let ax: i64;
        let ay: i64;
        let sx: i64;
        let sy: i64;
        let dx: i64;
        let dy: i64;

        dx = x2 - x1;
        ax = (dx * 2).abs();
        sx = if dx > 0 { 1 } else { -1 };

        dy = y2 - y1;
        ay = (dy * 2).abs();
        sy = if dy > 0 { 1 } else { -1 };

        x = x1;
        y = y1;

        let b = if dy == 0 { -1 } else { dx / dy };
        let a = 1;
        let double_d = (wd * f64::sqrt((a * a + b * b) as f64)) as i64;
        let delta = double_d / 2;

        if ax > ay {
            /* x step */
            d = ay - ax / 2;
            loop {
                self.plot(x, y);
                {
                    let total = |_x| {
                        if dy == 0 {
                            _x - x1
                        } else {
                            _x - (y * dx) / dy + (y1 * dx) / dy - x1
                        }
                    };
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x += 1;
                        self.plot(_x, y);
                    }
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x -= 1;
                        self.plot(_x, y);
                    }
                }
                if x == x2 {
                    return;
                }
                if d >= 0 {
                    y = y + sy;
                    d = d - ax;
                }
                x = x + sx;
                d = d + ay;
            }
        } else {
            /* y step */
            d = ax - ay / 2;
            let delta = double_d / 3;
            loop {
                self.plot(x, y);
                {
                    let total = |_x| {
                        if dy == 0 {
                            _x - x1
                        } else {
                            _x - (y * dx) / dy + (y1 * dx) / dy - x1
                        }
                    };
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x += 1;
                        self.plot(_x, y);
                    }
                    let mut _x = x;
                    loop {
                        let t = total(_x);
                        if t < -1 * delta || t >= delta {
                            break;
                        }
                        _x -= 1;
                        self.plot(_x, y);
                    }
                }
                if y == y2 {
                    return;
                }
                if d >= 0 {
                    x = x + sx;
                    d = d - ay;
                }
                y = y + sy;
                d = d + ax;
            }
        }
    }

    pub fn flood_fill(&mut self, x: i64, y: i64) {
        if self.get(x, y) != Some(WHITE) {
            return;
        }

        let w = self.width as i64;
        let h = self.height as i64;
        let mut span_above: bool;
        let mut span_below: bool;

        let mut s = VecDeque::new();
        s.push_back((x, y));

        while let Some((x, y)) = s.pop_back() {
            let mut x1 = x;
            while x1 >= 0 && self.get(x1, y).map(|some| some == WHITE).unwrap_or(false) {
                x1 -= 1;
            }
            x1 += 1;
            span_above = false;
            span_below = false;
            while x1 < w && self.get(x1, y).map(|some| some == WHITE).unwrap_or(false) {
                self.plot(x1, y);
                if !span_above
                    && y > 0
                    && self
                        .get(x1, y - 1)
                        .map(|some| some == WHITE)
                        .unwrap_or(false)
                {
                    s.push_back((x1, y - 1));
                    span_above = true;
                } else if span_above
                    && y > 0
                    && self
                        .get(x1, y - 1)
                        .map(|some| some != WHITE)
                        .unwrap_or(false)
                {
                    span_above = false;
                }
                if !span_below
                    && y < h - 1
                    && self
                        .get(x1, y + 1)
                        .map(|some| some == WHITE)
                        .unwrap_or(false)
                {
                    s.push_back((x1, y + 1));
                    span_below = true;
                } else if span_below
                    && y < h - 1
                    && self
                        .get(x1, y + 1)
                        .map(|some| some != WHITE)
                        .unwrap_or(false)
                {
                    span_below = false;
                }
                x1 += 1;
            }
        }
    }
}
