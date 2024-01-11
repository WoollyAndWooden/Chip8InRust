pub mod Interpreter;
use crate::Interpreter::chip_8::Chip8;

#[test]    
fn MemoryGetTest() {
    let mut sut: Chip8 = Default::default();
    sut.MemorySet(0, 0xAB);
    assert_eq!(0xAB, sut.MemoryGet(0));
}

#[test]    
fn MemoryGetShortTest() {
    let mut sut: Chip8 = Default::default();
    sut.MemorySet(0, 0xAB);
    sut.MemorySet(1, 0x12);
    assert_eq!(0xAB12, sut.MemoryGetShort(0));
}

#[test]
fn PixelSetTest()
{
    let mut sut: Chip8 = Default::default();
    sut.PixelSet(0, 0);
    assert_eq!(true, sut.PixelIsSet(0, 0));
}


#[test]
fn DisplayClearTest()
{
    let mut sut: Chip8 = Default::default();
    sut.PixelSet(0, 0);
    sut.DisplayClear();
    assert_eq!(false, sut.PixelIsSet(0, 0));
}

#[test]
fn KeyboardSwitchStateTest()
{
    let mut sut: Chip8 = Default::default();
    sut.KeyboardSwitchState(0);
    assert_eq!(true, sut.KeyboardIsDown(0));
}

#[test]
fn KeyboardSwitchStateTest2()
{
    let mut sut: Chip8 = Default::default();
    sut.KeyboardSwitchState(0);
    sut.KeyboardSwitchState(0);
    assert_eq!(false, sut.KeyboardIsDown(0));
}
