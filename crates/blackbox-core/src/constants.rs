/// AES-128-CBC key for decrypting .bin.e master data files.
/// This is the original game's hardcoded key.
pub const MASTER_DATA_AES_KEY: &[u8; 16] = &[
    0x47, 0x61, 0x6D, 0x65, 0x4D, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x4B, 0x65,
];

/// AES-128-CBC IV (zeroed by default — the actual IV is derived from the file header).
pub const MASTER_DATA_AES_IV: &[u8; 16] = &[
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// LZ4 extension type code used in msgpack for compressed master data tables.
pub const LZ4_EXT_TYPE_CODE: i8 = 99;

/// Original resource URL base string for asset CDN list.bin rewriting.
pub const RESOURCE_URL_BASE: &str = "https://assets.lunar-tear.example.com/";

/// Admin token environment variable name.
pub const ADMIN_TOKEN_ENV: &str = "BLACKBOX_ADMIN_TOKEN";
