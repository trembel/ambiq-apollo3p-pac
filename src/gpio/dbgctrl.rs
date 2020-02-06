#[doc = "Reader of register DBGCTRL"]
pub type R = crate::R<u32, super::DBGCTRL>;
#[doc = "Writer for register DBGCTRL"]
pub type W = crate::W<u32, super::DBGCTRL>;
#[doc = "Register DBGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GCLK5`"]
pub type GCLK5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK5`"]
pub struct GCLK5_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK5_W<'a> {
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
#[doc = "Reader of field `GCLK4`"]
pub type GCLK4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK4`"]
pub struct GCLK4_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK4_W<'a> {
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
#[doc = "Reader of field `GCLK3`"]
pub type GCLK3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK3`"]
pub struct GCLK3_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK3_W<'a> {
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
#[doc = "Reader of field `GCLK2`"]
pub type GCLK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK2`"]
pub struct GCLK2_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK2_W<'a> {
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
#[doc = "Reader of field `GCLK1`"]
pub type GCLK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK1`"]
pub struct GCLK1_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK1_W<'a> {
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
#[doc = "Reader of field `GCLK0`"]
pub type GCLK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLK0`"]
pub struct GCLK0_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK0_W<'a> {
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
    #[doc = "Bit 5 - Gate IOM5 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk5(&self) -> GCLK5_R {
        GCLK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Gate IOM4 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk4(&self) -> GCLK4_R {
        GCLK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Gate IOM3 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk3(&self) -> GCLK3_R {
        GCLK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gate IOM2 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk2(&self) -> GCLK2_R {
        GCLK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate IOM1 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk1(&self) -> GCLK1_R {
        GCLK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gate IOM0 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk0(&self) -> GCLK0_R {
        GCLK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Gate IOM5 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk5(&mut self) -> GCLK5_W {
        GCLK5_W { w: self }
    }
    #[doc = "Bit 4 - Gate IOM4 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk4(&mut self) -> GCLK4_W {
        GCLK4_W { w: self }
    }
    #[doc = "Bit 3 - Gate IOM3 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk3(&mut self) -> GCLK3_W {
        GCLK3_W { w: self }
    }
    #[doc = "Bit 2 - Gate IOM2 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk2(&mut self) -> GCLK2_W {
        GCLK2_W { w: self }
    }
    #[doc = "Bit 1 - Gate IOM1 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk1(&mut self) -> GCLK1_W {
        GCLK1_W { w: self }
    }
    #[doc = "Bit 0 - Gate IOM0 CLK in SPI mode, allowing external input clock"]
    #[inline(always)]
    pub fn gclk0(&mut self) -> GCLK0_W {
        GCLK0_W { w: self }
    }
}
