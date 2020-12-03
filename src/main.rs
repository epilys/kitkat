extern crate minifb;

use minifb::{Key, Window, WindowOptions};

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
                    if c == self.width {
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
    x_offset: 15,
    y_offset: 00,
};

fn main() {
    let mut buffer: Vec<u32> = vec![0; CAT_WIDTH * CAT_HEIGHT];
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

    let azure_blue = from_u8_rgb(0, 127, 255);
    let white = from_u8_rgb(255, 255, 255);
    let black = 0;
    CATWHITE.draw(&mut buffer, white, Some(white));
    CATBACK.draw(&mut buffer, black, None);
    CATTIE.draw(&mut buffer, azure_blue, None);
    //TAIL.draw(&mut buffer, black, None);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, CAT_WIDTH, CAT_HEIGHT)
            .unwrap();
    }
}
