#[doc = "Reader of register ENCC"]
pub type R = crate::R<u32, super::ENCC>;
#[doc = "Writer for register ENCC"]
pub type W = crate::W<u32, super::ENCC>;
#[doc = "Register ENCC `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENCC`"]
pub type ENCC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENCC`"]
pub struct ENCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Clear the GPIO73-64 output enables"]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Clear the GPIO73-64 output enables"]
    #[inline(always)]
    pub fn encc(&mut self) -> ENCC_W {
        ENCC_W { w: self }
    }
}
