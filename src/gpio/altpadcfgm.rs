#[doc = "Reader of register ALTPADCFGM"]
pub type R = crate::R<u32, super::ALTPADCFGM>;
#[doc = "Writer for register ALTPADCFGM"]
pub type W = crate::W<u32, super::ALTPADCFGM>;
#[doc = "Register ALTPADCFGM `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTPADCFGM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pad 51 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD51_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD51_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD51_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD51_SR`"]
pub type PAD51_SR_R = crate::R<bool, PAD51_SR_A>;
impl PAD51_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD51_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD51_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD51_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD51_SR`"]
pub struct PAD51_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD51_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD51_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD51_DS1`"]
pub type PAD51_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD51_DS1`"]
pub struct PAD51_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51_DS1_W<'a> {
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
#[doc = "Pad 51 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD50_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD50_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD50_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD50_SR`"]
pub type PAD50_SR_R = crate::R<bool, PAD50_SR_A>;
impl PAD50_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD50_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD50_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD50_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD50_SR`"]
pub struct PAD50_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD50_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD50_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD50_DS1`"]
pub type PAD50_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD50_DS1`"]
pub struct PAD50_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50_DS1_W<'a> {
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
#[doc = "Pad 51 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD49_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49_SR`"]
pub type PAD49_SR_R = crate::R<bool, PAD49_SR_A>;
impl PAD49_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD49_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD49_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD49_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD49_SR`"]
pub struct PAD49_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD49_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD49_DS1`"]
pub type PAD49_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD49_DS1`"]
pub struct PAD49_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49_DS1_W<'a> {
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
#[doc = "Pad 51 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48_SR_A {
    #[doc = "1: Enables Slew rate control on pad"]
    SR_EN = 1,
}
impl From<PAD48_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48_SR`"]
pub type PAD48_SR_R = crate::R<bool, PAD48_SR_A>;
impl PAD48_SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PAD48_SR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PAD48_SR_A::SR_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        *self == PAD48_SR_A::SR_EN
    }
}
#[doc = "Write proxy for field `PAD48_SR`"]
pub struct PAD48_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48_SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables Slew rate control on pad"]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD48_SR_A::SR_EN)
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
#[doc = "Reader of field `PAD48_DS1`"]
pub type PAD48_DS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAD48_DS1`"]
pub struct PAD48_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad51_sr(&self) -> PAD51_SR_R {
        PAD51_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 51 high order drive strength selection. Used in conjunction with PAD51STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad51_ds1(&self) -> PAD51_DS1_R {
        PAD51_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad50_sr(&self) -> PAD50_SR_R {
        PAD50_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 50 high order drive strength selection. Used in conjunction with PAD50STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad50_ds1(&self) -> PAD50_DS1_R {
        PAD50_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad49_sr(&self) -> PAD49_SR_R {
        PAD49_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad49_ds1(&self) -> PAD49_DS1_R {
        PAD49_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad48_sr(&self) -> PAD48_SR_R {
        PAD48_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad48_ds1(&self) -> PAD48_DS1_R {
        PAD48_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad51_sr(&mut self) -> PAD51_SR_W {
        PAD51_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 51 high order drive strength selection. Used in conjunction with PAD51STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad51_ds1(&mut self) -> PAD51_DS1_W {
        PAD51_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad50_sr(&mut self) -> PAD50_SR_W {
        PAD50_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 50 high order drive strength selection. Used in conjunction with PAD50STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad50_ds1(&mut self) -> PAD50_DS1_W {
        PAD50_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad49_sr(&mut self) -> PAD49_SR_W {
        PAD49_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad49_ds1(&mut self) -> PAD49_DS1_W {
        PAD49_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 51 slew rate selection."]
    #[inline(always)]
    pub fn pad48_sr(&mut self) -> PAD48_SR_W {
        PAD48_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad48_ds1(&mut self) -> PAD48_DS1_W {
        PAD48_DS1_W { w: self }
    }
}
