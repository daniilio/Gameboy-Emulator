use minifb::{Window, WindowOptions};

use crate::memory::constants::memory_map::*;
use crate::helpers::bitwise::*;

// Register addresses
const LCDC: u16 = 0xFF40;  // LCD control
const STAT: u16 = 0xFF41;  // STAT: LCD status
const SCY: u16 = 0xFF42;  // Background viewport Y position
const SCX: u16 = 0xFF43;  // Background viewport X position
const LY: u16 = 0xFF44;  // LCD Y coordinate (read-only)  // TODO: LY is read-only
const LYC: u16 = 0xFF45;  // LY compare
const BGP: u16 = 0xFF47;  // Background palette data
const OBP0: u16 = 0xFF48;  // OBJ palette 0
const OBP1: u16 = 0xFF49; // OBJ palette 1
const WY: u16 = 0xFF4A;  // Window Y-coordinate
const WX: u16 = 0xFF4B;  // Window X-coordinate

// LCDC register
const LCD_ENABLE: u8                              = 7;
const WINDOW_TILE_MAP_SELECT: u8                       = 6;
const WINDOW_ENABLE: u8                           = 5;
const BACKGROUND_AND_WINDOW_TILE_DATA_SELECT: u8  = 4;
const BACKGROUND_TILE_MAP_SELECT: u8              = 3;
const OBJ_SIZE: u8                                = 2;
const OBJ_ENABLE: u8                              = 1;
const BACKGROUND_AND_WINDOW_ENABLE: u8                = 0; 

// VRAM memory map
const TILE_DATA_0_BEGIN: u16 = 0x8000;
const TILE_DATA_0_END:   u16 = 0x87FF;
const TILE_DATA_1_BEGIN: u16 = 0x8800;
const TILE_DATA_1_END:   u16 = 0x8FFF;
const TILE_DATA_2_BEGIN: u16 = 0x9000;
const TILE_DATA_2_END:   u16 = 0x97FF;
const TILE_MAP_0_BEGIN: u16 = 0x9800;
const TILE_MAP_0_END:   u16 = 0x9BFF;
const TILE_MAP_1_BEGIN: u16 = 0x9C00;
const TILE_MAP_1_END:   u16 = 0x9FFF;

// Tile data block and tile map block sizes
const TILE_DATA_BLOCK_SIZE:  usize = (TILE_DATA_0_END - TILE_DATA_0_BEGIN + 1) as usize;
const TILE_MAP_BLOCK_SIZE: usize = (TILE_MAP_0_END - TILE_MAP_0_BEGIN + 1) as usize;

// Gameboy pixel color values
const GB_WHITE: u8 = 0;
const GB_LIGHT_GRAY: u8 = 1;
const GB_DARK_GRAY: u8 = 2;
const GB_BLACK: u8 = 3;

// Frame buffer colors
const FB_WHITE: u32 = 0xE0F8D0;
const FB_LIGHT_GRAY: u32 = 0x88C070;
const FB_DARK_GRAY: u32 = 0x346856;
const FB_BLACK: u32 = 0x081820;

// Screen dimensions
const BUFFER_DIM: usize = 256;  // 256x256 pixels frame buffer
const SCREEN_DIM_X: usize = 160;  // 160x144 pixel window into frame buffer
const SCREEN_DIM_Y: usize = 144;

// Tiles
const TILE_DIM: u8 = 8;  // 8x8 pixel tile
const TILE_SIZE: u8 = 16;  // 16 byte tile
const TILES_PER_SCREEN_AXIS: usize = FRAME_BUFFER_DIM / TILE_DIM;  // 32x32 tiles in frame buffer

// STAT register
const LYC_SELECT: u8 = 6;  // If set, selects the LYC == LY condition for STAT interrupt
const MODE_2_SELECT: u8 = 5;  // If set, selects the Mode 2 condition for STAT interrupt
const MODE_1_SELECT: u8 = 4;  // If set, selects the Mode 1 condition for STAT interrupt
const MODE_0_SELECT: u8 = 3;  // If set, selects the Mode 0 condition for STAT interrupt
const LYC_EQUAL_LY: u8 = 2; // Read-only
const PPU_MODE: u8 = 1;  // Read-only; Includes bit 1 and 0

// Object count
const OBJECT_COUNT: u8 = 40;  // 40 8x8 or 8x16 objects can be specified

// Object attribute flags
const PRIORITY: u8 = 7;
const Y_FLIP: u8 = 6;
const X_FLIP: u8 = 5;
const PALETTE: u8 = 4;

struct Object {
    x: i16,
    y: i16,
    tile_id: u8,
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    palette: u8,
}

pub struct PictureProcessingUnit {
    // Remaining clock cycles in the current PPU mode.
    remaining_clock_cycles: u16,
    // Complete the work for a PPU mode all at once (irregardless of timing). 
    // Then wait through remaining clock cycles.
    wait: bool,
    // Object IDs found in mode 0.
    objects_in_scanline: Vec<Object>,
    interrupt_controller: InterruptController,
    // Previous STAT interrupt signal. STAT interrupt is edge triggerred.
    stat_signal: bool,

    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],

    // LCD/PPU control registers.
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,

    // Palette registers.
    bgp: u8,
    obp0: u8,
    obp1: u8,

    // Rendering and display.
    screen_buffer: [[u32; SCREEN_DIM_X]; SCREEN_DIM_Y],
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

    pub fn read_byte(&self, addr: u16) -> u8 
    {
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                // Restrict access to VRAM when in mode 3
                let ppu_mode = self.stat & 0x03;
                if ppu_mode == 3 {
                    0xFF
                } else {
                    self.vram[(addr - VRAM_BEGIN) as usize]
                }
            }
            OAM_BEGIN ..= OAM_END => {
                // Restrict access to OAM when in mode 2 or 3
                let ppu_mode = self.stat & 0x03;
                if ppu_mode == 2 || ppu_mode == 3 {
                    0xFF
                } else {
                    self.oam[(addr - OAM_BEGIN) as usize]
                }
            }
            LCDC => {
                self.lcdc
            }
            STAT => {
                self.stat
            }
            SCY => {
                self.scy
            }
            SCX => {
                self.scx
            }
            LY => {
                self.ly
            }
            LYC => {
                self.lyc
            }
            BGP => {
                self.bgp
            }
            OBP0 => {
                self.obp0
            }
            OBP1 => {
                self.obp1
            }
            WY => {
                self.wy
            }
            WX => {
                self.wx
            }
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8)
    {
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                // Restrict VRAM access when in PPU mode 3
                let ppu_mode = self.stat & 0x03;
                if ppu_mode != 3 {
                    self.vram[(addr - VRAM_BEGIN) as usize] = value;
                }
            }
            OAM_BEGIN ..= OAM_END => {
                // Restrict OAM access when in PPU mode 2 or 3
                let ppu_mode = self.stat & 0x03;
                if ppu_mode != 2 && ppu_mode != 3 {
                    if is_bit_set(self.lcdc, OBJ_SIZE) {
                        self.oram[(addr - ORAM_BEGIN) as usize] = value & 0xFE;
                    } else {
                        self.oram[(addr - ORAM_BEGIN) as usize] = value;
                    }
                }
            }
            LCDC => {
                self.lcdc = value;
            }
            STAT => {
                // First 3 bits are read-only
                self.stat = (value & 0xF8) | (self.stat & 0x07);
            }
            SCY => {
                self.scy = value;
            }
            SCX => {
                self.scx = value;
            }
            LYC => {
                self.lyc = value;
            }
            BGP => {
                self.bgp = value;
            }
            OBP0 => {
                self.obp0 = value;
            }
            OBP1 => {
                self.obp1 = value;
            }
            WY => {
                self.wy = value;
            }
            WX => {
                self.wx = value;
            }
            _ => {}
        }
    }

    fn ppu_mode(&self) -> u8
    {
        self.stat & 0x03
    }

    fn lcd_enable(&self) -> bool {
        is_bit_set(self.lcdc, LCD_ENABLE)
    }
    
    fn window_tile_map_select(&self) -> bool {
        is_bit_set(self.lcdc, WINDOW_TILE_MAP_SELECT)
    }
    
    fn window_enable(&self) -> bool {
        is_bit_set(self.lcdc, WINDOW_ENABLE)
    }
    
    fn background_and_window_tile_data_select(&self) -> bool {
        is_bit_set(self.lcdc, BACKGROUND_AND_WINDOW_TILE_DATA_SELECT)
    }
    
    fn background_tile_map_select(&self) -> bool {
        is_bit_set(self.lcdc, BACKGROUND_TILE_MAP_SELECT)
    }
    
    fn obj_size(&self) -> bool {
        is_bit_set(self.lcdc, OBJ_SIZE)
    }
    
    fn obj_enable(&self) -> bool {
        is_bit_set(self.lcdc, OBJ_ENABLE)
    }
    
    fn background_and_window_enable(&self) -> bool {
        is_bit_set(self.lcdc, BACKGROUND_AND_WINDOW_ENABLE)
    }

    /// Given a palette and colour ID, returns a 32-bit colour value.
    fn colour(colour_id: u8, palette: u8) -> u32
    {
        let shift = colour_id * 2;
        let gb_colour = palette >> shift & 0x03;

        match gb_colour {
            GB_WHITE => FB_WHITE,
            GB_LIGHT_GRAY => FB_LIGHT_GRAY,
            GB_DARK_GRAY => FB_DARK_GRAY,
            GB_BLACK => FB_BLACK,
            _ => panic!(Invalid Game Boy Colour!),
        }
    }

    /// Get the address of the tile with given tile id.
    fn tile_vram_addr(&self, tile_id: u8) -> u16
    {
        if self.background_and_window_tile_data_select() {
            // Tile data block 0 and block 1 selected
            TILE_DATA_0_BEGIN + (tile_id as u16) - VRAM_BEGIN
        } else {
            // Tile data block 1 and block 2 selected
            match tile_id {
                0 ..= 127 => {
                    TILE_DATA_2_BEGIN + (tile_id as u16) - VRAM_BEGIN
                }
                128 ..= 255 => {
                    TILE_DATA_1_BEGIN + ((tile_id - 128) as u16) - VRAM_BEGIN
                }
            }
        }
    }

    /// Returns the colour ID of a pixel in the tile map.
    fn pixel_id(&self, tile_map_addr: u16, x: u8, y: u8) -> u8
    {
        // Index of the tile in the tile map.
        let tile_idx = 
            (y / TILE_DIM) * ((BUFFER_DIM / (TILE_DIM as usize)) as u8) + 
            (x / TILE_DIM);
        // Address of the tile in the tile map block.
        let addr = tile_map_addr + tile_idx as u16;
        
        // ID of the tile in tile data block.
        let tile_id = self.vram[addr as usize];
        // Address of the tile in the tile data block.
        let tile_addr = self.tile_vram_addr(tile_id);

        // x and y positions within the tile.
        let tile_x = x % TILE_DIM;
        let tile_y = y % TILE_DIM;

        // Each row of the tile is 2 bytes.
        let pixel_addr = (tile_addr + (tile_y as u16) * 2) as usize;

        let ls_bit = (self.vram[pixel_addr] >> (7 - tile_x)) & 1;
        let ms_bit = 
            (self.vram[pixel_addr.wrapping_add(1)] >> (7 - tile_x)) & 1;
        let colour_id = (ms_bit << 1) | ls_bit;

        colour_id
    }

    /// Returns the color ID of specified pixel in the background.
    /// Coordinates are relative to the tile map.
    fn background_pixel_id(&self, x: u8, y: u8) -> u8
    {
        // Base address of the tile map
        let tile_map_addr = if self.background_tile_map_select() {
            TILE_MAP_1_BEGIN
        } else {
            TILE_MAP_0_BEGIN
        };
        self.pixel_id(tile_map_addr, x, y)
    }

    /// Returns the colour ID of specified pixel in the window.
    /// Coordinates are relative to the tile map.
    fn window_pixel_id(&self, x: u8, y: u8) -> u8
    {
        let tile_map_addr = if self.window_tile_map_select() {
            // Render window from tile map 1
            TILE_MAP_1_BEGIN
        } else {
            // Render window from tile map 0
            TILE_MAP_0_BEGIN
        };
        self.pixel_id(tile_map_addr, x, y)
    }

    /// Returns the colour ID of specified pixel in the object.
    /// Coordinates are relative to the object.
    fn object_pixel_id(&self, obj: &Object, x: u8, y: u8) -> u8
    {
        // Address of the tile in the tile data block.
        let tile_addr = 
            TILE_DATA_0_BEGIN + obj.tile_id as u16 - VRAM_BEGIN;

        // x and y positions within the tile.
        let mut tile_x = x % TILE_DIM;
        let mut tile_y = y % TILE_DIM;
        
        if obj.x_flip {
            tile_x = 7 - tile_x;
        }
        if obj.y_flip {
            tile_y = 7 - tile_y;
        }

        // Each row of the tile is 2 bytes.
        let pixel_addr = tile_addr + (tile_y as u16) * 2;

        let ls_bit = (self.vram[pixel_addr as usize] >> (7 - tile_x)) & 1;
        let ms_bit = 
            (self.vram[(pixel_addr as usize).wrapping_add(1)] >> (7 - tile_x)) & 1;
        let colour_id = (ms_bit << 1) | ls_bit;

        colour_id
    }

    /// PPU mode 2. Save objects present in the current scanline.
    fn mode2(&mut self)
    {
        // Save objects present in this scanline
        self.objects_in_scanline.clear();
        for id in 0..OBJECT_COUNT {
            // Gameboy only draws up to 10 objects
            if self.objects_in_scanline.len() == 10 {
                break;
            }
            
            // Get object attributes
            let addr = (OAM_BEGIN as usize) + ((id * 4) as usize);
            let y_screen = (self.oam[addr] as i16) - 16;
            let x_screen = (self.oam[addr.wrapping_add(1)] as i16) - 8;
            let tile_id = if self.obj_size() {
                // 8x16 must be aligned to two tiles
                self.oam[addr.wrapping_add(2)] & 0xFE
            } else {
                // 8x8
                self.oam[addr.wrapping_add(2)]
            };
            let priority = is_bit_set(
                self.oam[addr.wrapping_add(3)], 
                PRIORITY
            );
            let y_flip = is_bit_set(
                self.oam[addr.wrapping_add(3)], 
                Y_FLIP
            );
            let x_flip = is_bit_set(
                self.oam[addr.wrapping_add(3)], 
                X_FLIP
            );
            let palette = 
                if is_bit_set(
                    self.oam[addr.wrapping_add(3)], 
                    PALETTE
                ) {
                self.obp1
            } else {
                self.obp0
            };

            let obj = Object {
                x: x_screen,
                y: y_screen,
                tile_id,
                priority,
                y_flip,
                x_flip,
                palette,
            };

            // Check if sprite is 8x8 or 8x16
            let offset = if self.obj_size() {
                16
            } else {
                8
            };

            // Check if the object intersects the scanline
            if y_screen <= self.ly && self.ly < y_screen + offset {
                self.present_objects.push(obj);
            }
        }
    }

    /// PPU mode 3. Render and display the scanline.
    fn mode3(&mut self)
    {
        for i in 0..SCREEN_DIM_X {
            let mut colour: u32;

            // x and y coordinate of pixel in tile map.
            let x = 
                ((self.scx as u16 + i as u16) % (BUFFER_DIM as u16)) as u8;
            let y = 
                ((self.scy as u16 + self.ly as u16) % (BUFFER_DIM as u16)) as u8;

            let mut colour_id = 0;
            if self.background_and_window_enable() {
                // WX register is x position plus 7.
                let window_x = i as i16 - (self.wx as i16 - 7);
                let window_y = self.ly as i16 - self.wy as i16;
                
                // Check that screen coordinates are in the window area.
                if self.window_enable() && 
                    window_x >= 0 &&
                    window_y >= 0 {

                    colour_id = 
                        self.window_pixel_id(window_x as u8, window_y as u8);
                } else {
                    colour_id = self.background_pixel_id(x, y);
                }
            }
            colour = Self::colour(colour_id, self.bgp);

            // Check if any sprites overwrite this pixel.
            // Largest possible x-coordinate origin for an object (255 - 8).
            let mut prev_obj_x: i16 = 247;
            // Largest possible tile ID for an object.
            let mut prev_obj_id: u8 = 255;
            for obj in &self.objects_in_scanline {
                if obj.priority && colour_id != 0 {
                    // Object is behind nonzero background/window colour IDs.
                    continue;
                }

                // Check whether the sprite overlaps this pixel.
                let offset = if self.obj_size() {
                    // 8x16 sprites.
                    16
                } else {
                    // 8x8 sprites.
                    8
                };
                if !(obj.x <= (x as i16) && (x as i16) < obj.x + 8 && 
                    obj.y <= (self.ly as i16) && (self.ly as i16) < obj.y + offset) {
                    continue;
                }

                let obj_colour_id = self.object_pixel_id(
                        obj, 
                        (x as i16 - obj.x) as u8, 
                        (y as i16 - obj.y) as u8
                );

                if obj_colour_id == 0 {
                    continue;
                }

                // Leftmost object has priority. If there's a tie, break it by 
                // prioritizing lowest tile ID.
                if obj.x < prev_obj_x {
                    colour_id = obj_colour_id;
                    colour = Self::colour(colour_id, obj.palette);

                    prev_obj_x = obj.x;
                    prev_obj_id = obj.tile_id;
                } else if obj.x == prev_obj_x && 
                    obj.tile_id <= prev_obj_id {

                    colour_id = obj_colour_id;
                    colour = Self::colour(colour_id, obj.palette);

                    prev_obj_id = obj.tile_id;
                }
            }

            self.screen_buffer[self.ly as usize][i] = colour;
        }
        
        // Write scanline to screen buffer and output to display.
        self.window.update_with_buffer(
            self.screen_buffer.as_flattened(),
            SCREEN_DIM_X,
            SCREEN_DIM_Y,
        ).unwrap();
    }

    fn cycles_per_mode(mode: u8) -> u16
    {
        // TODO: These are just baselines, need to include penalty calculation
        match mode {
            0 => 87,
            1 => 4560,
            2 => 80,
            3 => 172,
            _ => panic!("Invalid PPU mode!"),
        }
    }

    fn wait_mode(&self, cycles: u8)
    {
        if self.remaining_clock_cycles <= cycles {
            self.ppu_mode = (self.ppu_mode + 1) % 4;

            let remaining_cycles =  cycles - self.remaining_clock_cycles;

            self.remaining_clock_cycles = Self::cycles_per_mode(self.ppu_mode);

            // Continue rendering for the number of remaining cycles
            self.render(remaining_cycles);
        } else {
            self.remaining_clock_cycles -= cycles;
        }
    }

    fn stat_interrupt(&mut self)
    {
        let mode = self.ppu_mode();
        let stat_signal = 
            (is_bit_set(self.stat, MODE_0_SELECT) && mode == 0) ||
            (is_bit_set(self.stat, MODE_1_SELECT) && mode == 1) ||
            (is_bit_set(self.stat, MODE_2_SELECT) && mode == 2) ||
            (is_bit_set(self.stat, LYC_SELECT) && self.lyc == self.ly);
        // Request STAT interrupt if the stat signal goes low to high.
        if !self.stat_signal && stat_signal {
            self.interrupt_controller.request_interrupt();
        }
        self.stat_signal = stat_signal;
    }
    
    pub fn render(&mut self, cycles: u8) 
    {  
        if cycles == 0 {
            return;
        }
        if self.wait {
            self.wait_mode(cycles);
        }

        self.stat_interrupt();

        match self.ppu_mode() {
            0 => {

            }
            1 => {
                
            }
            2 => {
                self.mode2();
                self.wait_mode(cycles);
            }
            3 => {
                self.mode3();
                self.wait_mode(cycles);
            }
        }
    }
}
