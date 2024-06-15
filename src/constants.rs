// PPPoE constants

pub const ETHERTYPE_PPPOEDISC: u16 = 0x8863;
pub const PPPOE_CODE_PADI: u8 = 0x09;

pub const ETH_HEADER_LEN: usize = 14;
pub const ETH_SOURCE_MAC: usize = 6;
pub const PPPOE_HEADER_LEN: usize = 6;
pub const PPPOE_TAG_HEADER_LEN: usize = 4;

pub const PPPOE_TAG_HOST_UNIQ: u16 = 0x0103;

// Define constants used in the code
pub const ZERO: u64 = 0;
pub const IFT_ETHER: u8 = 0x06;
pub const RW_INIT_FLAGS: u32 = 0;
pub const RW_UNLOCKED: u64 = 0;
pub const MTX_INIT_FLAGS: u32 = 0;
pub const MTX_UNOWNED: u64 = 0;
