// PPPoE constants

pub const ETHERTYPE_PPPOEDISC: u16 = 0x8863;
pub const PPPOE_CODE_PADI: u8 = 0x09;
pub const PPPOE_CODE_PADO: u8 = 0x07;
pub const PPPOE_SOFTC_SC_AC_COOKIE: u8 = 0x40;

pub const ETH_HEADER_LEN: usize = 14;
pub const ETH_SOURCE_MAC: usize = 6;
pub const PPPOE_HEADER_LEN: usize = 6;
pub const PPPOE_TAG_HEADER_LEN: usize = 4;
pub const PPPOE_SOFTC_SC_DEST: u64 = 0x24;

pub const PPPOE_TAG_HOST_UNIQ: u16 = 0x0103;

// Define constants used in the code
pub const ZERO: u64 = 0;
pub const IFT_ETHER: u8 = 0x06;
pub const MTX_UNOWNED: u64 = 4;

pub const LO_INITIALIZED: u32 = 0x00010000;
pub const LO_WITNESS: u32 = 0x00020000;
pub const LO_UPGRADABLE: u32 = 0x00200000;
pub const RW_UNLOCKED: u64 = 1;

pub const LO_CLASSSHIFT: u32 = 24;

pub const RW_INIT_FLAGS: u32 = (4 << LO_CLASSSHIFT) | LO_INITIALIZED | LO_WITNESS | LO_UPGRADABLE;
pub const MTX_INIT_FLAGS: u32 = (1 << LO_CLASSSHIFT) | LO_INITIALIZED | LO_WITNESS;
