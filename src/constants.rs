// PPPoE constants
pub const PPPOE_TAG_HUNIQUE: u16 = 0x0103;
pub const PPPOE_TAG_ACOOKIE: u16 = 0x0104;

pub const ETHERTYPE_PPPOEDISC: u16 = 0x8863;
pub const ETHERTYPE_PPPOE: u16 = 0x8864;

pub const PCPP_PPP: u8 = 33;

// FreeBSD constants
pub const ZERO: i32 = 0;

pub const PAGE_SIZE: usize = 0x4000;

pub const IDT_UD: usize = 6;
pub const SDT_SYSIGT: usize = 14;
pub const SEL_KPL: usize = 0;

// CR0 flags
pub const CR0_PE: u32 = 0x00000001;
pub const CR0_MP: u32 = 0x00000002;
pub const CR0_EM: u32 = 0x00000004;
pub const CR0_TS: u32 = 0x00000008;
pub const CR0_ET: u32 = 0x00000010;
pub const CR0_NE: u32 = 0x00000020;
pub const CR0_WP: u32 = 0x00010000;
pub const CR0_AM: u32 = 0x00040000;
pub const CR0_NW: u32 = 0x20000000;
pub const CR0_CD: u32 = 0x40000000;
pub const CR0_PG: u32 = 0x80000000;

pub const CR0_ORI: u32 = CR0_PG | CR0_AM | CR0_WP | CR0_NE | CR0_ET | CR0_TS | CR0_MP | CR0_PE;

// VM_PROT flags
pub const VM_PROT_READ: u32 = 0x01;
pub const VM_PROT_WRITE: u32 = 0x02;
pub const VM_PROT_EXECUTE: u32 = 0x04;

pub const VM_PROT_ALL: u32 = VM_PROT_READ | VM_PROT_WRITE | VM_PROT_EXECUTE;

// LLE flags
pub const LLE_STATIC: u32 = 0x0002;
pub const LLE_LINKED: u32 = 0x0040;
pub const LLE_EXCLUSIVE: u32 = 0x2000;

// LO flags
pub const LO_INITIALIZED: u32 = 0x00010000;
pub const LO_WITNESS: u32 = 0x00020000;
pub const LO_UPGRADABLE: u32 = 0x00200000;
pub const LO_DUPOK: u32 = 0x00400000;

pub const LO_CLASSSHIFT: u32 = 24;

// RW and MTX flags
pub const RW_UNLOCKED: u32 = 1;
pub const MTX_UNOWNED: u32 = 4;

pub const RW_INIT_FLAGS: u32 = (4 << LO_CLASSSHIFT) | LO_INITIALIZED | LO_WITNESS | LO_UPGRADABLE;
pub const MTX_INIT_FLAGS: u32 = (1 << LO_CLASSSHIFT) | LO_INITIALIZED | LO_WITNESS;

pub const CALLOUT_RETURNUNLOCKED: u32 = 0x10;

// Networking constants
pub const AF_INET6: i32 = 28;

pub const IFT_ETHER: u16 = 0x6;

pub const ND6_LLINFO_NOSTATE: u16 = 0xfffe;

// FreeBSD offsets
pub const TARGET_SIZE: usize = 0x1;
