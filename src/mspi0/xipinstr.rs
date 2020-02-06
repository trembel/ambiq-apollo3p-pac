#[doc = "Reader of register XIPINSTR"]
pub type R = crate::R<u32, super::XIPINSTR>;
#[doc = "Writer for register XIPINSTR"]
pub type W = crate::W<u32, super::XIPINSTR>;
#[doc = "Register XIPINSTR `reset()`'s with value 0x000b_0006"]
impl crate::ResetValue for super::XIPINSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000b_0006
    }
}
#[doc = "Reader of field `READINSTR`"]
pub type READINSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `READINSTR`"]
pub struct READINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> READINSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WRITEINSTR`"]
pub type WRITEINSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WRITEINSTR`"]
pub struct WRITEINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEINSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    pub fn readinstr(&self) -> READINSTR_R {
        READINSTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Write command sent for DMA operations"]
    #[inline(always)]
    pub fn writeinstr(&self) -> WRITEINSTR_R {
        WRITEINSTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    pub fn readinstr(&mut self) -> READINSTR_W {
        READINSTR_W { w: self }
    }
    #[doc = "Bits 0:15 - Write command sent for DMA operations"]
    #[inline(always)]
    pub fn writeinstr(&mut self) -> WRITEINSTR_W {
        WRITEINSTR_W { w: self }
    }
}
