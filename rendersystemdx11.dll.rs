/*
 * All offsets dumped with: https://frezzyhook.com
 * Fri, 9 Feb 2024 11:33:20 +0000
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod RenderInputLayoutField_t {
    pub const m_pSemanticName: usize = 0x0; // uint8_t[32]
    pub const m_nSemanticIndex: usize = 0x20; // int32_t
    pub const m_Format: usize = 0x24; // uint32_t
    pub const m_nOffset: usize = 0x28; // int32_t
    pub const m_nSlot: usize = 0x2C; // int32_t
    pub const m_nSlotType: usize = 0x30; // RenderSlotType_t
    pub const m_nInstanceStepRate: usize = 0x34; // int32_t
}

pub mod VsInputSignatureElement_t {
    pub const m_pName: usize = 0x0; // char[64]
    pub const m_pSemantic: usize = 0x40; // char[64]
    pub const m_pD3DSemanticName: usize = 0x80; // char[64]
    pub const m_nD3DSemanticIndex: usize = 0xC0; // int32_t
}

pub mod VsInputSignature_t {
    pub const m_elems: usize = 0x0; // CUtlVector<VsInputSignatureElement_t>
}