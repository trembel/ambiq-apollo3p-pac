#[doc = "Reader of register ALTPADCFGQ"]
pub type R = crate::R<u32, super::ALTPADCFGQ>;
#[doc = "Writer for register ALTPADCFGQ"]
pub type W = crate::W<u32, super::ALTPADCFGQ>;
#[doc = "Register ALTPADCFGQ `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 67 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD67_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD67_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD67_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD67_SR`"]
pub type PAD67_SR_R = crate::R<bool, PAD67_SR_A>;
impl PAD67_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD67_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD67_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD67_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD67_SR`"]
pub struct PAD67_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD67_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD67_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD67_DS1`"]
pub type PAD67_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD67_DS1`"]
pub struct PAD67_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67_DS1_W<'a> {
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
#[doc = "Pad 67 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD66_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD66_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD66_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD66_SR`"]
pub type PAD66_SR_R = crate::R<bool, PAD66_SR_A>;
impl PAD66_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD66_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD66_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD66_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD66_SR`"]
pub struct PAD66_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD66_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD66_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD66_DS1`"]
pub type PAD66_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD66_DS1`"]
pub struct PAD66_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66_DS1_W<'a> {
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
#[doc = "Pad 67 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD65_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD65_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD65_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD65_SR`"]
pub type PAD65_SR_R = crate::R<bool, PAD65_SR_A>;
impl PAD65_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD65_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD65_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD65_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD65_SR`"]
pub struct PAD65_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD65_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD65_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD65_DS1`"]
pub type PAD65_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD65_DS1`"]
pub struct PAD65_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65_DS1_W<'a> {
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
#[doc = "Pad 67 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD64_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD64_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD64_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD64_SR`"]
pub type PAD64_SR_R = crate::R<bool, PAD64_SR_A>;
impl PAD64_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD64_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD64_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD64_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD64_SR`"]
pub struct PAD64_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD64_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD64_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD64_DS1`"]
pub type PAD64_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD64_DS1`"]
pub struct PAD64_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad67_sr(&self) -> PAD67_SR_R {
        PAD67_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 67 high order drive strength selection. Used in conjunction with PAD67STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad67_ds1(&self) -> PAD67_DS1_R {
        PAD67_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad66_sr(&self) -> PAD66_SR_R {
        PAD66_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 66 high order drive strength selection. Used in conjunction with PAD66STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad66_ds1(&self) -> PAD66_DS1_R {
        PAD66_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad65_sr(&self) -> PAD65_SR_R {
        PAD65_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 65 high order drive strength selection. Used in conjunction with PAD65STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad65_ds1(&self) -> PAD65_DS1_R {
        PAD65_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad64_sr(&self) -> PAD64_SR_R {
        PAD64_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 64 high order drive strength selection. Used in conjunction with PAD64STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad64_ds1(&self) -> PAD64_DS1_R {
        PAD64_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad67_sr(&mut self) -> PAD67_SR_W {
        PAD67_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 67 high order drive strength selection. Used in conjunction with PAD67STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad67_ds1(&mut self) -> PAD67_DS1_W {
        PAD67_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad66_sr(&mut self) -> PAD66_SR_W {
        PAD66_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 66 high order drive strength selection. Used in conjunction with PAD66STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad66_ds1(&mut self) -> PAD66_DS1_W {
        PAD66_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad65_sr(&mut self) -> PAD65_SR_W {
        PAD65_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 65 high order drive strength selection. Used in conjunction with PAD65STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad65_ds1(&mut self) -> PAD65_DS1_W {
        PAD65_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 67 slew rate selection."]
    #[inline(always)]
    pub fn pad64_sr(&mut self) -> PAD64_SR_W {
        PAD64_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 64 high order drive strength selection. Used in conjunction with PAD64STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad64_ds1(&mut self) -> PAD64_DS1_W {
        PAD64_DS1_W { w: self }
    }
}
