#[doc = "Reader of register FLASH3CFG"]
pub type R = crate::R<u32, super::FLASH3CFG>;
#[doc = "Writer for register FLASH3CFG"]
pub type W = crate::W<u32, super::FLASH3CFG>;
#[doc = "Register FLASH3CFG `reset()`'s with value 0x0873"]
impl crate::ResetValue for super::FLASH3CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0873
    }
}
#[doc = "Controls FLASH low power modes (control of LPM pin).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMMODE3_A {
    #[doc = "0: High power mode (LPM not used)."]
    NEVER = 0,
    #[doc = "1: Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    STANDBY = 1,
    #[doc = "2: Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    ALWAYS = 2,
}
impl From<LPMMODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMMODE3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMMODE3`"]
pub type LPMMODE3_R = crate::R<u8, LPMMODE3_A>;
impl LPMMODE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPMMODE3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPMMODE3_A::NEVER),
            1 => Val(LPMMODE3_A::STANDBY),
            2 => Val(LPMMODE3_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == LPMMODE3_A::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMMODE3_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == LPMMODE3_A::ALWAYS
    }
}
#[doc = "Write proxy for field `LPMMODE3`"]
pub struct LPMMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMMODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMMODE3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High power mode (LPM not used)."]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODE3_A::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODE3_A::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODE3_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPMRDWAIT3`"]
pub type LPMRDWAIT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMRDWAIT3`"]
pub struct LPMRDWAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMRDWAIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEDELAY3`"]
pub type SEDELAY3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEDELAY3`"]
pub struct SEDELAY3_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDELAY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RDWAIT3`"]
pub type RDWAIT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWAIT3`"]
pub struct RDWAIT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWAIT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13 - Controls FLASH low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode3(&self) -> LPMMODE3_R {
        LPMMODE3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait3(&self) -> LPMRDWAIT3_R {
        LPMRDWAIT3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay3(&self) -> SEDELAY3_R {
        SEDELAY3_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait3(&self) -> RDWAIT3_R {
        RDWAIT3_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Controls FLASH low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode3(&mut self) -> LPMMODE3_W {
        LPMMODE3_W { w: self }
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait3(&mut self) -> LPMRDWAIT3_W {
        LPMRDWAIT3_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay3(&mut self) -> SEDELAY3_W {
        SEDELAY3_W { w: self }
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait3(&mut self) -> RDWAIT3_W {
        RDWAIT3_W { w: self }
    }
}
