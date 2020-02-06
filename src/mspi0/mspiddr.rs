#[doc = "Reader of register MSPIDDR"]
pub type R = crate::R<u32, super::MSPIDDR>;
#[doc = "Writer for register MSPIDDR"]
pub type W = crate::W<u32, super::MSPIDDR>;
#[doc = "Register MSPIDDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MSPIDDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDQSDELAY`"]
pub type TXDQSDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDQSDELAY`"]
pub struct TXDQSDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDQSDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXDQSDELAY`"]
pub type RXDQSDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXDQSDELAY`"]
pub struct RXDQSDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDQSDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENABLEFINEDELAY`"]
pub type ENABLEFINEDELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEFINEDELAY`"]
pub struct ENABLEFINEDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEFINEDELAY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OVERRIDEDDRCLKOUTDELAY`"]
pub type OVERRIDEDDRCLKOUTDELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRIDEDDRCLKOUTDELAY`"]
pub struct OVERRIDEDDRCLKOUTDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDEDDRCLKOUTDELAY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OVERRIDERXDQSDELAY`"]
pub type OVERRIDERXDQSDELAY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRIDERXDQSDELAY`"]
pub struct OVERRIDERXDQSDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDERXDQSDELAY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DQSSYNCNEG`"]
pub type DQSSYNCNEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSSYNCNEG`"]
pub struct DQSSYNCNEG_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSSYNCNEG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ENABLEDQS`"]
pub type ENABLEDQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEDQS`"]
pub struct ENABLEDQS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEDQS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `QUADDDR`"]
pub type QUADDDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QUADDDR`"]
pub struct QUADDDR_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADDDR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EMULATEDDR`"]
pub type EMULATEDDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMULATEDDR`"]
pub struct EMULATEDDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EMULATEDDR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - When OVERRIDEDQSDELAY is set this sets the DQS delay line value. In ENABLEDQS mode, this acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn txdqsdelay(&self) -> TXDQSDELAY_R {
        TXDQSDELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - When OVERRIDEDQSDELAY is set this sets the DQS delay line value. In ENABLEDQS mode, this acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn rxdqsdelay(&self) -> RXDQSDELAY_R {
        RXDQSDELAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Enables use of delay line to provide fine control over traditional RX capture clock."]
    #[inline(always)]
    pub fn enablefinedelay(&self) -> ENABLEFINEDELAY_R {
        ENABLEFINEDELAY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Override TX delay line with the value in DQSDELAY (for TX clock offset when in QUADDDR mode)"]
    #[inline(always)]
    pub fn overrideddrclkoutdelay(&self) -> OVERRIDEDDRCLKOUTDELAY_R {
        OVERRIDEDDRCLKOUTDELAY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Override DQS delay line with the value in DQSDELAY (for RX capture in QUADDDR mode)"]
    #[inline(always)]
    pub fn overriderxdqsdelay(&self) -> OVERRIDERXDQSDELAY_R {
        OVERRIDERXDQSDELAY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Use negative edge of clock for DDR data sync"]
    #[inline(always)]
    pub fn dqssyncneg(&self) -> DQSSYNCNEG_R {
        DQSSYNCNEG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - In EMULATEDDR mode, enable DQS for read capture"]
    #[inline(always)]
    pub fn enabledqs(&self) -> ENABLEDQS_R {
        ENABLEDQS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables use of delay line to provide fine control over traditional RX capture clock."]
    #[inline(always)]
    pub fn quadddr(&self) -> QUADDDR_R {
        QUADDDR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Drive external clock at 1/2 rate to emulate DDR mode"]
    #[inline(always)]
    pub fn emulateddr(&self) -> EMULATEDDR_R {
        EMULATEDDR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - When OVERRIDEDQSDELAY is set this sets the DQS delay line value. In ENABLEDQS mode, this acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn txdqsdelay(&mut self) -> TXDQSDELAY_W {
        TXDQSDELAY_W { w: self }
    }
    #[doc = "Bits 8:12 - When OVERRIDEDQSDELAY is set this sets the DQS delay line value. In ENABLEDQS mode, this acts as an offset to the computed value (should be set to 0 by default)"]
    #[inline(always)]
    pub fn rxdqsdelay(&mut self) -> RXDQSDELAY_W {
        RXDQSDELAY_W { w: self }
    }
    #[doc = "Bit 6 - Enables use of delay line to provide fine control over traditional RX capture clock."]
    #[inline(always)]
    pub fn enablefinedelay(&mut self) -> ENABLEFINEDELAY_W {
        ENABLEFINEDELAY_W { w: self }
    }
    #[doc = "Bit 5 - Override TX delay line with the value in DQSDELAY (for TX clock offset when in QUADDDR mode)"]
    #[inline(always)]
    pub fn overrideddrclkoutdelay(&mut self) -> OVERRIDEDDRCLKOUTDELAY_W {
        OVERRIDEDDRCLKOUTDELAY_W { w: self }
    }
    #[doc = "Bit 4 - Override DQS delay line with the value in DQSDELAY (for RX capture in QUADDDR mode)"]
    #[inline(always)]
    pub fn overriderxdqsdelay(&mut self) -> OVERRIDERXDQSDELAY_W {
        OVERRIDERXDQSDELAY_W { w: self }
    }
    #[doc = "Bit 3 - Use negative edge of clock for DDR data sync"]
    #[inline(always)]
    pub fn dqssyncneg(&mut self) -> DQSSYNCNEG_W {
        DQSSYNCNEG_W { w: self }
    }
    #[doc = "Bit 2 - In EMULATEDDR mode, enable DQS for read capture"]
    #[inline(always)]
    pub fn enabledqs(&mut self) -> ENABLEDQS_W {
        ENABLEDQS_W { w: self }
    }
    #[doc = "Bit 1 - Enables use of delay line to provide fine control over traditional RX capture clock."]
    #[inline(always)]
    pub fn quadddr(&mut self) -> QUADDDR_W {
        QUADDDR_W { w: self }
    }
    #[doc = "Bit 0 - Drive external clock at 1/2 rate to emulate DDR mode"]
    #[inline(always)]
    pub fn emulateddr(&mut self) -> EMULATEDDR_W {
        EMULATEDDR_W { w: self }
    }
}
