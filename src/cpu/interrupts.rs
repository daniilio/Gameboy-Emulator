// Zero-Page Jump Vectors
const INTERRUPT_VECTORS = {0040, 0048, 0050, 0058, 0060}

struct InterruptController {
    // IE register (0xFFFF)
    interrupt_enable: u8,
}

impl InterruptController {
    fn set_interrupt_enable()
    {

    }
}
