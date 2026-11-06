use minifb::{Window, WindowOptions};

use crate::helpers::*;
use crate::memory::memory_bus::{VRAM_BEGIN, VRAM_END, OAM_BEGIN, OAM_END};


// LCDC register
const LCD_ENABLE: u8                              = 7; // 0 = Off; 1 = On
const WINDOW_MAP_SELECT: u8                       = 6; // 0 = 0x9800-0x9BFF; 1 = 0x9C00-0x9FFF
const WINDOW_ENABLE: u8                           = 5; // 0 = Off; 1 = On
const BACKGROUND_AND_WINDOW_TILE_DATA_SELECT: u8  = 4; // 0 = 0x8800-0x97FF; 1 = 0x8000-0x8FFF
const BACKGROUND_TILE_MAP_SELECT: u8              = 3; // 0 = 0x9800-0x9BFF; 1 = 0x9C00-0x9FFF
const OBJ_SIZE: u8                                = 2; // 0 = 8x8; 1 = 8x16
const OBJ_ENABLE: u8                              = 1; // 0 = Off; 1 = On
const BACKGROUND_ENABLE: u8                       = 0; // 0 = Off; 1 = On


// PPU constructs
const TILE_PIXEL_DIM: usize = 8


// Other
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[derive(Clone, Copy)]
enum ColorID {
    Transparent,
    White,
    LightGray,
    DarkGray,
    Black,
}

struct Tile {
    tile: [PixelLine; TILE_PIXEL_DIM],
    addr: u16,
}

impl Tile {
    /// ID is relative to all of VRAM (not per block).
    fn get_id(&self) -> u8
    {
        self.addr / 16 % 256
    }

    ///  ___________
    /// |    x ->   |
    /// | y         |
    /// | |         |
    /// | V         |
    /// |___________|
    fn get_pixel(&self, x: u8, y: u8) -> ColorID
    {
        tile[y].get_pixel(x)
    }
}

struct PixelLine {
    frame_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pixel_line: u16,
}

impl PixelLine {
    fn get_pixel(&self, x: u8) -> ColorID
    {
        let lsb = get_lsb(self.pixel_line);
        let msb = get_msb(self.pixel_line);

        let bit0 = (lsb >> (7 - x)) & 0x1;
        let bit1 = (msb >> (7 - x)) & 0x1;

        let val = (bit1 << 1) | bit0;
        match val {
            0 => ColorID::White,
            1 => ColorID::LightGray,
            2 => ColorID::DarkGray,
            3 => ColorID::Black,
        }
    }
}


struct PictureProcessingUnit {
    vram: [u8; VRAM_END - VRAM_BEGIN + 1],
    oam: [u8; OAM_END - OAM_BEGIN + 1],

    // LCD registers
    lcdc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,

    // Rendering buffer
    frame_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    window: Window,
}

impl PictureProcessingUnit {
    pub fn new() -> Self 
    {
        let window = Window::new(
            "Game Boy Emulator 2000",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            WindowOptions::default(),
        ).unwrap();

        Self {
            lcd_control_register: 0,
            frame_buffer: [0xFFFFFFFF; SCREEN_WIDTH * SCREEN_HEIGHT],
            window,
        }
    }

    pub fn write_vram(&mut self)
    {

    }

    fn read_vram()
    {

    }

    fn write_vram()
    {

    }
    
    fn render() 
    {
        // TODO: fill in all the stufffff
        // fill in the buffer 


        self.window.update_with_buffer(
            &self.frame_buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        ).unwrap();
    }
}
