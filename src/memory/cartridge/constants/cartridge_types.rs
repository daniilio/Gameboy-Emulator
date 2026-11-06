// Cartridge type codes
const ROM_ONLY: u8 = 0x00;

const MBC1: u8 = 0x01;
const MBC1_RAM: u8 = 0x02;
const MBC1_RAM_BATTERY: u8 = 0x03;

const MBC2: u8 = 0x05;
const MBC2_BATTERY: u8 = 0x06;

const ROM_RAM: u8 = 0x08;
const ROM_RAM_BATTERY: u8 = 0x09;

const MMM01: u8 = 0x0B;
const MMM01_RAM: u8 = 0x0C;
const MMM01_RAM_BATTERY: u8 = 0x0D;

const MBC3_TIMER_BATTERY: u8 = 0x0F;
const MBC3_TIMER_RAM_BATTERY: u8 = 0x10;
const MBC3: u8 = 0x11;
const MBC3_RAM: u8 = 0x12;
const MBC3_RAM_BATTERY: u8 = 0x13;

const MBC5: u8 = 0x19;
const MBC5_RAM: u8 = 0x1A;
const MBC5_RAM_BATTERY: u8 = 0x1B;
const MBC5_RUMBLE: u8 = 0x1C;
const MBC5_RUMBLE_RAM: u8 = 0x1D;
const MBC5_RUMBLE_RAM_BATTERY: u8 = 0x1E;

const MBC6: u8 = 0x20;
const MBC7_SENSOR_RUMBLE_RAM_BATTERY: u8 = 0x22;

const POCKET_CAMERA: u8 = 0xFC;
const BANDAI_TAMA5: u8 = 0xFD;
const HUC3: u8 = 0xFE;
const HUC1_RAM_BATTERY: u8 = 0xFF;
