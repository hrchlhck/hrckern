#[repr(C)]
pub struct EAXE820 {
    pub bp: u64,
    pub region_lenth: u64,
    pub free_mem: u32,
    pub reserved: u32, 
    pub acpi_reclaimable: u32,
    pub acpi_nvs: u32,
    pub reserved_bad: u32
}

pub const MULTIBOOT2_MAGIC_NUMBER: u64 = 0x36D76289;