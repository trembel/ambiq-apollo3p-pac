#[doc = "Reader of register WTSC"]
pub type R = crate::R<u32, super::WTSC>;
#[doc = "Writer for register WTSC"]
pub type W = crate::W<u32, super::WTSC>;
#[doc = "Register WTSC `reset()`'s with value 0"]
impl crate::ResetValue for super::WTSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTSC`"]
pub type WTSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WTSC`"]
pub struct WTSC_W<'a> {
    w: &'a mut W,
}
impl<'a> WTSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Set the GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtsc(&self) -> WTSC_R {
        WTSC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Set the GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtsc(&mut self) -> WTSC_W {
        WTSC_W { w: self }
    }
}
