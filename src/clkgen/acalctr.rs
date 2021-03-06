#[doc = "Reader of register ACALCTR"]
pub type R = crate::R<u32, super::ACALCTR>;
#[doc = "Writer for register ACALCTR"]
pub type W = crate::W<u32, super::ACALCTR>;
#[doc = "Register ACALCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACALCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACALCTR`"]
pub type ACALCTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ACALCTR`"]
pub struct ACALCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACALCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[inline(always)]
    pub fn acalctr(&self) -> ACALCTR_R {
        ACALCTR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[inline(always)]
    pub fn acalctr(&mut self) -> ACALCTR_W {
        ACALCTR_W { w: self }
    }
}
