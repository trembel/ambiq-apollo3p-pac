#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `WRITELATENCY`"]
pub type WRITELATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRITELATENCY`"]
pub struct WRITELATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITELATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Serial clock polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Clock inactive state is low."]
    LOW = 0,
    #[doc = "1: Clock inactive state is high."]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::HIGH
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock inactive state is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "Clock inactive state is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Serial clock phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Clock toggles in middle of data bit."]
    MIDDLE = 0,
    #[doc = "1: Clock toggles at start of data bit."]
    START = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::MIDDLE,
            true => CPHA_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `MIDDLE`"]
    #[inline(always)]
    pub fn is_middle(&self) -> bool {
        *self == CPHA_A::MIDDLE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CPHA_A::START
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock toggles in middle of data bit."]
    #[inline(always)]
    pub fn middle(self) -> &'a mut W {
        self.variant(CPHA_A::MIDDLE)
    }
    #[doc = "Clock toggles at start of data bit."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CPHA_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TURNAROUND`"]
pub type TURNAROUND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TURNAROUND`"]
pub struct TURNAROUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TURNAROUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SEPIO`"]
pub type SEPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEPIO`"]
pub struct SEPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPIO_W<'a> {
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
#[doc = "Reader of field `ISIZE`"]
pub type ISIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISIZE`"]
pub struct ISIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISIZE_W<'a> {
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
#[doc = "Address Size. Address bytes to send from ADDR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASIZE_A {
    #[doc = "0: Send one address byte"]
    A1 = 0,
    #[doc = "1: Send two address bytes"]
    A2 = 1,
    #[doc = "2: Send three address bytes"]
    A3 = 2,
    #[doc = "3: Send four address bytes"]
    A4 = 3,
}
impl From<ASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ASIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ASIZE`"]
pub type ASIZE_R = crate::R<u8, ASIZE_A>;
impl ASIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASIZE_A {
        match self.bits {
            0 => ASIZE_A::A1,
            1 => ASIZE_A::A2,
            2 => ASIZE_A::A3,
            3 => ASIZE_A::A4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A1`"]
    #[inline(always)]
    pub fn is_a1(&self) -> bool {
        *self == ASIZE_A::A1
    }
    #[doc = "Checks if the value of the field is `A2`"]
    #[inline(always)]
    pub fn is_a2(&self) -> bool {
        *self == ASIZE_A::A2
    }
    #[doc = "Checks if the value of the field is `A3`"]
    #[inline(always)]
    pub fn is_a3(&self) -> bool {
        *self == ASIZE_A::A3
    }
    #[doc = "Checks if the value of the field is `A4`"]
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == ASIZE_A::A4
    }
}
#[doc = "Write proxy for field `ASIZE`"]
pub struct ASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Send one address byte"]
    #[inline(always)]
    pub fn a1(self) -> &'a mut W {
        self.variant(ASIZE_A::A1)
    }
    #[doc = "Send two address bytes"]
    #[inline(always)]
    pub fn a2(self) -> &'a mut W {
        self.variant(ASIZE_A::A2)
    }
    #[doc = "Send three address bytes"]
    #[inline(always)]
    pub fn a3(self) -> &'a mut W {
        self.variant(ASIZE_A::A3)
    }
    #[doc = "Send four address bytes"]
    #[inline(always)]
    pub fn a4(self) -> &'a mut W {
        self.variant(ASIZE_A::A4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVCFG_A {
    #[doc = "1: Single bit SPI flash on chip select 0"]
    SERIAL0 = 1,
    #[doc = "2: Single bit SPI flash on chip select 1"]
    SERIAL1 = 2,
    #[doc = "5: Dual SPI flash on chip select 0"]
    DUAL0 = 5,
    #[doc = "6: Dual bit SPI flash on chip select 1"]
    DUAL1 = 6,
    #[doc = "9: Quad SPI flash on chip select 0"]
    QUAD0 = 9,
    #[doc = "10: Quad SPI flash on chip select 1"]
    QUAD1 = 10,
    #[doc = "13: Octal SPI flash on chip select 0"]
    OCTAL0 = 13,
    #[doc = "14: Octal SPI flash on chip select 1"]
    OCTAL1 = 14,
}
impl From<DEVCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEVCFG`"]
pub type DEVCFG_R = crate::R<u8, DEVCFG_A>;
impl DEVCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEVCFG_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DEVCFG_A::SERIAL0),
            2 => Val(DEVCFG_A::SERIAL1),
            5 => Val(DEVCFG_A::DUAL0),
            6 => Val(DEVCFG_A::DUAL1),
            9 => Val(DEVCFG_A::QUAD0),
            10 => Val(DEVCFG_A::QUAD1),
            13 => Val(DEVCFG_A::OCTAL0),
            14 => Val(DEVCFG_A::OCTAL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        *self == DEVCFG_A::SERIAL0
    }
    #[doc = "Checks if the value of the field is `SERIAL1`"]
    #[inline(always)]
    pub fn is_serial1(&self) -> bool {
        *self == DEVCFG_A::SERIAL1
    }
    #[doc = "Checks if the value of the field is `DUAL0`"]
    #[inline(always)]
    pub fn is_dual0(&self) -> bool {
        *self == DEVCFG_A::DUAL0
    }
    #[doc = "Checks if the value of the field is `DUAL1`"]
    #[inline(always)]
    pub fn is_dual1(&self) -> bool {
        *self == DEVCFG_A::DUAL1
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        *self == DEVCFG_A::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        *self == DEVCFG_A::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL0`"]
    #[inline(always)]
    pub fn is_octal0(&self) -> bool {
        *self == DEVCFG_A::OCTAL0
    }
    #[doc = "Checks if the value of the field is `OCTAL1`"]
    #[inline(always)]
    pub fn is_octal1(&self) -> bool {
        *self == DEVCFG_A::OCTAL1
    }
}
#[doc = "Write proxy for field `DEVCFG`"]
pub struct DEVCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single bit SPI flash on chip select 0"]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut W {
        self.variant(DEVCFG_A::SERIAL0)
    }
    #[doc = "Single bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn serial1(self) -> &'a mut W {
        self.variant(DEVCFG_A::SERIAL1)
    }
    #[doc = "Dual SPI flash on chip select 0"]
    #[inline(always)]
    pub fn dual0(self) -> &'a mut W {
        self.variant(DEVCFG_A::DUAL0)
    }
    #[doc = "Dual bit SPI flash on chip select 1"]
    #[inline(always)]
    pub fn dual1(self) -> &'a mut W {
        self.variant(DEVCFG_A::DUAL1)
    }
    #[doc = "Quad SPI flash on chip select 0"]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUAD0)
    }
    #[doc = "Quad SPI flash on chip select 1"]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUAD1)
    }
    #[doc = "Octal SPI flash on chip select 0"]
    #[inline(always)]
    pub fn octal0(self) -> &'a mut W {
        self.variant(DEVCFG_A::OCTAL0)
    }
    #[doc = "Octal SPI flash on chip select 1"]
    #[inline(always)]
    pub fn octal1(self) -> &'a mut W {
        self.variant(DEVCFG_A::OCTAL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:25 - Number of cycles between addressn and TX data. Qualified by ENLAT"]
    #[inline(always)]
    pub fn writelatency(&self) -> WRITELATENCY_R {
        WRITELATENCY_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Number of turnaround cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline(always)]
    pub fn turnaround(&self) -> TURNAROUND_R {
        TURNAROUND_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub fn sepio(&self) -> SEPIO_R {
        SEPIO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register"]
    #[inline(always)]
    pub fn asize(&self) -> ASIZE_R {
        ASIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub fn devcfg(&self) -> DEVCFG_R {
        DEVCFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:25 - Number of cycles between addressn and TX data. Qualified by ENLAT"]
    #[inline(always)]
    pub fn writelatency(&mut self) -> WRITELATENCY_W {
        WRITELATENCY_W { w: self }
    }
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bits 8:13 - Number of turnaround cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline(always)]
    pub fn turnaround(&mut self) -> TURNAROUND_W {
        TURNAROUND_W { w: self }
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub fn sepio(&mut self) -> SEPIO_W {
        SEPIO_W { w: self }
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W {
        ISIZE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register"]
    #[inline(always)]
    pub fn asize(&mut self) -> ASIZE_W {
        ASIZE_W { w: self }
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub fn devcfg(&mut self) -> DEVCFG_W {
        DEVCFG_W { w: self }
    }
}
