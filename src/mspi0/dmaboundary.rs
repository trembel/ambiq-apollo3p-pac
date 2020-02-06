#[doc = "Reader of register DMABOUNDARY"]
pub type R = crate::R<u32, super::DMABOUNDARY>;
#[doc = "Writer for register DMABOUNDARY"]
pub type W = crate::W<u32, super::DMABOUNDARY>;
#[doc = "Register DMABOUNDARY `reset()`'s with value 0"]
impl crate::ResetValue for super::DMABOUNDARY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Address boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMABOUND_A {
    #[doc = "0: Disable DMA address boundary breaks"]
    NONE = 0,
    #[doc = "1: Break at 32 byte boundary (0x20 increments)"]
    BREAK32 = 1,
    #[doc = "2: Break at 64 byte boundary (0x40 increments)"]
    BREAK64 = 2,
    #[doc = "3: Break at 128 byte boundary (0x80 increments)"]
    BREAK128 = 3,
    #[doc = "4: Break at 256 byte boundary (0x100 increments)"]
    BREAK256 = 4,
    #[doc = "5: Break at 512 byte boundary (0x200 increments)"]
    BREAK512 = 5,
    #[doc = "6: Break at 1KB boundary (0x400 increments)"]
    BREAK1K = 6,
    #[doc = "7: Break at 2KB boundary (0x800 increments)"]
    BREAK2K = 7,
    #[doc = "8: Break at 4KB boundary (0x1000 increments)"]
    BREAK4K = 8,
    #[doc = "9: Break at 8KB boundary (0x2000 increments)"]
    BREAK8K = 9,
    #[doc = "10: Break at 16KB boundary (0x4000 increments)"]
    BREAK16K = 10,
}
impl From<DMABOUND_A> for u8 {
    #[inline(always)]
    fn from(variant: DMABOUND_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMABOUND`"]
pub type DMABOUND_R = crate::R<u8, DMABOUND_A>;
impl DMABOUND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMABOUND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMABOUND_A::NONE),
            1 => Val(DMABOUND_A::BREAK32),
            2 => Val(DMABOUND_A::BREAK64),
            3 => Val(DMABOUND_A::BREAK128),
            4 => Val(DMABOUND_A::BREAK256),
            5 => Val(DMABOUND_A::BREAK512),
            6 => Val(DMABOUND_A::BREAK1K),
            7 => Val(DMABOUND_A::BREAK2K),
            8 => Val(DMABOUND_A::BREAK4K),
            9 => Val(DMABOUND_A::BREAK8K),
            10 => Val(DMABOUND_A::BREAK16K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMABOUND_A::NONE
    }
    #[doc = "Checks if the value of the field is `BREAK32`"]
    #[inline(always)]
    pub fn is_break32(&self) -> bool {
        *self == DMABOUND_A::BREAK32
    }
    #[doc = "Checks if the value of the field is `BREAK64`"]
    #[inline(always)]
    pub fn is_break64(&self) -> bool {
        *self == DMABOUND_A::BREAK64
    }
    #[doc = "Checks if the value of the field is `BREAK128`"]
    #[inline(always)]
    pub fn is_break128(&self) -> bool {
        *self == DMABOUND_A::BREAK128
    }
    #[doc = "Checks if the value of the field is `BREAK256`"]
    #[inline(always)]
    pub fn is_break256(&self) -> bool {
        *self == DMABOUND_A::BREAK256
    }
    #[doc = "Checks if the value of the field is `BREAK512`"]
    #[inline(always)]
    pub fn is_break512(&self) -> bool {
        *self == DMABOUND_A::BREAK512
    }
    #[doc = "Checks if the value of the field is `BREAK1K`"]
    #[inline(always)]
    pub fn is_break1k(&self) -> bool {
        *self == DMABOUND_A::BREAK1K
    }
    #[doc = "Checks if the value of the field is `BREAK2K`"]
    #[inline(always)]
    pub fn is_break2k(&self) -> bool {
        *self == DMABOUND_A::BREAK2K
    }
    #[doc = "Checks if the value of the field is `BREAK4K`"]
    #[inline(always)]
    pub fn is_break4k(&self) -> bool {
        *self == DMABOUND_A::BREAK4K
    }
    #[doc = "Checks if the value of the field is `BREAK8K`"]
    #[inline(always)]
    pub fn is_break8k(&self) -> bool {
        *self == DMABOUND_A::BREAK8K
    }
    #[doc = "Checks if the value of the field is `BREAK16K`"]
    #[inline(always)]
    pub fn is_break16k(&self) -> bool {
        *self == DMABOUND_A::BREAK16K
    }
}
#[doc = "Write proxy for field `DMABOUND`"]
pub struct DMABOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABOUND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMABOUND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable DMA address boundary breaks"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMABOUND_A::NONE)
    }
    #[doc = "Break at 32 byte boundary (0x20 increments)"]
    #[inline(always)]
    pub fn break32(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK32)
    }
    #[doc = "Break at 64 byte boundary (0x40 increments)"]
    #[inline(always)]
    pub fn break64(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK64)
    }
    #[doc = "Break at 128 byte boundary (0x80 increments)"]
    #[inline(always)]
    pub fn break128(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK128)
    }
    #[doc = "Break at 256 byte boundary (0x100 increments)"]
    #[inline(always)]
    pub fn break256(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK256)
    }
    #[doc = "Break at 512 byte boundary (0x200 increments)"]
    #[inline(always)]
    pub fn break512(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK512)
    }
    #[doc = "Break at 1KB boundary (0x400 increments)"]
    #[inline(always)]
    pub fn break1k(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK1K)
    }
    #[doc = "Break at 2KB boundary (0x800 increments)"]
    #[inline(always)]
    pub fn break2k(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK2K)
    }
    #[doc = "Break at 4KB boundary (0x1000 increments)"]
    #[inline(always)]
    pub fn break4k(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK4K)
    }
    #[doc = "Break at 8KB boundary (0x2000 increments)"]
    #[inline(always)]
    pub fn break8k(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK8K)
    }
    #[doc = "Break at 16KB boundary (0x4000 increments)"]
    #[inline(always)]
    pub fn break16k(self) -> &'a mut W {
        self.variant(DMABOUND_A::BREAK16K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DMATIMELIMIT`"]
pub type DMATIMELIMIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMATIMELIMIT`"]
pub struct DMATIMELIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATIMELIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - DMA Address boundary"]
    #[inline(always)]
    pub fn dmabound(&self) -> DMABOUND_R {
        DMABOUND_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in 100 ns increments. A value of 0 disables the counter."]
    #[inline(always)]
    pub fn dmatimelimit(&self) -> DMATIMELIMIT_R {
        DMATIMELIMIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:15 - DMA Address boundary"]
    #[inline(always)]
    pub fn dmabound(&mut self) -> DMABOUND_W {
        DMABOUND_W { w: self }
    }
    #[doc = "Bits 0:11 - DMA time limit. Can be used to limit the transaction time on the MSPI bus. The count is in 100 ns increments. A value of 0 disables the counter."]
    #[inline(always)]
    pub fn dmatimelimit(&mut self) -> DMATIMELIMIT_W {
        DMATIMELIMIT_W { w: self }
    }
}
