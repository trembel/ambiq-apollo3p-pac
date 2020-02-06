#[doc = "Reader of register PADOVEREN"]
pub type R = crate::R<u32, super::PADOVEREN>;
#[doc = "Writer for register PADOVEREN"]
pub type W = crate::W<u32, super::PADOVEREN>;
#[doc = "Register PADOVEREN `reset()`'s with value 0"]
impl crate::ResetValue for super::PADOVEREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDEEN`"]
pub type OVERRIDEEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OVERRIDEEN`"]
pub struct OVERRIDEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDEEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn overrideen(&self) -> OVERRIDEEN_R {
        OVERRIDEEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Output pad override enable. Bit mask for pad outputs. When set to 1, the values in the OVERRIDE field are driven on the pad (output enable is implicitly set in this mode). \\[7:0\\]=data \\[8\\]=clock \\[9\\]=DM"]
    #[inline(always)]
    pub fn overrideen(&mut self) -> OVERRIDEEN_W {
        OVERRIDEEN_W { w: self }
    }
}
