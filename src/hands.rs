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
//const VERTICES_IN_HANDS: i64 = 4; /*  Hands are triangles      */
//const SECOND_HAND_FRACT: i64 = 90; /*  Percentages of radius    */
//const MINUTE_HAND_FRACT: i64 = 70;
//const HOUR_HAND_FRACT: i64 = 40;
//const HAND_WIDTH_FRACT: i64 = 7;
//const SECOND_WIDTH_FRACT: i64 = 5;
//
//const SECOND_HAND_TIME: i64 = 30; /*  Update limit for second hand*/
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
pub fn draw_hand(
    buffer: &mut Image,
    length: i64,
    width: i64,
    _offset: i64,
    fraction_of_a_circle: f64,
) -> (i64, i64) {
    let width = width as f64;
    let length = length as f64;
    //let offset = offset as f64;

    let angle: f64;
    let cos_angle: f64;
    let sin_angle: f64;

    let ws: f64;
    let wc: f64;

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
    let center_point = ((FACE_WIDTH / 2) as i64, (FACE_HEIGHT / 2) as i64); //(5,50);

    let a = (
        center_point.0 + (length * sin_angle) as i64,
        center_point.1 - (length * cos_angle) as i64,
    );
    let b = (
        center_point.0 - (ws + wc) as i64,
        center_point.1 + (wc - ws) as i64,
    );
    let c = (
        center_point.0 - (ws - wc) as i64,
        center_point.1 + (wc + ws) as i64,
    );
    buffer.plot_line_width(a, b, 0.0); /* 1 ---- 2 */
    buffer.plot_line_width(b, c, 0.0); /* 2 ----- 3 */
    buffer.plot_line_width(c, a, 0.0); /* 3 ----- 1(4) */

    (
        (a.0 + b.0 + c.0) / 3 + center_point.0,
        (a.1 + b.1 + c.1) / 3 + center_point.1,
    )
}

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
    length: i64,
    width: i64,
    offset: i64,
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
    let center_point = ((FACE_WIDTH / 2) as i64, (FACE_HEIGHT / 2) as i64); //(5,50);
                                                                            /*1 ---- 2 */
    {
        let x0 = center_point.0 + (length * sin_angle) as i64;
        let y0 = center_point.1 - (length * cos_angle) as i64;
        let x1 = center_point.0 + (ms - wc) as i64;
        let y1 = center_point.1 - (mc + ws) as i64;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("1-2: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 2-----3 */
    {
        let x0 = center_point.0 + (ms - wc) as i64;
        let y0 = center_point.1 - (mc + ws) as i64;
        let x1 = center_point.0 + (offset * sin_angle) as i64;
        let y1 = center_point.1 - (offset * cos_angle) as i64;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("2-3: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 3-----4 */
    {
        let x0 = center_point.0 + (offset * sin_angle) as i64;
        let y0 = center_point.1 - (offset * cos_angle) as i64;
        let x1 = center_point.0 + (ms + wc) as i64;
        let y1 = center_point.1 - (mc - ws) as i64;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("3-4: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
    /* 4-----1 */
    {
        let x0 = center_point.0 + (ms + wc) as i64;
        let y0 = center_point.1 - (mc - ws) as i64;
        let x1 = center_point.0 + (length * sin_angle) as i64;
        let y1 = center_point.1 - (length * cos_angle) as i64;
        buffer.plot_line_width((x0, y0), (x1, y1), 1.0);
        //eprintln!("4-1: 0: {:?} 1: {:?}", (x0, y0), (x1, y1));
    }
}
