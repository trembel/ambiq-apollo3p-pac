#[doc = "Reader of register FLASHRPROT2"]
pub type R = crate::R<u32, super::FLASHRPROT2>;
#[doc = "Writer for register FLASHRPROT2"]
pub type W = crate::W<u32, super::FLASHRPROT2>;
#[doc = "Register FLASHRPROT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHRPROT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FR2BITS`"]
pub type FR2BITS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FR2BITS`"]
pub struct FR2BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> FR2BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr2bits(&self) -> FR2BITS_R {
        FR2BITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Copy (read) protect flash 0x00100000 - 0x0017FFFF. Each bit provides read protection for 16KB chunks of flash. Bits are cleared by writing a 1 to the bit. When read, 0 indicates the region is protected. Bits are sticky (can be set when PROTLOCK is 1, but only cleared by reset)"]
    #[inline(always)]
    pub fn fr2bits(&mut self) -> FR2BITS_W {
        FR2BITS_W { w: self }
    }
}
