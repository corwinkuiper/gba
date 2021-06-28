#![no_std]
#![no_main]

extern crate agb;
#[no_mangle]
pub fn main() -> ! {
    let mut gba = agb::Gba::new();
    let mut mgba = agb::mgba::Mgba::new().unwrap();

    let vblank = agb::interrupt::VBlank::new();

    let mut count = 0;
    loop {
        vblank.wait_for_vblank();

        mgba.print(
            format_args!("Hello, world, frame = {}", count),
            agb::mgba::DebugLevel::Info,
        )
        .unwrap();

        count += 1;
    }
}
