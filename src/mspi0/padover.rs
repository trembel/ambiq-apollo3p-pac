#[doc = "Reader of register PADOVER"]
pub type R = crate::R<u32, super::PADOVER>;
#[doc = "Writer for register PADOVER"]
pub type W = crate::W<u32, super::PADOVER>;
#[doc = "Register PADOVER `reset()`'s with value 0"]
impl crate::ResetValue for super::PADOVER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE`"]
pub type OVERRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVERRIDE`"]
pub struct OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Output pad override value. \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn override_(&mut self) -> OVERRIDE_W {
        OVERRIDE_W { w: self }
    }
}
