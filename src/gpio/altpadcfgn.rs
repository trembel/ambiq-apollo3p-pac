#[doc = "Reader of register ALTPADCFGN"]
pub type R = crate::R<u32, super::ALTPADCFGN>;
#[doc = "Writer for register ALTPADCFGN"]
pub type W = crate::W<u32, super::ALTPADCFGN>;
#[doc = "Register ALTPADCFGN `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 55 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD55_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD55_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD55_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD55_SR`"]
pub type PAD55_SR_R = crate::R<bool, PAD55_SR_A>;
impl PAD55_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD55_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD55_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD55_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD55_SR`"]
pub struct PAD55_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD55_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD55_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD55_DS1`"]
pub type PAD55_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD55_DS1`"]
pub struct PAD55_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55_DS1_W<'a> {
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
#[doc = "Pad 55 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD54_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD54_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD54_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD54_SR`"]
pub type PAD54_SR_R = crate::R<bool, PAD54_SR_A>;
impl PAD54_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD54_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD54_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD54_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD54_SR`"]
pub struct PAD54_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD54_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD54_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD54_DS1`"]
pub type PAD54_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD54_DS1`"]
pub struct PAD54_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54_DS1_W<'a> {
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
#[doc = "Pad 55 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD53_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD53_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD53_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD53_SR`"]
pub type PAD53_SR_R = crate::R<bool, PAD53_SR_A>;
impl PAD53_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD53_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD53_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD53_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD53_SR`"]
pub struct PAD53_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD53_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD53_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD53_DS1`"]
pub type PAD53_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD53_DS1`"]
pub struct PAD53_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53_DS1_W<'a> {
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
#[doc = "Pad 55 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD52_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD52_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD52_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD52_SR`"]
pub type PAD52_SR_R = crate::R<bool, PAD52_SR_A>;
impl PAD52_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD52_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD52_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD52_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD52_SR`"]
pub struct PAD52_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD52_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD52_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD52_DS1`"]
pub type PAD52_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD52_DS1`"]
pub struct PAD52_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad55_sr(&self) -> PAD55_SR_R {
        PAD55_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 55 high order drive strength selection. Used in conjunction with PAD55STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad55_ds1(&self) -> PAD55_DS1_R {
        PAD55_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad54_sr(&self) -> PAD54_SR_R {
        PAD54_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 54 high order drive strength selection. Used in conjunction with PAD54STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad54_ds1(&self) -> PAD54_DS1_R {
        PAD54_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad53_sr(&self) -> PAD53_SR_R {
        PAD53_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 53 high order drive strength selection. Used in conjunction with PAD53STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad53_ds1(&self) -> PAD53_DS1_R {
        PAD53_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad52_sr(&self) -> PAD52_SR_R {
        PAD52_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 52 high order drive strength selection. Used in conjunction with PAD52STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad52_ds1(&self) -> PAD52_DS1_R {
        PAD52_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad55_sr(&mut self) -> PAD55_SR_W {
        PAD55_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 55 high order drive strength selection. Used in conjunction with PAD55STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad55_ds1(&mut self) -> PAD55_DS1_W {
        PAD55_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad54_sr(&mut self) -> PAD54_SR_W {
        PAD54_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 54 high order drive strength selection. Used in conjunction with PAD54STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad54_ds1(&mut self) -> PAD54_DS1_W {
        PAD54_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad53_sr(&mut self) -> PAD53_SR_W {
        PAD53_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 53 high order drive strength selection. Used in conjunction with PAD53STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad53_ds1(&mut self) -> PAD53_DS1_W {
        PAD53_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 55 slew rate selection."]
    #[inline(always)]
    pub fn pad52_sr(&mut self) -> PAD52_SR_W {
        PAD52_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 52 high order drive strength selection. Used in conjunction with PAD52STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad52_ds1(&mut self) -> PAD52_DS1_W {
        PAD52_DS1_W { w: self }
    }
}
