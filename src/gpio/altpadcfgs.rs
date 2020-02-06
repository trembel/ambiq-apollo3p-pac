#[doc = "Reader of register ALTPADCFGS"]
pub type R = crate::R<u32, super::ALTPADCFGS>;
#[doc = "Writer for register ALTPADCFGS"]
pub type W = crate::W<u32, super::ALTPADCFGS>;
#[doc = "Register ALTPADCFGS `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 73 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD73_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD73_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD73_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD73_SR`"]
pub type PAD73_SR_R = crate::R<bool, PAD73_SR_A>;
impl PAD73_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD73_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD73_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD73_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD73_SR`"]
pub struct PAD73_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD73_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD73_SR_A::SR_EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PAD73_DS1`"]
pub type PAD73_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD73_DS1`"]
pub struct PAD73_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73_DS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 72 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD72_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD72_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD72_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD72_SR`"]
pub type PAD72_SR_R = crate::R<bool, PAD72_SR_A>;
impl PAD72_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD72_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD72_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD72_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD72_SR`"]
pub struct PAD72_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD72_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD72_SR_A::SR_EN)
    }
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
#[doc = "Reader of field `PAD72_DS1`"]
pub type PAD72_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD72_DS1`"]
pub struct PAD72_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72_DS1_W<'a> {
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
    #[doc = "Bit 12 - Pad 73 slew rate selection."]
    #[inline(always)]
    pub fn pad73_sr(&self) -> PAD73_SR_R {
        PAD73_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 73 high order drive strength selection. Used in conjunction with PAD73STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad73_ds1(&self) -> PAD73_DS1_R {
        PAD73_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 72 slew rate selection."]
    #[inline(always)]
    pub fn pad72_sr(&self) -> PAD72_SR_R {
        PAD72_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 72 high order drive strength selection. Used in conjunction with PAD72STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad72_ds1(&self) -> PAD72_DS1_R {
        PAD72_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Pad 73 slew rate selection."]
    #[inline(always)]
    pub fn pad73_sr(&mut self) -> PAD73_SR_W {
        PAD73_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 73 high order drive strength selection. Used in conjunction with PAD73STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad73_ds1(&mut self) -> PAD73_DS1_W {
        PAD73_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 72 slew rate selection."]
    #[inline(always)]
    pub fn pad72_sr(&mut self) -> PAD72_SR_W {
        PAD72_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 72 high order drive strength selection. Used in conjunction with PAD72STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad72_ds1(&mut self) -> PAD72_DS1_W {
        PAD72_DS1_W { w: self }
    }
}
