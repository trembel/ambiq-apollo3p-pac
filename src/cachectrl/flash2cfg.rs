#[doc = "Reader of register FLASH2CFG"]
pub type R = crate::R<u32, super::FLASH2CFG>;
#[doc = "Writer for register FLASH2CFG"]
pub type W = crate::W<u32, super::FLASH2CFG>;
#[doc = "Register FLASH2CFG `reset()`'s with value 0x0873"]
impl crate::ResetValue for super::FLASH2CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0873
    }
}
#[doc = "Controls FLASH low power modes (control of LPM pin).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMMODE2_A {
    #[doc = "0: High power mode (LPM not used)."]
    NEVER = 0,
    #[doc = "1: Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    STANDBY = 1,
    #[doc = "2: Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    ALWAYS = 2,
}
impl From<LPMMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMMODE2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMMODE2`"]
pub type LPMMODE2_R = crate::R<u8, LPMMODE2_A>;
impl LPMMODE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPMMODE2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPMMODE2_A::NEVER),
            1 => Val(LPMMODE2_A::STANDBY),
            2 => Val(LPMMODE2_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == LPMMODE2_A::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMMODE2_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == LPMMODE2_A::ALWAYS
    }
}
#[doc = "Write proxy for field `LPMMODE2`"]
pub struct LPMMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMMODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMMODE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High power mode (LPM not used)."]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODE2_A::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODE2_A::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODE2_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPMRDWAIT2`"]
pub type LPMRDWAIT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMRDWAIT2`"]
pub struct LPMRDWAIT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMRDWAIT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEDELAY2`"]
pub type SEDELAY2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEDELAY2`"]
pub struct SEDELAY2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDELAY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RDWAIT2`"]
pub type RDWAIT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWAIT2`"]
pub struct RDWAIT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWAIT2_W<'a> {
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
    pub fn lpmmode2(&self) -> LPMMODE2_R {
        LPMMODE2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait2(&self) -> LPMRDWAIT2_R {
        LPMRDWAIT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay2(&self) -> SEDELAY2_R {
        SEDELAY2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait2(&self) -> RDWAIT2_R {
        RDWAIT2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Controls FLASH low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode2(&mut self) -> LPMMODE2_W {
        LPMMODE2_W { w: self }
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait2(&mut self) -> LPMRDWAIT2_W {
        LPMRDWAIT2_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay2(&mut self) -> SEDELAY2_W {
        SEDELAY2_W { w: self }
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait2(&mut self) -> RDWAIT2_W {
        RDWAIT2_W { w: self }
    }
}
