const LCD_DISPLAY_ENABLED: u8 = 7;
const WINDOW_TILE_MAP_DISPLAY_SELECT: u8 = 6;
const WINDOW_DISPLAY_ENABLE: u8 = 5;
const BACKGROUND_AND_WINDOW_TILE_DATA_SELECT: u8 = 4;
const BACKGROUND_TILE_MAP_DISPLAY_SELECT: u8 = 3;
const OBJ_SIZE: u8 = 2;
const OBJ_DISPLAY_ENABLE: u8 = 1;
const BACKGROUND_DISPLAY: u8 = 0;

struct GPU {
    lcd_control_register: u8,
}

impl GPU {
    fn refresh_graphics() {
        
    }
}
