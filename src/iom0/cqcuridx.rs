#[doc = "Reader of register CQCURIDX"]
pub type R = crate::R<u32, super::CQCURIDX>;
#[doc = "Writer for register CQCURIDX"]
pub type W = crate::W<u32, super::CQCURIDX>;
#[doc = "Register CQCURIDX `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCURIDX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CQCURIDX`"]
pub type CQCURIDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CQCURIDX`"]
pub struct CQCURIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCURIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command queue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CQCURIDX_R {
        CQCURIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command queue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[inline(always)]
    pub fn cqcuridx(&mut self) -> CQCURIDX_W {
        CQCURIDX_W { w: self }
    }
}
