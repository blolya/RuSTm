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

    let i2c1 = I2c1::new();

    let dbgr = Usart1::new();

    let gpioc = Gpioc::new();
    let pc13 = Port::new(PortNum::P13, PortMode::Output(OutputConfig::GeneralPurposePushPull(MaxSpeed::S50MHz)), &gpioc);

    pc13.set_high();


    let addr = i2c1.read(0x0212);
    dbgr.send("Device address is: ");
    dbgr.send_char(addr as char);
    dbgr.send("_____________________________________________-\r\n");


    i2c1.write(0x0207, 0x01);
    i2c1.write(0x0208, 0x01);
    i2c1.write(0x0096, 0x00);
    i2c1.write(0x0097, 0xfd);
    i2c1.write(0x00e3, 0x00);
    i2c1.write(0x00e4, 0x04);
    i2c1.write(0x00e5, 0x02);
    i2c1.write(0x00e6, 0x01);
    i2c1.write(0x00e7, 0x03);
    i2c1.write(0x00f5, 0x02);
    i2c1.write(0x00d9, 0x05);
    i2c1.write(0x00db, 0xce);
    i2c1.write(0x00dc, 0x03);
    i2c1.write(0x00dd, 0xf8);
    i2c1.write(0x009f, 0x00);
    i2c1.write(0x00a3, 0x3c);
    i2c1.write(0x00b7, 0x00);
    i2c1.write(0x00bb, 0x3c);
    i2c1.write(0x00b2, 0x09);
    i2c1.write(0x00ca, 0x09);
    i2c1.write(0x0198, 0x01);
    i2c1.write(0x01b0, 0x17);
    i2c1.write(0x01ad, 0x00);
    i2c1.write(0x00ff, 0x05);
    i2c1.write(0x0100, 0x05);
    i2c1.write(0x0199, 0x05);
    i2c1.write(0x01a6, 0x1b);
    i2c1.write(0x01ac, 0x3e);
    i2c1.write(0x01a7, 0x1f);
    i2c1.write(0x0030, 0x00);
    i2c1.write(0x0011, 0x10);
    i2c1.write(0x010a, 0x30);
    i2c1.write(0x003f, 0x46);
    i2c1.write(0x0031, 0xFF);
    i2c1.write(0x0040, 0x63);
    i2c1.write(0x002e, 0x01);
    i2c1.write(0x001b, 0x09);
    i2c1.write(0x003e, 0x31);
    i2c1.write(0x0014, 0x24);

    dbgr.send("Settings are loaded\r\n");

    i2c1.write(0x0016, 0x00);

    dbgr.send("Init complete\r\n");

    
    i2c1.write(0x0018, 0x03);
    loop {


        let status = i2c1.read( 0x4f);
        let range_status = status & 0x07;

        if range_status == 0x04 {

            let data = i2c1.read(0x62);
            i2c1.write(0x0015, 0x07);

            dbgr.send_char(data as char);
        }

    }
}
