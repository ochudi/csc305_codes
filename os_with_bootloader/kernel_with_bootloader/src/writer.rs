mod constants;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use core::{fmt, ptr};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    color: [u8; 4],
}

impl FrameBufferWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            color: [255, 255, 255, 255],
        };
        logger.clear();
        logger
    }

    //? CA Question A (1)
    pub fn set_pos(&mut self, height: usize, width: usize) {
        self.x_pos += width;
        self.y_pos += height;
    }

    pub fn set_color(&mut self, color: [u8; 4]) {
        self.color = color;
    }

    fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    //? CA Question B (i)
    pub fn backspace(&mut self) {
        if self.x_pos > BORDER_PADDING {
            self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
        } else {
            if self.y_pos
                >= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING + BORDER_PADDING
            {
                self.x_pos = self.width() - (font_constants::CHAR_RASTER_WIDTH + LETTER_SPACING);
                self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
            } else {
                return;
            }
        }

        for u in 0..font_constants::CHAR_RASTER_HEIGHT.val() {
            for w in 0..font_constants::CHAR_RASTER_WIDTH {
                self.write_pixel(self.x_pos + w, self.y_pos + u, 0);
            }
        }
    }

    //? Extras
    pub fn arrow_up(&mut self) {
        if self.y_pos > BORDER_PADDING {
            self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        }
    }

    pub fn arrow_down(&mut self) {
        if self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING < self.height() {
            self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        }
    }

    pub fn arrow_left(&mut self) {
        if self.x_pos > BORDER_PADDING {
            self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
        }
    }

    pub fn arrow_right(&mut self) {
        if self.x_pos + font_constants::CHAR_RASTER_WIDTH < self.width() {
            self.x_pos += font_constants::CHAR_RASTER_WIDTH;
        }
    }

    pub fn tab(&mut self) {
        const TAB_WIDTH: usize = 4; // Number of characters to jump on tab

        let remaining_space = self.width() - self.x_pos;
        let tab_width = font_constants::CHAR_RASTER_WIDTH * TAB_WIDTH;

        if remaining_space >= tab_width {
            self.x_pos += tab_width;
        } else {
            self.newline();
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
