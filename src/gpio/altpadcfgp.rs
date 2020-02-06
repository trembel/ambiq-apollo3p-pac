#[doc = "Reader of register ALTPADCFGP"]
pub type R = crate::R<u32, super::ALTPADCFGP>;
#[doc = "Writer for register ALTPADCFGP"]
pub type W = crate::W<u32, super::ALTPADCFGP>;
#[doc = "Register ALTPADCFGP `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 63 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD63_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD63_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD63_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD63_SR`"]
pub type PAD63_SR_R = crate::R<bool, PAD63_SR_A>;
impl PAD63_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD63_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD63_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD63_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD63_SR`"]
pub struct PAD63_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD63_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD63_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD63_DS1`"]
pub type PAD63_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD63_DS1`"]
pub struct PAD63_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63_DS1_W<'a> {
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
#[doc = "Pad 63 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD62_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD62_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD62_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD62_SR`"]
pub type PAD62_SR_R = crate::R<bool, PAD62_SR_A>;
impl PAD62_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD62_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD62_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD62_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD62_SR`"]
pub struct PAD62_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD62_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD62_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD62_DS1`"]
pub type PAD62_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD62_DS1`"]
pub struct PAD62_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62_DS1_W<'a> {
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
#[doc = "Pad 63 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD61_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD61_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD61_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD61_SR`"]
pub type PAD61_SR_R = crate::R<bool, PAD61_SR_A>;
impl PAD61_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD61_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD61_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD61_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD61_SR`"]
pub struct PAD61_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD61_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD61_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD61_DS1`"]
pub type PAD61_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD61_DS1`"]
pub struct PAD61_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61_DS1_W<'a> {
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
#[doc = "Pad 63 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD60_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD60_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD60_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD60_SR`"]
pub type PAD60_SR_R = crate::R<bool, PAD60_SR_A>;
impl PAD60_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD60_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD60_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD60_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD60_SR`"]
pub struct PAD60_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD60_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD60_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD60_DS1`"]
pub type PAD60_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD60_DS1`"]
pub struct PAD60_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad63_sr(&self) -> PAD63_SR_R {
        PAD63_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 63 high order drive strength selection. Used in conjunction with PAD63STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad63_ds1(&self) -> PAD63_DS1_R {
        PAD63_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad62_sr(&self) -> PAD62_SR_R {
        PAD62_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 62 high order drive strength selection. Used in conjunction with PAD62STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad62_ds1(&self) -> PAD62_DS1_R {
        PAD62_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad61_sr(&self) -> PAD61_SR_R {
        PAD61_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 61 high order drive strength selection. Used in conjunction with PAD61STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad61_ds1(&self) -> PAD61_DS1_R {
        PAD61_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad60_sr(&self) -> PAD60_SR_R {
        PAD60_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 60 high order drive strength selection. Used in conjunction with PAD60STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad60_ds1(&self) -> PAD60_DS1_R {
        PAD60_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad63_sr(&mut self) -> PAD63_SR_W {
        PAD63_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 63 high order drive strength selection. Used in conjunction with PAD63STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad63_ds1(&mut self) -> PAD63_DS1_W {
        PAD63_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad62_sr(&mut self) -> PAD62_SR_W {
        PAD62_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 62 high order drive strength selection. Used in conjunction with PAD62STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad62_ds1(&mut self) -> PAD62_DS1_W {
        PAD62_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad61_sr(&mut self) -> PAD61_SR_W {
        PAD61_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 61 high order drive strength selection. Used in conjunction with PAD61STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad61_ds1(&mut self) -> PAD61_DS1_W {
        PAD61_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 63 slew rate selection."]
    #[inline(always)]
    pub fn pad60_sr(&mut self) -> PAD60_SR_W {
        PAD60_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 60 high order drive strength selection. Used in conjunction with PAD60STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad60_ds1(&mut self) -> PAD60_DS1_W {
        PAD60_DS1_W { w: self }
    }
}
