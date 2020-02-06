#[doc = "Reader of register RDC"]
pub type R = crate::R<u32, super::RDC>;
#[doc = "Writer for register RDC"]
pub type W = crate::W<u32, super::RDC>;
#[doc = "Register RDC `reset()`'s with value 0"]
impl crate::ResetValue for super::RDC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDC`"]
pub type RDC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RDC`"]
pub struct RDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - GPIO73-64 read data."]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - GPIO73-64 read data."]
    #[inline(always)]
    pub fn rdc(&mut self) -> RDC_W {
        RDC_W { w: self }
    }
}
