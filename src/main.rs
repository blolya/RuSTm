#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_reset as _;

use peris::peripherals::clock;
use peris::peripherals::communication::i2c::i2c1::I2c1;
use peris::peripherals::communication::uart::usart1::Usart1;

use peris::peripherals::ports::{
    Port,
    PortMode,
    PortNum,
    OutputConfig,
    MaxSpeed,
};

use peris::core::gpio::gpiob::Gpiob;
use peris::core::gpio::gpioc::Gpioc;


use peris::core::rcc::Rcc;

#[entry]
fn main() -> ! {
    clock::init();

    let rcc = Rcc::new();
    rcc.enable_afio();
    rcc.enable_iopb();
    rcc.enable_i2c1();

    let dbgr = Usart1::new();

    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S50MHz)), &gpioc);

    let gpiob = Gpiob::new();

    let pb6 = Port::new(PortNum::P6, PortMode::Output(OutputConfig::AlternativeFunctionOpenDrain(MaxSpeed::S2MHz)), &gpiob);
    let pb7 = Port::new(PortNum::P7, PortMode::Output(OutputConfig::AlternativeFunctionOpenDrain(MaxSpeed::S2MHz)), &gpiob);

    pc13.set_high();

    let i2c1 = I2c1::new();

    i2c1.cr1.reset_bit(1);

    i2c1.cr2.write_and(!0x0000_003f);
    i2c1.cr2.write_or(36);

    i2c1.ccr.reset_bit(14);
    i2c1.ccr.reset_bit(15);

    i2c1.ccr.write_and(!0x0000_0fff);
    i2c1.ccr.write(180);

    i2c1.trise.write(36);

    i2c1.cr1.set_bit(0);
    
    i2c1.cr1.set_bit(8);
    while i2c1.sr1.get_bit(0) == 0 {};
    
    i2c1.sr1.read();
    i2c1.sr2.read();
    
    i2c1.dr.write(0x29 << 1);
    
    while i2c1.sr1.get_bit(1) == 0 {
        if i2c1.sr1.get_bit(10) == 1 {
            pc13.set_low();
        }
    };
    i2c1.sr1.read();
    i2c1.sr2.read();

    dbgr.send("Heyoloqwprqpwrkpqowrpqwr\r\n");

    
    loop {
        let distance = i2c_read(&i2c1, 0x0062, &pc13);
        dbgr.send_char(distance as char);
        dbgr.send("\r\n");

        for _ in 0..10000 {};
    }
}


fn i2c_write(i2c: &I2c1, address: u8, data: u8) {
    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();

    i2c.dr.write(0x29 << 1 + 1);
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();
		
    i2c.dr.write(address as u32);
    while i2c.sr1.get_bit(7) == 0 {};

    i2c.dr.write(data as u32);
    while i2c.sr1.get_bit(2) == 0 {};
    i2c.cr1.set_bit(9);	
}

fn i2c_read(i2c: &I2c1, address: u8, led: &Port) -> u8 {

    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();


    i2c.dr.write((0x29 << 1));
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();


    i2c.dr.write(address as u32);
    while i2c.sr1.get_bit(7) == 0 {};
    i2c.cr1.set_bit(9);	


    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();


    i2c.dr.write((0x29 << 1) + 1);
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();

    led.set_low();

		
    i2c.cr1.write_and(!0x0000_0400);
    while i2c.sr1.get_bit(6) == 0 {};
    let data: u8 = i2c.dr.read() as u8;			
    i2c.cr1.set_bit(9);	

	data
}