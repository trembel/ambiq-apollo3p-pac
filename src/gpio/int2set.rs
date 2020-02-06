#[doc = "Reader of register INT2SET"]
pub type R = crate::R<u32, super::INT2SET>;
#[doc = "Writer for register INT2SET"]
pub type W = crate::W<u32, super::INT2SET>;
#[doc = "Register INT2SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INT2SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO73`"]
pub type GPIO73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO73`"]
pub struct GPIO73_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO73_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `GPIO72`"]
pub type GPIO72_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO72`"]
pub struct GPIO72_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO72_W<'a> {
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
#[doc = "Reader of field `GPIO71`"]
pub type GPIO71_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO71`"]
pub struct GPIO71_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO71_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GPIO70`"]
pub type GPIO70_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO70`"]
pub struct GPIO70_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO70_W<'a> {
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
#[doc = "Reader of field `GPIO69`"]
pub type GPIO69_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO69`"]
pub struct GPIO69_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO69_W<'a> {
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
#[doc = "Reader of field `GPIO68`"]
pub type GPIO68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO68`"]
pub struct GPIO68_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO68_W<'a> {
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
#[doc = "Reader of field `GPIO67`"]
pub type GPIO67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO67`"]
pub struct GPIO67_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO67_W<'a> {
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
#[doc = "Reader of field `GPIO66`"]
pub type GPIO66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO66`"]
pub struct GPIO66_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO66_W<'a> {
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
#[doc = "Reader of field `GPIO65`"]
pub type GPIO65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO65`"]
pub struct GPIO65_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO65_W<'a> {
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
#[doc = "Reader of field `GPIO64`"]
pub type GPIO64_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO64`"]
pub struct GPIO64_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO64_W<'a> {
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
    #[doc = "Bit 9 - GPIO73 interrupt."]
    #[inline(always)]
    pub fn gpio73(&self) -> GPIO73_R {
        GPIO73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO72 interrupt."]
    #[inline(always)]
    pub fn gpio72(&self) -> GPIO72_R {
        GPIO72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO71 interrupt."]
    #[inline(always)]
    pub fn gpio71(&self) -> GPIO71_R {
        GPIO71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO70 interrupt."]
    #[inline(always)]
    pub fn gpio70(&self) -> GPIO70_R {
        GPIO70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO69 interrupt."]
    #[inline(always)]
    pub fn gpio69(&self) -> GPIO69_R {
        GPIO69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO68 interrupt."]
    #[inline(always)]
    pub fn gpio68(&self) -> GPIO68_R {
        GPIO68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO67 interrupt."]
    #[inline(always)]
    pub fn gpio67(&self) -> GPIO67_R {
        GPIO67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO66 interrupt."]
    #[inline(always)]
    pub fn gpio66(&self) -> GPIO66_R {
        GPIO66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO65 interrupt."]
    #[inline(always)]
    pub fn gpio65(&self) -> GPIO65_R {
        GPIO65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO64 interrupt."]
    #[inline(always)]
    pub fn gpio64(&self) -> GPIO64_R {
        GPIO64_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - GPIO73 interrupt."]
    #[inline(always)]
    pub fn gpio73(&mut self) -> GPIO73_W {
        GPIO73_W { w: self }
    }
    #[doc = "Bit 8 - GPIO72 interrupt."]
    #[inline(always)]
    pub fn gpio72(&mut self) -> GPIO72_W {
        GPIO72_W { w: self }
    }
    #[doc = "Bit 7 - GPIO71 interrupt."]
    #[inline(always)]
    pub fn gpio71(&mut self) -> GPIO71_W {
        GPIO71_W { w: self }
    }
    #[doc = "Bit 6 - GPIO70 interrupt."]
    #[inline(always)]
    pub fn gpio70(&mut self) -> GPIO70_W {
        GPIO70_W { w: self }
    }
    #[doc = "Bit 5 - GPIO69 interrupt."]
    #[inline(always)]
    pub fn gpio69(&mut self) -> GPIO69_W {
        GPIO69_W { w: self }
    }
    #[doc = "Bit 4 - GPIO68 interrupt."]
    #[inline(always)]
    pub fn gpio68(&mut self) -> GPIO68_W {
        GPIO68_W { w: self }
    }
    #[doc = "Bit 3 - GPIO67 interrupt."]
    #[inline(always)]
    pub fn gpio67(&mut self) -> GPIO67_W {
        GPIO67_W { w: self }
    }
    #[doc = "Bit 2 - GPIO66 interrupt."]
    #[inline(always)]
    pub fn gpio66(&mut self) -> GPIO66_W {
        GPIO66_W { w: self }
    }
    #[doc = "Bit 1 - GPIO65 interrupt."]
    #[inline(always)]
    pub fn gpio65(&mut self) -> GPIO65_W {
        GPIO65_W { w: self }
    }
    #[doc = "Bit 0 - GPIO64 interrupt."]
    #[inline(always)]
    pub fn gpio64(&mut self) -> GPIO64_W {
        GPIO64_W { w: self }
    }
}
