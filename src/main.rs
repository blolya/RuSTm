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
    i2c1.ccr.write_or(180);

    i2c1.trise.write(36);

    i2c1.cr1.set_bit(0);


    let addr = i2c_read(&i2c1, 0x0212);
    dbgr.send("Device address is: ");
    dbgr.send_char(addr as char);
    dbgr.send("_____________________________________________-\r\n");


    i2c_write(&i2c1, 0x0207, 0x01);
    i2c_write(&i2c1, 0x0208, 0x01);
    i2c_write(&i2c1, 0x0096, 0x00);
    i2c_write(&i2c1, 0x0097, 0xfd);
    i2c_write(&i2c1, 0x00e3, 0x00);
    i2c_write(&i2c1, 0x00e4, 0x04);
    i2c_write(&i2c1, 0x00e5, 0x02);
    i2c_write(&i2c1, 0x00e6, 0x01);
    i2c_write(&i2c1, 0x00e7, 0x03);
    i2c_write(&i2c1, 0x00f5, 0x02);
    i2c_write(&i2c1, 0x00d9, 0x05);
    i2c_write(&i2c1, 0x00db, 0xce);
    i2c_write(&i2c1, 0x00dc, 0x03);
    i2c_write(&i2c1, 0x00dd, 0xf8);
    i2c_write(&i2c1, 0x009f, 0x00);
    i2c_write(&i2c1, 0x00a3, 0x3c);
    i2c_write(&i2c1, 0x00b7, 0x00);
    i2c_write(&i2c1, 0x00bb, 0x3c);
    i2c_write(&i2c1, 0x00b2, 0x09);
    i2c_write(&i2c1, 0x00ca, 0x09);
    i2c_write(&i2c1, 0x0198, 0x01);
    i2c_write(&i2c1, 0x01b0, 0x17);
    i2c_write(&i2c1, 0x01ad, 0x00);
    i2c_write(&i2c1, 0x00ff, 0x05);
    i2c_write(&i2c1, 0x0100, 0x05);
    i2c_write(&i2c1, 0x0199, 0x05);
    i2c_write(&i2c1, 0x01a6, 0x1b);
    i2c_write(&i2c1, 0x01ac, 0x3e);
    i2c_write(&i2c1, 0x01a7, 0x1f);
    i2c_write(&i2c1, 0x0030, 0x00);
    i2c_write(&i2c1, 0x0011, 0x10);
    i2c_write(&i2c1, 0x010a, 0x30);
    i2c_write(&i2c1, 0x003f, 0x46);
    i2c_write(&i2c1, 0x0031, 0xFF);
    i2c_write(&i2c1, 0x0040, 0x63);
    i2c_write(&i2c1, 0x002e, 0x01);
    i2c_write(&i2c1, 0x001b, 0x09);
    i2c_write(&i2c1, 0x003e, 0x31);
    i2c_write(&i2c1, 0x0014, 0x24);

    dbgr.send("Settings are loaded\r\n");

    i2c_write(&i2c1, 0x0016, 0x00);

    dbgr.send("Init complete\r\n");

    
    i2c_write(&i2c1, 0x0018, 0x03);
    loop {


        let status = i2c_read(&i2c1, 0x4f);
        let range_status = status & 0x07;

        if range_status == 0x04 {

            let data = i2c_read(&i2c1, 0x62);
            i2c_write(&i2c1, 0x0015, 0x07);

            dbgr.send_char(data as char);
            dbgr.send("\r\n");
        }

    }
}


fn i2c_write(i2c: &I2c1, address: u32, data: u8) {
    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();

    
    i2c.dr.write(0x29 << 1);
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();
    
    i2c.dr.write(((address >> 8) & 0xff) as u32);
    while i2c.sr1.get_bit(7) == 0 {};
    i2c.dr.write((address & 0xff) as u32);
    while i2c.sr1.get_bit(7) == 0 {};
    
    i2c.dr.write(data as u32);
    while i2c.sr1.get_bit(2) == 0 {};
    i2c.cr1.set_bit(9);	
    
}

fn i2c_read(i2c: &I2c1, address: u32) -> u8 {
    
    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();

    i2c.dr.write(0x29 << 1);
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();


    i2c.dr.write(((address >> 8) & 0xff) as u32);
    while i2c.sr1.get_bit(7) == 0 {};
    i2c.dr.write((address & 0xff) as u32);
    while i2c.sr1.get_bit(7) == 0 {};
    i2c.cr1.set_bit(9);	


    i2c.cr1.set_bit(8);
    while i2c.sr1.get_bit(0) == 0 {};
    i2c.sr1.read();


    i2c.dr.write((0x29 << 1) + 1);
    while i2c.sr1.get_bit(1) == 0 {};
    i2c.sr1.read();
    i2c.sr2.read();

		
    i2c.cr1.write_and(!0x0000_0400);
    while i2c.sr1.get_bit(6) == 0 {};
    let data: u8 = i2c.dr.read() as u8;			
    i2c.cr1.set_bit(9);	

	data
}