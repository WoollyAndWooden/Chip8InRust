use sdl2::{keyboard::Keycode, sys::{SDL_Event, SDL_WaitEvent}};

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

// Masks to get specufic bits of the opcode
fn _get_nnn(opcode: u16) -> u16
{
    return opcode & 0x0fff;
}
fn _get_x(opcode: u16) -> u16
{
    return (opcode >> 8) & 0x0f;
}
fn _get_y(opcode: u16) -> u16
{
    return (opcode >> 4) & 0x00f;
}
fn _get_kk(opcode: u16) -> u16
{
    return opcode & 0x00ff;
}
fn _get_last(opcode: u16) -> u16
{
    return opcode & 0x000f;
}


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

    pub fn DrawSprite(&mut self, x: usize, y: usize, _sprite: &[u8], num: usize) -> bool
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
        let val = self.Stack[self.SP as usize];
        self.SP -= 1;
        return val;
    }

    // Program takes from ROM 4 bytes of opcode, and simulates CHIP-8's instruction set. _0x8 executes all operations from opcodes matching 0x8***, _0xF executes 0xF*** commands. Rest are in main exec function.
    fn _0x8(&mut self, opcode: u16)
    {
        let x = _get_x(opcode) as usize;
        let y = _get_y(opcode) as usize;
        let last = _get_last(opcode) as usize;

        if(last == 0x0)
        {
            self.V[x as usize] = self.V[y];
        }
        else if(last == 0x1)
        {
            self.V[x] = self.V[x] | self.V[y];
        }
        else if(last == 0x2)
        {
            self.V[x] = self.V[x] & self.V[y];
        }
        else if(last == 0x3)
        {
            self.V[x] = self.V[x] ^ self.V[y];
        }
        else if(last == 0x4)
        {
            let check = self.V[x] + self.V[y] > 0xff;
            self.V[x] += self.V[y];
            self.V[0xf] = check as u8;
        }
        else if(last == 0x5)
        {
            let check = self.V[x] >= self.V[y];
            self.V[x] -= self.V[y];
            self.V[0xf] = check as u8;
        }
        else if(last == 0x6)
        {
            let check = _get_last(self.V[x].into()) & 0b0001;
            self.V[x] /= 2;
            self.V[0xf] = check as u8;
        }
        else if(last == 0x7)
        {

            self.V[x] = self.V[y] - self.V[x];
            self.V[0xf] = (self.V[y] >= self.V[x]) as u8;
        }
        else if(last == 0xE)
        {
            let check = self.V[x] >> 7;
            self.V[x] *= 2;
            self.V[0xf] = check as u8;
        }
        
    }

    fn _0xF(&mut self, opcode: u16)
    {
        let x = _get_x(opcode) as usize;
        let kk = _get_kk(opcode);
        if kk == 0x07
        {
            self.V[x] = self.DT;
        }
        else if kk == 0x0A
        {
            // KEYBOARD NOT IMPLEMENTED PROPERLY YET
        }
        else if kk == 0x15
        {
            self.DT = self.V[x];
        }
        else if kk == 0x18
        {
            self.ST = self.V[x];
        }
        else if kk == 0x1E
        {
            self.I += self.V[x] as u16;
        }
        else if kk == 0x29
        {
            self.I += self.V[x] as u16 * DEFAULT_SPRITE_HEIGHT as u16;
        }
        // LD B, Vx: Store BCD representation of Vx in memory locations I (hundreds), I+1 (tens) and I+2 (units)
        else if kk == 0x33
        {
            self.MemorySet(self.I as usize, self.V[x] / 100);
            self.MemorySet(self.I as usize +1, self.V[x] / 10 % 10);
            self.MemorySet(self.I as usize +2, self.V[x] % 10);
        }
        else if kk == 0x55
        {
            for i in 0..x
            {
                self.MemorySet(self.I as usize + 1, self.V[i]);
            }
        }
        else if kk == 0x65
        {
            for i in 0..x
            {
                 self.V[i] = self.MemoryGet(self.I as usize);
            }
        }

    }

    fn _extend_execute(&mut self, opcode: u16)
    {
        let opcodeCheck = opcode & 0xF000;
        if opcodeCheck == 0x1000
        {
            self.PC = _get_nnn(opcode);
        }
        else if opcodeCheck == 0x2000
        {
            self.StackPush(self.PC);
            self.PC = _get_nnn(opcode);
        }
        else if opcodeCheck == 0x3000
        {
            if self.V[_get_x(opcode) as usize] as u16 == _get_kk(opcode)
            {
                self.PC += 2;
            }
        }
        else if opcodeCheck == 0x4000
        {
            if self.V[_get_x(opcode) as usize] as u16 != _get_kk(opcode)
            {
                self.PC += 2;
            }
        }
        else if opcodeCheck == 0x5000
        {
            if self.V[_get_x(opcode) as usize] == self.V[_get_y(opcode) as usize]
            {
                self.PC += 2;
            }
        }
        else if opcodeCheck == 0x6000
        {
            self.V[_get_x(opcode) as usize] = _get_kk(opcode) as u8;
        }
        else if opcodeCheck == 0x7000
        {
            self.V[_get_x(opcode) as usize] += _get_kk(opcode) as u8;
        }
        else if opcodeCheck == 0x8000
        {
            self._0x8(opcode);
        }
        else if opcodeCheck == 0x9000
        {
            if self.V[_get_x(opcode) as usize] != self.V[_get_y(opcode) as usize]
            {
                self.PC += 2;
            }
        }
        else if opcodeCheck == 0xA000
        {
            self.I = _get_nnn(opcode);
        }
        else if opcodeCheck == 0xB000
        {
            self.PC = _get_nnn(opcode) + self.V[0] as u16;
        }
        else if opcodeCheck == 0xC000
        {
            self.V[_get_x(opcode) as usize] = rand::random::<u8>();
        }
        else if opcodeCheck == 0xD000
        {
            let nibble = _get_last(opcode);
            let x = _get_x(opcode) as usize;
            let y = _get_y(opcode) as usize;
            self.DrawSprite(self.V[x].into(), self.V[y].into(), &[self.Memory[self.I as usize]], nibble as usize);
        }
        else if opcodeCheck == 0xE000
        {
            let kk = _get_kk(opcode);
            if kk == 0x9E
            {
                if self.KeyboardIsDown(self.V[_get_x(opcode) as usize].into())
                {
                    self.PC += 2;
                }
            }
            if kk == 0xA1
            {
                if !self.KeyboardIsDown(self.V[_get_x(opcode) as usize].into())
                {
                    self.PC += 2;
                }
            }
        }
        else if opcodeCheck == 0xF000
        {
            self._0xF(opcode);
        }
    }

    pub fn execute(&mut self, opcode: u16)
    {
        if opcode == 0x00E0
        {
            self.DisplayClear();
        }
        if opcode == 0x00EE
        {
            self.PC = self.StackPop();
        }
        else 
        {
            self._extend_execute(opcode);   
        }
    }

}