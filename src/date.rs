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

const DATE_WIDTH: usize = 10;

include!("letters/kitkat/0.xbm.rs");
include!("letters/kitkat/1.xbm.rs");
include!("letters/kitkat/2.xbm.rs");
include!("letters/kitkat/3.xbm.rs");
include!("letters/kitkat/4.xbm.rs");
include!("letters/kitkat/5.xbm.rs");
include!("letters/kitkat/6.xbm.rs");
include!("letters/kitkat/7.xbm.rs");
include!("letters/kitkat/8.xbm.rs");
include!("letters/kitkat/9.xbm.rs");

pub fn make_date(mday: i64) -> Image {
    let mut ret = Image::new(
        DATE_WIDTH + 2,
        DATE_WIDTH + 2,
        CAT_WIDTH / 2 - DATE_WIDTH / 2 + 1,
        TAIL.y_offset - 4 * DATE_WIDTH,
    );

    ret.draw_outline();
    if mday < 31 {
        let mday = mday.to_string();
        let first_digit: Option<Image> = if mday.len() == 1 {
            None
        } else {
            match &mday[0..1] {
                "1" => {
                    let mut first_digit = Image {
                        bytes: vec![],
                        width: 0,
                        height: 0,
                        y_offset: 0,
                        x_offset: 1,
                    };
                    first_digit.bytes = bits_to_bytes(_1_BITS, _1_WIDTH);
                    first_digit.width = _1_WIDTH;
                    first_digit.height = _1_HEIGHT;
                    Some(first_digit)
                }
                "2" => {
                    let mut first_digit = Image {
                        bytes: vec![],
                        width: 0,
                        height: 0,
                        y_offset: 0,
                        x_offset: 1,
                    };
                    first_digit.bytes = bits_to_bytes(_2_BITS, _2_WIDTH);
                    first_digit.width = _2_WIDTH;
                    first_digit.height = _2_HEIGHT;
                    Some(first_digit)
                }
                _ => unreachable!(),
            }
        };
        let mut second_digit = Image {
            bytes: vec![],
            width: 0,
            height: 0,
            y_offset: 0,
            x_offset: 0,
        };
        let second_digit: Image = match if mday.len() == 1 {
            &mday[0..1]
        } else {
            &mday[1..]
        } {
            "0" => {
                second_digit.bytes = bits_to_bytes(_0_BITS, _0_WIDTH);
                second_digit.width = _0_WIDTH;
                second_digit.height = _0_HEIGHT;
                second_digit
            }
            "1" => {
                second_digit.bytes = bits_to_bytes(_1_BITS, _1_WIDTH);
                second_digit.width = _1_WIDTH;
                second_digit.height = _1_HEIGHT;
                second_digit
            }
            "2" => {
                second_digit.bytes = bits_to_bytes(_2_BITS, _2_WIDTH);
                second_digit.width = _2_WIDTH;
                second_digit.height = _2_HEIGHT;
                second_digit
            }
            "3" => {
                second_digit.bytes = bits_to_bytes(_3_BITS, _3_WIDTH);
                second_digit.width = _3_WIDTH;
                second_digit.height = _3_HEIGHT;
                second_digit
            }
            "4" => {
                second_digit.bytes = bits_to_bytes(_4_BITS, _4_WIDTH);
                second_digit.width = _4_WIDTH;
                second_digit.height = _4_HEIGHT;
                second_digit
            }
            "5" => {
                second_digit.bytes = bits_to_bytes(_5_BITS, _5_WIDTH);
                second_digit.width = _5_WIDTH;
                second_digit.height = _5_HEIGHT;
                second_digit
            }
            "6" => {
                second_digit.bytes = bits_to_bytes(_6_BITS, _6_WIDTH);
                second_digit.width = _6_WIDTH;
                second_digit.height = _6_HEIGHT;
                second_digit
            }
            "7" => {
                second_digit.bytes = bits_to_bytes(_7_BITS, _7_WIDTH);
                second_digit.width = _7_WIDTH;
                second_digit.height = _7_HEIGHT;
                second_digit
            }
            "8" => {
                second_digit.bytes = bits_to_bytes(_8_BITS, _8_WIDTH);
                second_digit.width = _8_WIDTH;
                second_digit.height = _8_HEIGHT;
                second_digit
            }
            "9" => {
                second_digit.bytes = bits_to_bytes(_9_BITS, _9_WIDTH);
                second_digit.width = _9_WIDTH;
                second_digit.height = _9_HEIGHT;
                second_digit
            }
            _ => unreachable!(),
        };

        if let Some(ref first_digit) = first_digit {
            ret.copy(&first_digit, 1, 2);
            ret.copy(&second_digit, DATE_WIDTH / 2 + 1, 2);
        } else {
            ret.copy(&second_digit, DATE_WIDTH / 2 + 1, 2);
        }
    }

    ret
}
