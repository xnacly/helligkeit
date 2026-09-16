pub const HEADER: [u8; 8] = [0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];

/// edid structure
///
/// taken from https://github.com/torvalds/linux/blob/master/include/drm/drm_edid.h#L291
#[repr(C)]
pub struct Edid {
    pub product_id: ProductId,
    pub name: Option<String>,
}

#[repr(C, packed)]
pub struct ProductId {
    pub manufacturer: [u8; 2],
    pub product_code: u16,
    pub serial: u32,
}
