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
/*
 *  Clock hand stuff
 */
//const VERTICES_IN_HANDS: i32 = 4; /*  Hands are triangles      */
//const SECOND_HAND_FRACT: i32 = 90; /*  Percentages of radius    */
//const MINUTE_HAND_FRACT: i32 = 70;
//const HOUR_HAND_FRACT: i32 = 40;
//const HAND_WIDTH_FRACT: i32 = 7;
//const SECOND_WIDTH_FRACT: i32 = 5;
//
//const SECOND_HAND_TIME: i32 = 30; /*  Update limit for second hand*/
pub const FACE_WIDTH: usize = 80;
pub const FACE_HEIGHT: usize = 80;
pub const FACE_OFFSET_X: usize = CAT_WIDTH / 2 - FACE_WIDTH / 2 - 1;
pub const FACE_OFFSET_Y: usize = CAT_HEIGHT / 2 - FACE_HEIGHT / 2;
/*
 *  DrawHand - Draws a hand.
 *
 *  length is the maximum length of the hand.
 *  width is the half-width of the hand.
 *  fractionOfACircle is a fraction between 0 and 1 (inclusive) indicating
 *  how far around the circle (clockwise) from high noon.
 *
 */
/*
pub fn draw_hand(_buffer: &mut Buffer, _length: i32, width: i32, fraction_of_a_circle: f64) {
    let angle: f64;
    let cos_angle: f64;
    let sin_angle: f64;

    let ws: f64;
    let wc: f64;

    let mut _x: i32;
    let mut _y: i32;
    let mut _x1: i32;
    let mut _y1: i32;
    let mut _x2: i32;
    let mut _y2: i32;

    /*
     *  A full circle is 2 PI radians.
     *  Angles are measured from 12 o'clock, clockwise increasing.
     *  Since in X, +x is to the right and +y is downward:
     *
     *    x = x0 + r * sin(theta)
     *    y = y0 - r * cos(theta)
     *
     */
    angle = 2. * PI * fraction_of_a_circle;
    cos_angle = f64::cos(angle);
    sin_angle = f64::sin(angle);

    /*
     * Order of points when drawing the hand.
     *
     *        1,4
     *        / \
     *       /   \
     *      /     \
     *    2 ------- 3
     */
    wc = (width as f64) * cos_angle;
    ws = (width as f64) * sin_angle;
    /*
    SetSeg(x = centerX + Round(length * sinAngle),
           y = centerY - Round(length * cosAngle),
           x1 = centerX - Round(ws + wc),
           y1 = centerY + Round(wc - ws));  /* 1 ---- 2 */
    /* 2 */
    SetSeg(x1, y1,
           x2 = centerX - Round(ws - wc),
           y2 = centerY + Round(wc + ws));  /* 2 ----- 3 */

    SetSeg(x2, y2, x, y);    /* 3 ----- 1(4) */
    */
}
*/

/*
 *  DrawSecond - Draws the second hand (diamond).
 *
 *  length is the maximum length of the hand.
 *  width is the half-width of the hand.
 *  offset is direct distance from Center to tail end.
 *  fractionOfACircle is a fraction between 0 and 1 (inclusive) indicating
 *  how far around the circle (clockwise) from high noon.
 *
 */
pub fn draw_second(
    buffer: &mut Image,
    length: i32,
    width: i32,
    offset: i32,
    fraction_of_a_circle: f64,
) {
    let width = width as f64;
    let length = length as f64;
    let offset = offset as f64;
    let angle: f64;
    let cos_angle: f64;
    let sin_angle: f64;

    let ms: f64;
    let mc: f64;
    let ws: f64;
    let wc: f64;

    let mid: f64;
    let mut _x: i32;
    let mut _y: i32;

    /*
     *  A full circle is 2 PI radians.
     *  Angles are measured from 12 o'clock, clockwise increasing.
     *  Since in X, +x is to the right and +y is downward:
     *
     *    x = x0 + r * sin(theta)
     *    y = y0 - r * cos(theta)
     *
     */
    angle = 2. * PI * fraction_of_a_circle;
    cos_angle = f64::cos(angle);
    sin_angle = f64::sin(angle);

    /*
     * Order of points when drawing the hand.
     *
     *        1,5
     *        / \
     *       /   \
     *      /     \
     *    2<       >4
     *      \     /
     *       \   /
     *        \ /
     *    -    3
     *    |
     *    |
     *   offset
     *    |
     *    |
     *    -     + center
     */
    mid = (length + offset) / 2.;
    mc = mid * cos_angle;
    ms = mid * sin_angle;
    wc = width * cos_angle;
    ws = width * sin_angle;

    //buffer.draw_outline();
    let center_point = ((FACE_WIDTH / 2) as i32, (FACE_HEIGHT / 2) as i32); //(5,50);
                                                                            /*1 ---- 2 */
    {
        let x0 = center_point.0 + (length * sin_angle) as i32;
        let y0 = center_point.1 - (length * cos_angle) as i32;
        let x1 = center_point.0 + (ms - wc) as i32;
        let y1 = center_point.1 - (mc + ws) as i32;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("1-2: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 2-----3 */
    {
        let x0 = center_point.0 + (ms - wc) as i32;
        let y0 = center_point.1 - (mc + ws) as i32;
        let x1 = center_point.0 + (offset * sin_angle) as i32;
        let y1 = center_point.1 - (offset * cos_angle) as i32;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("2-3: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 3-----4 */
    {
        let x0 = center_point.0 + (offset * sin_angle) as i32;
        let y0 = center_point.1 - (offset * cos_angle) as i32;
        let x1 = center_point.0 + (ms + wc) as i32;
        let y1 = center_point.1 - (mc - ws) as i32;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("3-4: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 4-----1 */
    {
        let x0 = center_point.0 + (ms + wc) as i32;
        let y0 = center_point.1 - (mc - ws) as i32;
        let x1 = center_point.0 + (length * sin_angle) as i32;
        let y1 = center_point.1 - (length * cos_angle) as i32;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("4-1: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
}
