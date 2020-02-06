#[doc = "Reader of register ALTPADCFGO"]
pub type R = crate::R<u32, super::ALTPADCFGO>;
#[doc = "Writer for register ALTPADCFGO"]
pub type W = crate::W<u32, super::ALTPADCFGO>;
#[doc = "Register ALTPADCFGO `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 59 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD59_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD59_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD59_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD59_SR`"]
pub type PAD59_SR_R = crate::R<bool, PAD59_SR_A>;
impl PAD59_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD59_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD59_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD59_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD59_SR`"]
pub struct PAD59_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD59_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD59_SR_A::SR_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PAD59_DS1`"]
pub type PAD59_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD59_DS1`"]
pub struct PAD59_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59_DS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 59 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD58_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD58_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD58_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD58_SR`"]
pub type PAD58_SR_R = crate::R<bool, PAD58_SR_A>;
impl PAD58_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD58_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD58_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD58_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD58_SR`"]
pub struct PAD58_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD58_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD58_SR_A::SR_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PAD58_DS1`"]
pub type PAD58_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD58_DS1`"]
pub struct PAD58_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58_DS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Pad 59 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD57_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD57_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD57_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD57_SR`"]
pub type PAD57_SR_R = crate::R<bool, PAD57_SR_A>;
impl PAD57_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD57_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD57_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD57_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD57_SR`"]
pub struct PAD57_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD57_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD57_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD57_DS1`"]
pub type PAD57_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD57_DS1`"]
pub struct PAD57_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57_DS1_W<'a> {
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
#[doc = "Pad 59 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD56_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD56_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD56_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD56_SR`"]
pub type PAD56_SR_R = crate::R<bool, PAD56_SR_A>;
impl PAD56_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD56_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD56_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD56_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD56_SR`"]
pub struct PAD56_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD56_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD56_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD56_DS1`"]
pub type PAD56_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD56_DS1`"]
pub struct PAD56_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad59_sr(&self) -> PAD59_SR_R {
        PAD59_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 59 high order drive strength selection. Used in conjunction with PAD59STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad59_ds1(&self) -> PAD59_DS1_R {
        PAD59_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad58_sr(&self) -> PAD58_SR_R {
        PAD58_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 58 high order drive strength selection. Used in conjunction with PAD58STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad58_ds1(&self) -> PAD58_DS1_R {
        PAD58_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad57_sr(&self) -> PAD57_SR_R {
        PAD57_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 57 high order drive strength selection. Used in conjunction with PAD57STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad57_ds1(&self) -> PAD57_DS1_R {
        PAD57_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad56_sr(&self) -> PAD56_SR_R {
        PAD56_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 56 high order drive strength selection. Used in conjunction with PAD56STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad56_ds1(&self) -> PAD56_DS1_R {
        PAD56_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad59_sr(&mut self) -> PAD59_SR_W {
        PAD59_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 59 high order drive strength selection. Used in conjunction with PAD59STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad59_ds1(&mut self) -> PAD59_DS1_W {
        PAD59_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad58_sr(&mut self) -> PAD58_SR_W {
        PAD58_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 58 high order drive strength selection. Used in conjunction with PAD58STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad58_ds1(&mut self) -> PAD58_DS1_W {
        PAD58_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad57_sr(&mut self) -> PAD57_SR_W {
        PAD57_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 57 high order drive strength selection. Used in conjunction with PAD57STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad57_ds1(&mut self) -> PAD57_DS1_W {
        PAD57_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 59 slew rate selection."]
    #[inline(always)]
    pub fn pad56_sr(&mut self) -> PAD56_SR_W {
        PAD56_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 56 high order drive strength selection. Used in conjunction with PAD56STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad56_ds1(&mut self) -> PAD56_DS1_W {
        PAD56_DS1_W { w: self }
    }
}
