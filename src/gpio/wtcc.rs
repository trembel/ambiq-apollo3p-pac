#[doc = "Reader of register WTCC"]
pub type R = crate::R<u32, super::WTCC>;
#[doc = "Writer for register WTCC"]
pub type W = crate::W<u32, super::WTCC>;
#[doc = "Register WTCC `reset()`'s with value 0"]
impl crate::ResetValue for super::WTCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTCB`"]
pub type WTCB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WTCB`"]
pub struct WTCB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Clear the GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtcb(&self) -> WTCB_R {
        WTCB_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Clear the GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtcb(&mut self) -> WTCB_W {
        WTCB_W { w: self }
    }
}
