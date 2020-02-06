#[doc = "Reader of register WTC"]
pub type R = crate::R<u32, super::WTC>;
#[doc = "Writer for register WTC"]
pub type W = crate::W<u32, super::WTC>;
#[doc = "Register WTC `reset()`'s with value 0"]
impl crate::ResetValue for super::WTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTC`"]
pub type WTC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WTC`"]
pub struct WTC_W<'a> {
    w: &'a mut W,
}
impl<'a> WTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtc(&self) -> WTC_R {
        WTC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - GPIO73-64 write data."]
    #[inline(always)]
    pub fn wtc(&mut self) -> WTC_W {
        WTC_W { w: self }
    }
}
