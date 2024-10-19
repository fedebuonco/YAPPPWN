// PPPoE constants
pub const ETHERTYPE_PPPOEDISC: u16 = 0x8863;
pub const ETHERTYPE_PPPOESESS: u16 = 0x8864;
pub const ETHERTYPE_IPV6: u16 = 0x86dd;
pub const ETHERTYPE_LCP: u16 = 0xc021;
pub const ETHERTYPE_IPCP: u16 = 0x8021;
pub const PPPOE_CODE_PADI: u8 = 0x09;
pub const PPPOE_CODE_PADR: u8 = 0x19;
pub const IPCPV6_RS: u8 = 0x85;
pub const PPPOE_CODE_PADO: u8 = 0x07;
pub const PPPOE_CODE_PADS: u8 = 0x65;
pub const PPPOE_CODE_PADT: u8 = 0xa7;

pub const PPPOE_CODE_LCP_ECHO_REQ: u8 = 0x09;
pub const PPPOE_SESSION_ID: u16 = 0xffff;
pub const ETH_HEADER_LEN: usize = 14;
pub const ETH_SOURCE_MAC: usize = 6;
pub const PPPOE_HEADER_LEN: usize = 6;
pub const PPPOE_TAG_HEADER_LEN: usize = 4;
pub const PPPOE_SOFTC_SC_DEST: u64 = 0x24;
pub const SPRAY_NUM: u32 = 0x1000;
pub const PIN_NUM: u32 = 0x1000;
pub const LCP_ID: u8 = 0x41;
pub const FAKE_PRIMARY_DNS_SERVER: [u8; 4] = [0, 0, 0, 0];
pub const LCP_CONF_REQ: u8 = 1;
pub const LCP_CONF_ACK: u8 = 2;
pub const LCP_CONF_REJECT: u8 = 4;
pub const LCP_TERM_REQ: u8 = 5;

pub const HOLE_START: u32 = 0x400;
pub const HOLE_SPACE: u32 = 0x10;
pub const PPPOE_SOFTC_SC_AC_COOKIE: u64 = 0x40;
pub const LO_DUPOK: u32 = 0x00400000;
pub const LLTABLE_LLTIFP: u64 = 0x110;
pub const LLE_EXCLUSIVE: u16 = 0x2000;
pub const ND6_LLINFO_NOSTATE: u16 = 0xfffe;
pub const CORRUPT_NUM: u8 = 0x1;
pub const IP_ADDRESS_TYPE: u8 = 3;
pub const IPCP_CONF_REQ: u8 = 1;
pub const IPCP_CONF_ACK: u8 = 2;
pub const IPCP_CONF_NAK: u8 = 3;
pub const SOURCE_IPV4: [u8; 4] = [0x29, 0x29, 0x29, 0x29];
pub const TARGET_IPV4: [u8; 4] = [0x2A, 0x2A, 0x2A, 0x2A];

pub const PPPOE_TAG_HOST_UNIQ: u16 = 0x0103;
pub const PPPOE_TAG_ACOOKIE: u16 = 0x0104;

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
