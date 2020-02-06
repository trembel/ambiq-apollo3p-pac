#[doc = "Reader of register FLASH0CFG"]
pub type R = crate::R<u32, super::FLASH0CFG>;
#[doc = "Writer for register FLASH0CFG"]
pub type W = crate::W<u32, super::FLASH0CFG>;
#[doc = "Register FLASH0CFG `reset()`'s with value 0x0873"]
impl crate::ResetValue for super::FLASH0CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0873
    }
}
#[doc = "Controls FLASH low power modes (control of LPM pin).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMMODE0_A {
    #[doc = "0: High power mode (LPM not used)."]
    NEVER = 0,
    #[doc = "1: Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    STANDBY = 1,
    #[doc = "2: Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    ALWAYS = 2,
}
impl From<LPMMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMMODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPMMODE0`"]
pub type LPMMODE0_R = crate::R<u8, LPMMODE0_A>;
impl LPMMODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPMMODE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPMMODE0_A::NEVER),
            1 => Val(LPMMODE0_A::STANDBY),
            2 => Val(LPMMODE0_A::ALWAYS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NEVER`"]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == LPMMODE0_A::NEVER
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMMODE0_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == LPMMODE0_A::ALWAYS
    }
}
#[doc = "Write proxy for field `LPMMODE0`"]
pub struct LPMMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMMODE0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "High power mode (LPM not used)."]
    #[inline(always)]
    pub fn never(self) -> &'a mut W {
        self.variant(LPMMODE0_A::NEVER)
    }
    #[doc = "Fast Standby mode. LPM deasserted for read operations, but asserted while FLASH IDLE."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMMODE0_A::STANDBY)
    }
    #[doc = "Low Power mode. LPM always asserted for reads. LPM_RD_WAIT must be programmed to accommodate longer read access times."]
    #[inline(always)]
    pub fn always(self) -> &'a mut W {
        self.variant(LPMMODE0_A::ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPMRDWAIT0`"]
pub type LPMRDWAIT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMRDWAIT0`"]
pub struct LPMRDWAIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMRDWAIT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEDELAY0`"]
pub type SEDELAY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEDELAY0`"]
pub struct SEDELAY0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEDELAY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `RDWAIT0`"]
pub type RDWAIT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWAIT0`"]
pub struct RDWAIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWAIT0_W<'a> {
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
    pub fn lpmmode0(&self) -> LPMMODE0_R {
        LPMMODE0_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait0(&self) -> LPMRDWAIT0_R {
        LPMRDWAIT0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay0(&self) -> SEDELAY0_R {
        SEDELAY0_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait0(&self) -> RDWAIT0_R {
        RDWAIT0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - Controls FLASH low power modes (control of LPM pin)."]
    #[inline(always)]
    pub fn lpmmode0(&mut self) -> LPMMODE0_W {
        LPMMODE0_W { w: self }
    }
    #[doc = "Bits 8:11 - Sets FLASH wait states when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)"]
    #[inline(always)]
    pub fn lpmrdwait0(&mut self) -> LPMRDWAIT0_W {
        LPMRDWAIT0_W { w: self }
    }
    #[doc = "Bits 4:6 - Sets SE delay (FLASH address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub fn sedelay0(&mut self) -> SEDELAY0_W {
        SEDELAY0_W { w: self }
    }
    #[doc = "Bits 0:3 - Sets read wait states for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub fn rdwait0(&mut self) -> RDWAIT0_W {
        RDWAIT0_W { w: self }
    }
}
