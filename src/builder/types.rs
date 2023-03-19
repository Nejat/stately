use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct NodeType: u8 {
        const END   = 0b10;
        const START = 0b01;
        const STATE = 0b00;
    }
}
