#[doc = "Reader of register ALTPADCFGR"]
pub type R = crate::R<u32, super::ALTPADCFGR>;
#[doc = "Writer for register ALTPADCFGR"]
pub type W = crate::W<u32, super::ALTPADCFGR>;
#[doc = "Register ALTPADCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 71 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD71_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD71_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD71_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD71_SR`"]
pub type PAD71_SR_R = crate::R<bool, PAD71_SR_A>;
impl PAD71_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD71_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD71_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD71_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD71_SR`"]
pub struct PAD71_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD71_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD71_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD71_DS1`"]
pub type PAD71_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD71_DS1`"]
pub struct PAD71_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71_DS1_W<'a> {
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
#[doc = "Pad 71 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD70_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD70_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD70_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD70_SR`"]
pub type PAD70_SR_R = crate::R<bool, PAD70_SR_A>;
impl PAD70_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD70_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD70_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD70_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD70_SR`"]
pub struct PAD70_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD70_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD70_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD70_DS1`"]
pub type PAD70_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD70_DS1`"]
pub struct PAD70_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70_DS1_W<'a> {
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
#[doc = "Pad 71 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD69_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD69_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD69_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD69_SR`"]
pub type PAD69_SR_R = crate::R<bool, PAD69_SR_A>;
impl PAD69_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD69_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD69_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD69_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD69_SR`"]
pub struct PAD69_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD69_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD69_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD69_DS1`"]
pub type PAD69_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD69_DS1`"]
pub struct PAD69_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69_DS1_W<'a> {
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
#[doc = "Pad 71 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD68_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD68_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD68_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD68_SR`"]
pub type PAD68_SR_R = crate::R<bool, PAD68_SR_A>;
impl PAD68_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD68_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD68_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD68_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD68_SR`"]
pub struct PAD68_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD68_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD68_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD68_DS1`"]
pub type PAD68_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD68_DS1`"]
pub struct PAD68_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad71_sr(&self) -> PAD71_SR_R {
        PAD71_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 71 high order drive strength selection. Used in conjunction with PAD71STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad71_ds1(&self) -> PAD71_DS1_R {
        PAD71_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad70_sr(&self) -> PAD70_SR_R {
        PAD70_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 70 high order drive strength selection. Used in conjunction with PAD70STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad70_ds1(&self) -> PAD70_DS1_R {
        PAD70_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad69_sr(&self) -> PAD69_SR_R {
        PAD69_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 69 high order drive strength selection. Used in conjunction with PAD69STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad69_ds1(&self) -> PAD69_DS1_R {
        PAD69_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad68_sr(&self) -> PAD68_SR_R {
        PAD68_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 68 high order drive strength selection. Used in conjunction with PAD68STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad68_ds1(&self) -> PAD68_DS1_R {
        PAD68_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad71_sr(&mut self) -> PAD71_SR_W {
        PAD71_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 71 high order drive strength selection. Used in conjunction with PAD71STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad71_ds1(&mut self) -> PAD71_DS1_W {
        PAD71_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad70_sr(&mut self) -> PAD70_SR_W {
        PAD70_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 70 high order drive strength selection. Used in conjunction with PAD70STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad70_ds1(&mut self) -> PAD70_DS1_W {
        PAD70_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad69_sr(&mut self) -> PAD69_SR_W {
        PAD69_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 69 high order drive strength selection. Used in conjunction with PAD69STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad69_ds1(&mut self) -> PAD69_DS1_W {
        PAD69_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 71 slew rate selection."]
    #[inline(always)]
    pub fn pad68_sr(&mut self) -> PAD68_SR_W {
        PAD68_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 68 high order drive strength selection. Used in conjunction with PAD68STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad68_ds1(&mut self) -> PAD68_DS1_W {
        PAD68_DS1_W { w: self }
    }
}
