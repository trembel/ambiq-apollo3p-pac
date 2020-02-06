#[doc = "Reader of register ENSC"]
pub type R = crate::R<u32, super::ENSC>;
#[doc = "Writer for register ENSC"]
pub type W = crate::W<u32, super::ENSC>;
#[doc = "Register ENSC `reset()`'s with value 0"]
impl crate::ResetValue for super::ENSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENSC`"]
pub type ENSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENSC`"]
pub struct ENSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Set the GPIO73-64 output enables"]
    #[inline(always)]
    pub fn ensc(&self) -> ENSC_R {
        ENSC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the GPIO73-64 output enables"]
    #[inline(always)]
    pub fn ensc(&mut self) -> ENSC_W {
        ENSC_W { w: self }
    }
}
