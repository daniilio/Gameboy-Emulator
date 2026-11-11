use minifb::{Window, WindowOptions};

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
const FRAME_BUFFER_DIM: usize = 256;  // 256x256 pixels frame buffer
const SCREEN_DIM_X: usize = 160;  // 160x144 pixel window into frame buffer
const SCREEN_DIM_Y: usize = 144;

// Tiles
const TILE_DIM: usize = 8;  // 8x8 pixel tile
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
    // Top-left position
    x: u8,
    y: u8,

    // False=8x8 or True=8x16 mode
    sprite_mode: bool,

    tile_idx: u8,

    // Flags
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    palette: bool,
}

pub struct PictureProcessingUnit {
    // Remaining clock cycles in the current PPU mode
    remaining_clock_cycles: u16,

    // Object IDs found in mode 0
    objects_present: Vec<Object>,

    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],

    // LCD/PPU control registers
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,

    // Palette registers
    bgp: u8,
    obp0: u8,
    obp1: u8,

    // Rendering and display
    frame_buffer: [u32; SCREEN_BUFFER_DIM * SCREEN_BUFFER_DIM],
    window_buffer: [u32; SCREEN_DIM_X * SCREEN_DIM_Y],
    window: Window,

    interrupt_controller: InterruptController,
    // The STAT interrupt needs to be edge trigerred
    stat_signal: bool,
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

    pub read_byte(&self, addr: u16) -> u8 
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
            _ => 0xFF
        }
    }

    pub write_byte(&mut self, addr: u16, value: u8)
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
            _ => {}
        }
    }

    /// Converts pixel position to the frame buffer index.
    ///
    /// # Arguments
    /// * `tile_idx` - Index of the tile in tile map.
    /// * `pixel_line` - Line (y-coordinate) within the tile.
    /// * `pixel_idx` - Index (x-coordinate) within the tile.
    fn to_fb_index(tile_idx: u8, pixel_line: u8, pixel_idx: u8) -> usize
    {
        let tile_idx = tile_idx as usize;
        let pixel_line = pixel_line as usize;
        let pixel_idx = pixel_idx as usize;

        let x = pixel_idx + (tile_idx % TILES_PER_SCREEN_AXIS) * TILE_DIM;
        let y = pixel_line + (tile_idx / TILES_PER_SCREEN_AXIS) * TILE_DIM;

        x + y * SCREEN_BUFFER_DIM
    }

    /// Converts a Gameboy color value to a frame buffer
    /// color value used by the emulator.
    fn to_fb_color(gb_color: u8) -> u32
    {
        match gb_color {
            GB_WHITE => FB_WHITE,
            GB_LIGHT_GRAY => FB_LIGHT_GRAY,
            GB_DARK_GRAY => FB_DARK_GRAY,
            GB_BLACK => FB_BLACK,
        }
    }

    /// Maps the palette register to an array.
    /// 
    /// The array is indexed by ID, with color value elements.
    fn palette_map(palette: u8) -> [u8; 4]
    {
        let mut map = [u8; 4];
        let mask = 0x03;
        map[0] = palette & mask;
        map[1] = (palette >> 2) & mask;
        map[2] = (palette >> 4) & mask;
        map[3] = (palette >> 6) & mask;

        map
    }

    /// Get the address of the tile with given tile id.
    ///
    /// # Arguments
    /// * `tile_id` - The ID of the tile.
    fn tile_addr(&self, tile_id: u8) -> u16
    {
        if is_bit_set(self.lcdc, BACKGROUND_AND_WINDOW_TILE_DATA_SELECT) {
            // Tile data block 0 and block 1 selected
            TILE_DATA_0_BEGIN + (tile_id as u16)
        } else {
            // Tile data block 1 and block 2 selected
            match tile_id {
                0 ..= 127 => {
                    TILE_DATA_2_BEGIN + (tile_id as u16)
                }
                128 ..= 255 => {
                    TILE_DATA_1_BEGIN + ((tile_id - 128) as u16)
                }
            }
        }
    }

    /// Render a tile.
    ///
    /// # Arguments
    /// * `tile_id` - ID of tile in tile data block.
    /// * `tile_idx` - Index of tile in tile map.
    /// * `palette` - Palette register value.
    fn render_tile(&mut self, tile_id: u8, tile_idx: usize, palette: u8)
    {
        let mut cur_addr = self.tile_addr(tile_id);
        let palette_map = Self::palette_map(palette);
        
        // Each horizontal line of pixels is 2 bytes
        // First byte specifies least significant bit of color ID
        // Second byte specifies most significant bit of color ID
        // Bit 7 is the leftmost pixel, and bit 0 is the rightmost
        for line in 0..8 {
            // Process horizontal line
            for idx in 0..8 {
                // Process pixels left to right
                let ls_bit = is_bit_set(self.vram[cur_addr as usize], (7 - idx)) as u8;
                let ms_bit = is_bit_set(self.vram[cur_addr.wrapping_add(1) as usize], (7 - idx)) as u8;
                let color_id = (ms_bit << 1) | ls_bit;

                // Map color ID to color specified by palette
                let gb_color = palette_map[color_id];
                let fb_color = Self::to_fb_color(gb_color);

                // Pixel index in the screen buffer
                let fb_pixel_idx = Self::to_fb_index(tile_idx, line, idx);

                // Draw pixel to the screen
                self.frame_buffer[fb_pixel_idx] = fb_color;
            }
            cur_addr = cur_addr.wrapping_add(2);
        }
    }

    fn render(&mut self, base_addr: u16)
    {
        for idx in 0..TILE_MAP_BLOCK_SIZE {
            let tile_id = self.vram[(base_addr as usize) + idx];
            self.render_tile(tile_id, idx);
        }
    }

    fn render_background(&mut self)
    {
        let base_addr = if is_bit_set(self.lcdc, BACKGROUND_TILE_MAP_SELECT) {
            // Render background from tile map 1
            TILE_MAP_1_BEGIN
        } else {
            // Render background from tile map 0
            TILE_MAP_0_BEGIN
        };
        self.render(base_addr);
    }

    fn render_window(&mut self)
    {
        let base_addr = if is_bit_set(self.lcdc, WINDOW_TILE_MAP_SELECT) {
            // Render window from tile map 1
            TILE_MAP_1_BEGIN
        } else {
            // Render window from tile map 0
            TILE_MAP_0_BEGIN
        };
        self.render(base_addr);
    }

    fn render_object(&mut self)
    {

    }

    fn copy_to_window(&mut self)
    {

    }
    
    pub fn render(&mut self) 
    {  
        // Request STAT interrupt if the stat signal goes low to high
        let mode = self.stat & 0x03;
        let new_stat_signal = (is_bit_set(self.stat, MODE_0_SELECT) && mode == 0) ||
            (is_bit_set(self.stat, MODE_1_SELECT) && mode == 1) ||
            (is_bit_set(self.stat, MODE_2_SELECT) && mode == 2) ||
            (is_bit_set(self.stat, LYC_SELECT) && self.lyc == self.ly);
        if !self.stat_signal && new_stat_signal {
            self.interrupt_controller.request_interrupt();
        }
        self.stat_signal = new_stat_signal;

        match mode {
            0 => {

            }
            1 => {
                
            }
            2 => {
                // Save objects present in this scanline
                self.present_objects.clear();
                for id in 0..OBJECT_COUNT {
                    // Gameboy only draws up to 10 objects
                    if self.present_objects.len() >= 10 {
                        break;
                    }
                    
                    // Get object attributes
                    let addr = (OAM_BEGIN as usize) + (id as usize);
                    let y_screen = (self.oam[addr] as i16) - 16;
                    let x_screen = (self.oam[addr.wrapping_add(1)] as i16) - 8;
                    let sprite_mode = is_bit_set(self.lcdc, OBJ_SIZE);
                    let tile_idx = if sprite_mode {
                        self.oam[addr.wrapping_add(2)] & 0xFE
                    } else {
                        self.oam[addr.wrapping_add(2)]
                    };
                    let priority = is_bit_set(self.oam[addr.wrapping_add(3)], PRIORITY);
                    let y_flip = is_bit_set(self.oam[addr.wrapping_add(3)], Y_FLIP);
                    let x_flip = is_bit_set(self.oam[addr.wrapping_add(3)], X_FLIP);
                    let palette = is_bit_set(self.oam[addr.wrapping_add(3)], PALETTE);

                    let obj = Object {
                        x: x_screen,
                        y: y_screen,
                        sprite_mode,
                        tile_idx,
                        priority,
                        y_flip,
                        x_flip,
                        palette,
                    };

                    // Check if sprite is 8x8 or 8x16
                    let offset = if obj.sprite_mode {
                        16
                    } else {
                        8
                    };

                    // Check if the object intersects the scanline
                    if y_screen <= self.ly && self.ly <= y_screen + offset {
                        self.present_objects.push(obj);
                    }
                }

                if self.remaining_clock_cycles == 0 {
                    // Set to mode 3
                    self.stat = (self.stat & 0xFC) | 3;
                    self.remaining_clock_cycles = ;
                }
            }
            3 => {
                // Render background and window
                let mut scan_line = [0u32; SCREEN_DIM_X];
                for x_screen in 0..SCREEN_DIM_X {
                    let x = self.scx + x_screen;
                    let y = self.scy + self.ly;

                    render_background_pixel(&mut scan_line, x, y);
                    render_window_pixel(&mut scan_line, x, y);
                }

                // Render sprites
                for obj in &self.present_objects {
                    obj.priority[]

                    for i in 0..8 {
                        let x_pixel = (x_screen + i);
                        let color_id = 
                        if 0 <= x_pixel && x_pixel < SCREEN_DIM_X {
                            scan_line[]
                        }
                    }
                }
            }
        }
        
        // Copy 
        window_buffer[]

        self.window.update_with_buffer(
            &self.window_buffer,
            SCREEN_DIM_X,
            SCREEN_DIM_Y,
        ).unwrap();
    }
}
