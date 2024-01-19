use sdl2::keyboard::Keycode;

// Memory
const MEMORY_SIZE: u16 = 4096;
const PROGRAM_LOAD_ADDRESS: u16 = 0x200;

// Display
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY__MULTIPLIER: usize = 10;

// Registers
const NO_OF_REGISTERS: usize = 16;

// Stack
const STACK_DEPTH: usize = 16;

// Keyboard
const TOTAL_KEYS: usize = 16;
const CHARSET_LOAD_ADDRESS: u8 = 0x00;
const DEFAULT_SPRITE_HEIGHT: usize = 5;


pub struct Chip8
{
    // Memory
    Memory: [u8; MEMORY_SIZE as usize],

    // Display
    Pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],

    // Keyboard
    Keyboard: [bool; TOTAL_KEYS], 
    KeyboardMap: [Keycode; TOTAL_KEYS],
    pub DefaultCharset: [u8; 80],

    // Stack
    



}
impl Default for Chip8
{
    fn default() -> Chip8 
    {
        Chip8
        {
            Memory: [0; MEMORY_SIZE as usize],
            Pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            Keyboard: [false; TOTAL_KEYS],
            KeyboardMap: [
                Keycode::Num0, Keycode::Num1, Keycode::Num2, Keycode::Num3,
                Keycode::Num4, Keycode::Num5, Keycode::Num6, Keycode::Num7,
                Keycode::Num8, Keycode::Num9, Keycode::A, Keycode::B, 
                Keycode::C, Keycode::D, Keycode::E, Keycode::F
            ],
            DefaultCharset: [
                0xF0, 0x90, 0x90, 0x90, 0xF0,
                0x20, 0x60, 0x20, 0x20, 0x70,
                0xF0, 0x10, 0xF0, 0x80, 0xF0, 
                0xF0, 0x10, 0xF0, 0x10, 0xF0, 
                0x90, 0x90, 0xF0, 0x10, 0x10, 
                0xF0, 0x80, 0xF0, 0x10, 0xF0,
                0xF0, 0x80, 0xF0, 0x90, 0xF0,
                0xF0, 0x10, 0x20, 0x40, 0x40, 
                0xF0, 0x90, 0xF0, 0x90, 0xF0,
                0xF0, 0x90, 0xF0, 0x10, 0xF0, 
                0xF0, 0x90, 0xF0, 0x90, 0x90, 
                0xE0, 0x90, 0xE0, 0x90, 0xE0, 
                0xF0, 0x80, 0x80, 0x80, 0xF0, 
                0xE0, 0x90, 0x90, 0x90, 0xE0, 
                0xF0, 0x80, 0xF0, 0x80, 0xF0, 
                0xF0, 0x80, 0xF0, 0x80, 0x80
            ],
        }
    }
}
impl Chip8 
{
    // Memory
    pub fn MemorySet(&mut self, index: usize, val: u8)
    {
        self.Memory[index as usize] = val;
    }

    pub fn MemoryGet(&self, index: usize) -> u8
    {
        return self.Memory[index as usize];
    }

    pub fn MemoryGetShort(&self, index: usize) -> u16
    {
        let byte1: u16 = self.MemoryGet(index).into();
        let byte2: u16 = self.MemoryGet(index+1).into();
        let res: u16 = (byte1 << 8) | byte2;
        return res;
    }

    // Display
    pub fn PixelSet(&mut self, x: usize, y: usize)
    {
        self.Pixels[x][y] = true;
    }

    pub fn PixelIsSet(&self, x: usize, y: usize) -> bool
    {
        return self.Pixels[x][y];
    }


    /*
    What i had in C:

    bool chip8_display_draw_sprite(struct chip8_display *display,
    int x, int y,
    const char *sprite, 
    int num)
{
    bool pixel_collision = false;

    for (int ly = 0; ly < num; ly++)
    {
        char c = sprite[ly];
        for(int lx = 0; lx < num; lx++)
        {
            if ((c & (0b10000000 >> lx)) == 0)
                continue;
            
            pixel_collision = display->pixels[(lx + x) % DISPLAY_WIDTH][(ly + y) % DISPLAY_HEIGHT];
            display->pixels[(lx + x) % DISPLAY_WIDTH][(ly + y) % DISPLAY_HEIGHT] ^= true;
        }
    }

    return pixel_collision;
}
     */
    pub fn DrawSprite(&mut self, x: usize, y: usize, sprite: &[u8], num: usize) -> bool
    {
        let mut collision: bool = false;
        for ly in 0..num-1 
        {
            for lx in 0..num-1
            {
                collision = self.Pixels[(lx + x) % DISPLAY_HEIGHT][(ly + y) % DISPLAY_WIDTH];
                self.Pixels[(lx + x) % DISPLAY_HEIGHT][(ly + y) % DISPLAY_HEIGHT] ^= true;
            }
        }
        return collision;
    }

    pub fn DisplayClear(&mut self)
    {
        self.Pixels = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn KeyboardSwitchState(&mut self, key: usize)
    {
        self.Keyboard[key] = !self.Keyboard[key];
    }

    pub fn KeyboardIsDown(&self, key: usize) -> bool 
    {
        return self.Keyboard[key];
    }
}