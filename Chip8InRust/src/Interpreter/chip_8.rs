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
    Stack: [u16; STACK_DEPTH],

    // Registers
    V: [u8; NO_OF_REGISTERS],
    I: u16,
    DT: u8,
    ST: u8,
    PC: u16,
    SP: u8



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
            Stack: [0; STACK_DEPTH],
            V: [0; NO_OF_REGISTERS],
            I: 0,
            DT: 0,
            ST: 0,
            PC: 0,
            SP: 0
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

    // Keyboard
    pub fn KeyboardSwitchState(&mut self, key: usize)
    {
        self.Keyboard[key] = !self.Keyboard[key];
    }

    pub fn KeyboardIsDown(&self, key: usize) -> bool 
    {
        return self.Keyboard[key];
    }

    // Stack
    pub fn StackPush(&mut self, val: u16)
    {
        self.SP += 1;
        self.Stack[self.SP as usize] = val;
    }
    pub fn StackPop(&mut self) -> u16
    {
        self.SP -= 1;
        return self.Stack[self.SP as usize];
    }
}