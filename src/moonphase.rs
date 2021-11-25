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

const _2001_01_01: u64 = 978300000;

#[repr(u8)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum MoonPosition {
    /// 🌑
    NewMoon = 0,
    /// 🌒
    WaxingCrescent = 1,
    /// 🌓
    FirstQuarter = 2,
    /// 🌔
    WaxingGibbous = 3,
    /// 🌕
    FullMoon = 4,
    /// 🌖
    WaningGibbous = 5,
    /// 🌗
    LastQuarter = 6,
    /// 🌘
    WaningCrescent = 7,
}

pub fn corner_fill() -> Image {
    let mut ret = Image::new(
        MOON_WIDTH,
        MOON_WIDTH,
        CAT_WIDTH / 2 + CAT_WIDTH / 5 + 2,
        TAIL.y_offset - 4 * MOON_WIDTH,
    );
    /* fill corners */
    ret.plot_line_width((2, 0), (0, 2), 0.0);
    ret.flood_fill(0, 0);
    ret.plot_line_width((0, ret.height as i64 - 3), (2, ret.height as i64 - 1), 0.0);
    ret.flood_fill(0, ret.height as i64 - 1);

    ret.plot_line_width(
        (ret.width as i64 - 1, ret.height as i64 - 3),
        (ret.width as i64 - 3, ret.height as i64 - 1),
        0.0,
    );
    ret.flood_fill(ret.width as i64 - 1, ret.height as i64 - 1);

    ret.plot_line_width((ret.width as i64 - 3, 0), (ret.width as i64 - 1, 2), 0.0);
    ret.flood_fill(ret.width as i64 - 1, 0);
    ret
}

const MOON_WIDTH: usize = 16;
impl From<MoonPosition> for Image {
    fn from(val: MoonPosition) -> Image {
        use MoonPosition::*;
        let mut ret = Image::new(
            MOON_WIDTH,
            MOON_WIDTH,
            CAT_WIDTH / 2 + CAT_WIDTH / 5 + 2,
            TAIL.y_offset - 4 * MOON_WIDTH,
        );

        const CENTER: (i64, i64) = ((MOON_WIDTH / 2) as i64, (MOON_WIDTH / 2) as i64);
        match val {
            NewMoon => {
                // 🌑
                ret.plot_ellipse(CENTER, (6, 6), [true, true, true, true], 1.0);
            }
            WaxingCrescent => {
                // 🌒
                ret.plot_ellipse(CENTER, (6, 6), [true, false, false, true], 1.0);
                ret.plot_ellipse(
                    (CENTER.0 - 2, CENTER.1),
                    (5, 6),
                    [true, false, false, true],
                    1.0,
                );
                ret.flood_fill(CENTER.0 + 5, CENTER.1);
            }
            FirstQuarter => {
                // 🌓
                ret.plot_ellipse(CENTER, (6, 6), [true, false, false, true], 1.0);
                ret.plot_line_width((CENTER.0, CENTER.0 - 6), (CENTER.0, CENTER.0 + 6), 0.0);
                ret.flood_fill(CENTER.0 + 1, CENTER.1);
            }
            WaxingGibbous => {
                // 🌔
                ret.plot_ellipse(CENTER, (6, 6), [true, false, false, true], 1.0);
                ret.plot_ellipse(
                    (CENTER.0, CENTER.1),
                    (5, 6),
                    [false, true, true, false],
                    1.0,
                );
                ret.flood_fill(CENTER.0 + 1, CENTER.1);
            }
            FullMoon => {
                //🌕
                ret.plot_ellipse(CENTER, (6, 6), [true, true, true, true], 1.0);
                ret.flood_fill(CENTER.0, CENTER.1);
            }
            WaningGibbous => {
                //🌖
                ret.plot_ellipse(CENTER, (6, 6), [false, true, true, false], 1.0);
                ret.plot_ellipse(
                    (CENTER.0, CENTER.1),
                    (4, 6),
                    [true, false, false, true],
                    1.0,
                );
                ret.flood_fill(CENTER.0 + 1, CENTER.1);
            }
            LastQuarter => {
                // 🌗
                ret.plot_ellipse(CENTER, (6, 6), [false, true, true, false], 1.0);
                ret.plot_line_width((CENTER.0, CENTER.0 - 6), (CENTER.0, CENTER.0 + 6), 0.0);
                ret.flood_fill(CENTER.0 - 1, CENTER.1);
            }
            WaningCrescent => {
                // 🌘
                ret.plot_ellipse(CENTER, (6, 6), [false, true, true, false], 1.0);
                ret.plot_ellipse(
                    (CENTER.0 + 2, CENTER.1),
                    (5, 6),
                    [false, true, true, false],
                    1.0,
                );
                ret.flood_fill(CENTER.0 - 5, CENTER.1);
            }
        }
        ret
    }
}

pub fn position(now: Option<u64>) -> f64 {
    let now: u64 = now.unwrap_or(
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    );

    let diff = now - _2001_01_01;
    let days = diff / 86400;
    let lunations: f64 = 0.20439731 + ((days as f64) * 0.03386319269);
    lunations % 1.0
}

pub fn phase(pos: f64) -> MoonPosition {
    let idx = (pos * 8.) + 0.5;
    let idx = (idx as usize) & 7;
    match idx {
        0 => MoonPosition::NewMoon,
        1 => MoonPosition::WaxingCrescent,
        2 => MoonPosition::FirstQuarter,
        3 => MoonPosition::WaxingGibbous,
        4 => MoonPosition::FullMoon,
        5 => MoonPosition::WaningGibbous,
        6 => MoonPosition::LastQuarter,
        7 => MoonPosition::WaningCrescent,
        _ => unreachable!(),
    }
}

#[test]
fn test_position() {
    assert_eq!(position(Some(1637831210)), 0.6821471127699965);
    assert_eq!(
        phase(position(Some(1637831210))),
        MoonPosition::WaningGibbous
    );
}

pub fn sun() -> Image {
    let mut ret = Image::new(
        MOON_WIDTH,
        MOON_WIDTH,
        CAT_WIDTH / 2 + CAT_WIDTH / 5 + 2,
        TAIL.y_offset - 4 * MOON_WIDTH,
    );
    const CENTER: (i64, i64) = ((MOON_WIDTH / 2) as i64, (MOON_WIDTH / 2) as i64);
    ret.plot_ellipse(CENTER, (6, 6), [true, true, true, true], 1.0);
    ret.plot_line_width((MOON_WIDTH as i64 / 2, 0), (MOON_WIDTH as i64 / 2, 2), 1.0);
    ret.plot_line_width(
        (MOON_WIDTH as i64 / 2, MOON_WIDTH as i64 - 2),
        (MOON_WIDTH as i64 / 2, MOON_WIDTH as i64 - 1),
        1.0,
    );
    ret.plot_line_width(
        (1, MOON_WIDTH as i64 / 2 - 1),
        (1, MOON_WIDTH as i64 / 2),
        1.0,
    );
    ret.plot_line_width(
        (MOON_WIDTH as i64 - 2, MOON_WIDTH as i64 / 2 - 1),
        (MOON_WIDTH as i64 - 1, MOON_WIDTH as i64 / 2),
        1.0,
    );
    ret
}

pub fn sun_background() -> Image {
    let mut ret = Image::new(
        MOON_WIDTH,
        MOON_WIDTH,
        CAT_WIDTH / 2 + CAT_WIDTH / 5 + 2,
        TAIL.y_offset - 4 * MOON_WIDTH,
    );
    const CENTER: (i64, i64) = ((MOON_WIDTH / 2) as i64, (MOON_WIDTH / 2) as i64);
    ret.plot_ellipse(CENTER, (6, 6), [true, true, true, true], 1.0);
    ret.flood_fill(CENTER.0, CENTER.1);
    ret
}
