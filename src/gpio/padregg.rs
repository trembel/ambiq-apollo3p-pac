#[doc = "Reader of register PADREGG"]
pub type R = crate::R<u32, super::PADREGG>;
#[doc = "Writer for register PADREGG"]
pub type W = crate::W<u32, super::PADREGG>;
#[doc = "Register PADREGG `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 27 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD27RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD27RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD27RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD27RSEL`"]
pub type PAD27RSEL_R = crate::R<u8, PAD27RSEL_A>;
impl PAD27RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD27RSEL_A {
        match self.bits {
            0 => PAD27RSEL_A::PULL1_5K,
            1 => PAD27RSEL_A::PULL6K,
            2 => PAD27RSEL_A::PULL12K,
            3 => PAD27RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD27RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD27RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD27RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD27RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD27RSEL`"]
pub struct PAD27RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD27RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD27RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD27RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD27RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Pad 27 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD27FNCSEL_A {
    #[doc = "0: Configure as UART0 RX input signal"]
    UART0RX = 0,
    #[doc = "1: IOM/MSPI nCE group 27"]
    NCE27 = 1,
    #[doc = "2: CTIMER connection 5"]
    CT5 = 2,
    #[doc = "3: Configure as GPIO27"]
    GPIO27 = 3,
    #[doc = "4: Configure as I2C clock I/O signal from IOMSTR2"]
    M2SCL = 4,
    #[doc = "5: Configure as SPI clock output signal from IOMSTR2"]
    M2SCK = 5,
}
impl From<PAD27FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD27FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD27FNCSEL`"]
pub type PAD27FNCSEL_R = crate::R<u8, PAD27FNCSEL_A>;
impl PAD27FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD27FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD27FNCSEL_A::UART0RX),
            1 => Val(PAD27FNCSEL_A::NCE27),
            2 => Val(PAD27FNCSEL_A::CT5),
            3 => Val(PAD27FNCSEL_A::GPIO27),
            4 => Val(PAD27FNCSEL_A::M2SCL),
            5 => Val(PAD27FNCSEL_A::M2SCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD27FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE27`"]
    #[inline(always)]
    pub fn is_nce27(&self) -> bool {
        *self == PAD27FNCSEL_A::NCE27
    }
    #[doc = "Checks if the value of the field is `CT5`"]
    #[inline(always)]
    pub fn is_ct5(&self) -> bool {
        *self == PAD27FNCSEL_A::CT5
    }
    #[doc = "Checks if the value of the field is `GPIO27`"]
    #[inline(always)]
    pub fn is_gpio27(&self) -> bool {
        *self == PAD27FNCSEL_A::GPIO27
    }
    #[doc = "Checks if the value of the field is `M2SCL`"]
    #[inline(always)]
    pub fn is_m2scl(&self) -> bool {
        *self == PAD27FNCSEL_A::M2SCL
    }
    #[doc = "Checks if the value of the field is `M2SCK`"]
    #[inline(always)]
    pub fn is_m2sck(&self) -> bool {
        *self == PAD27FNCSEL_A::M2SCK
    }
}
#[doc = "Write proxy for field `PAD27FNCSEL`"]
pub struct PAD27FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as UART0 RX input signal"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::UART0RX)
    }
    #[doc = "IOM/MSPI nCE group 27"]
    #[inline(always)]
    pub fn nce27(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::NCE27)
    }
    #[doc = "CTIMER connection 5"]
    #[inline(always)]
    pub fn ct5(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::CT5)
    }
    #[doc = "Configure as GPIO27"]
    #[inline(always)]
    pub fn gpio27(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::GPIO27)
    }
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2"]
    #[inline(always)]
    pub fn m2scl(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::M2SCL)
    }
    #[doc = "Configure as SPI clock output signal from IOMSTR2"]
    #[inline(always)]
    pub fn m2sck(self) -> &'a mut W {
        self.variant(PAD27FNCSEL_A::M2SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 27 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD27STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD27STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD27STRNG`"]
pub type PAD27STRNG_R = crate::R<bool, PAD27STRNG_A>;
impl PAD27STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD27STRNG_A {
        match self.bits {
            false => PAD27STRNG_A::LOW,
            true => PAD27STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD27STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD27STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD27STRNG`"]
pub struct PAD27STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD27STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD27STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 27 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD27INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD27INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD27INPEN`"]
pub type PAD27INPEN_R = crate::R<bool, PAD27INPEN_A>;
impl PAD27INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD27INPEN_A {
        match self.bits {
            false => PAD27INPEN_A::DIS,
            true => PAD27INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD27INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD27INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD27INPEN`"]
pub struct PAD27INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD27INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD27INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 27 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD27PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD27PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD27PULL`"]
pub type PAD27PULL_R = crate::R<bool, PAD27PULL_A>;
impl PAD27PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD27PULL_A {
        match self.bits {
            false => PAD27PULL_A::DIS,
            true => PAD27PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD27PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD27PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD27PULL`"]
pub struct PAD27PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD27PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD27PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 26 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD26FNCSEL_A {
    #[doc = "0: Configure as the external HFRC oscillator input"]
    EXTHF = 0,
    #[doc = "1: IOM/MSPI nCE group 26"]
    NCE26 = 1,
    #[doc = "2: CTIMER connection 3"]
    CT3 = 2,
    #[doc = "3: Configure as GPIO26"]
    GPIO26 = 3,
    #[doc = "4: SCARD reset output"]
    SCCRST = 4,
    #[doc = "5: MSPI data connection 1"]
    MSPI1 = 5,
    #[doc = "6: Configure as UART0 TX output signal"]
    UART0TX = 6,
    #[doc = "7: Configure as UART1 CTS input signal"]
    UA1CTS = 7,
}
impl From<PAD26FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD26FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD26FNCSEL`"]
pub type PAD26FNCSEL_R = crate::R<u8, PAD26FNCSEL_A>;
impl PAD26FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD26FNCSEL_A {
        match self.bits {
            0 => PAD26FNCSEL_A::EXTHF,
            1 => PAD26FNCSEL_A::NCE26,
            2 => PAD26FNCSEL_A::CT3,
            3 => PAD26FNCSEL_A::GPIO26,
            4 => PAD26FNCSEL_A::SCCRST,
            5 => PAD26FNCSEL_A::MSPI1,
            6 => PAD26FNCSEL_A::UART0TX,
            7 => PAD26FNCSEL_A::UA1CTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EXTHF`"]
    #[inline(always)]
    pub fn is_exthf(&self) -> bool {
        *self == PAD26FNCSEL_A::EXTHF
    }
    #[doc = "Checks if the value of the field is `NCE26`"]
    #[inline(always)]
    pub fn is_nce26(&self) -> bool {
        *self == PAD26FNCSEL_A::NCE26
    }
    #[doc = "Checks if the value of the field is `CT3`"]
    #[inline(always)]
    pub fn is_ct3(&self) -> bool {
        *self == PAD26FNCSEL_A::CT3
    }
    #[doc = "Checks if the value of the field is `GPIO26`"]
    #[inline(always)]
    pub fn is_gpio26(&self) -> bool {
        *self == PAD26FNCSEL_A::GPIO26
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline(always)]
    pub fn is_sccrst(&self) -> bool {
        *self == PAD26FNCSEL_A::SCCRST
    }
    #[doc = "Checks if the value of the field is `MSPI1`"]
    #[inline(always)]
    pub fn is_mspi1(&self) -> bool {
        *self == PAD26FNCSEL_A::MSPI1
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD26FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD26FNCSEL_A::UA1CTS
    }
}
#[doc = "Write proxy for field `PAD26FNCSEL`"]
pub struct PAD26FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the external HFRC oscillator input"]
    #[inline(always)]
    pub fn exthf(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::EXTHF)
    }
    #[doc = "IOM/MSPI nCE group 26"]
    #[inline(always)]
    pub fn nce26(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::NCE26)
    }
    #[doc = "CTIMER connection 3"]
    #[inline(always)]
    pub fn ct3(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::CT3)
    }
    #[doc = "Configure as GPIO26"]
    #[inline(always)]
    pub fn gpio26(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::GPIO26)
    }
    #[doc = "SCARD reset output"]
    #[inline(always)]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::SCCRST)
    }
    #[doc = "MSPI data connection 1"]
    #[inline(always)]
    pub fn mspi1(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::MSPI1)
    }
    #[doc = "Configure as UART0 TX output signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as UART1 CTS input signal"]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD26FNCSEL_A::UA1CTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 26 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD26STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD26STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD26STRNG`"]
pub type PAD26STRNG_R = crate::R<bool, PAD26STRNG_A>;
impl PAD26STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD26STRNG_A {
        match self.bits {
            false => PAD26STRNG_A::LOW,
            true => PAD26STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD26STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD26STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD26STRNG`"]
pub struct PAD26STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD26STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD26STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 26 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD26INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD26INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD26INPEN`"]
pub type PAD26INPEN_R = crate::R<bool, PAD26INPEN_A>;
impl PAD26INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD26INPEN_A {
        match self.bits {
            false => PAD26INPEN_A::DIS,
            true => PAD26INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD26INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD26INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD26INPEN`"]
pub struct PAD26INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD26INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD26INPEN_A::EN)
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
#[doc = "Pad 26 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD26PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD26PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD26PULL`"]
pub type PAD26PULL_R = crate::R<bool, PAD26PULL_A>;
impl PAD26PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD26PULL_A {
        match self.bits {
            false => PAD26PULL_A::DIS,
            true => PAD26PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD26PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD26PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD26PULL`"]
pub struct PAD26PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD26PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD26PULL_A::EN)
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
#[doc = "Pad 25 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD25RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD25RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD25RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD25RSEL`"]
pub type PAD25RSEL_R = crate::R<u8, PAD25RSEL_A>;
impl PAD25RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD25RSEL_A {
        match self.bits {
            0 => PAD25RSEL_A::PULL1_5K,
            1 => PAD25RSEL_A::PULL6K,
            2 => PAD25RSEL_A::PULL12K,
            3 => PAD25RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD25RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD25RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD25RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD25RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD25RSEL`"]
pub struct PAD25RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD25RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD25RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD25RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD25RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 25 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD25FNCSEL_A {
    #[doc = "0: Configure as UART1 RX input signal"]
    UART1RX = 0,
    #[doc = "1: IOM/MSPI nCE group 25"]
    NCE25 = 1,
    #[doc = "2: CTIMER connection 1"]
    CT1 = 2,
    #[doc = "3: Configure as GPIO25"]
    GPIO25 = 3,
    #[doc = "4: Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal"]
    M2SDAWIR3 = 4,
    #[doc = "5: Configure as the IOMSTR2 SPI MISO input signal"]
    M2MISO = 5,
}
impl From<PAD25FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD25FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD25FNCSEL`"]
pub type PAD25FNCSEL_R = crate::R<u8, PAD25FNCSEL_A>;
impl PAD25FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD25FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD25FNCSEL_A::UART1RX),
            1 => Val(PAD25FNCSEL_A::NCE25),
            2 => Val(PAD25FNCSEL_A::CT1),
            3 => Val(PAD25FNCSEL_A::GPIO25),
            4 => Val(PAD25FNCSEL_A::M2SDAWIR3),
            5 => Val(PAD25FNCSEL_A::M2MISO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD25FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `NCE25`"]
    #[inline(always)]
    pub fn is_nce25(&self) -> bool {
        *self == PAD25FNCSEL_A::NCE25
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline(always)]
    pub fn is_ct1(&self) -> bool {
        *self == PAD25FNCSEL_A::CT1
    }
    #[doc = "Checks if the value of the field is `GPIO25`"]
    #[inline(always)]
    pub fn is_gpio25(&self) -> bool {
        *self == PAD25FNCSEL_A::GPIO25
    }
    #[doc = "Checks if the value of the field is `M2SDAWIR3`"]
    #[inline(always)]
    pub fn is_m2sdawir3(&self) -> bool {
        *self == PAD25FNCSEL_A::M2SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M2MISO`"]
    #[inline(always)]
    pub fn is_m2miso(&self) -> bool {
        *self == PAD25FNCSEL_A::M2MISO
    }
}
#[doc = "Write proxy for field `PAD25FNCSEL`"]
pub struct PAD25FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as UART1 RX input signal"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::UART1RX)
    }
    #[doc = "IOM/MSPI nCE group 25"]
    #[inline(always)]
    pub fn nce25(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::NCE25)
    }
    #[doc = "CTIMER connection 1"]
    #[inline(always)]
    pub fn ct1(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::CT1)
    }
    #[doc = "Configure as GPIO25"]
    #[inline(always)]
    pub fn gpio25(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::GPIO25)
    }
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal"]
    #[inline(always)]
    pub fn m2sdawir3(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::M2SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal"]
    #[inline(always)]
    pub fn m2miso(self) -> &'a mut W {
        self.variant(PAD25FNCSEL_A::M2MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 25 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD25STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD25STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD25STRNG`"]
pub type PAD25STRNG_R = crate::R<bool, PAD25STRNG_A>;
impl PAD25STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD25STRNG_A {
        match self.bits {
            false => PAD25STRNG_A::LOW,
            true => PAD25STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD25STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD25STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD25STRNG`"]
pub struct PAD25STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD25STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD25STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Pad 25 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD25INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD25INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD25INPEN`"]
pub type PAD25INPEN_R = crate::R<bool, PAD25INPEN_A>;
impl PAD25INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD25INPEN_A {
        match self.bits {
            false => PAD25INPEN_A::DIS,
            true => PAD25INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD25INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD25INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD25INPEN`"]
pub struct PAD25INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD25INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD25INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Pad 25 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD25PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD25PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD25PULL`"]
pub type PAD25PULL_R = crate::R<bool, PAD25PULL_A>;
impl PAD25PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD25PULL_A {
        match self.bits {
            false => PAD25PULL_A::DIS,
            true => PAD25PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD25PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD25PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD25PULL`"]
pub struct PAD25PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD25PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD25PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 24 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD24FNCSEL_A {
    #[doc = "0: Configure as UART1 TX output signal"]
    UART1TX = 0,
    #[doc = "1: IOM/MSPI nCE group 24"]
    NCE24 = 1,
    #[doc = "2: MSPI data connection 8"]
    MSPI8 = 2,
    #[doc = "3: Configure as GPIO24"]
    GPIO24 = 3,
    #[doc = "4: Configure as UART0 CTS input signal"]
    UA0CTS = 4,
    #[doc = "5: CTIMER connection 21"]
    CT21 = 5,
    #[doc = "6: Configure as the 32kHz crystal output signal"]
    _32KHZXT = 6,
    #[doc = "7: Configure as the serial trace data output signal"]
    SWO = 7,
}
impl From<PAD24FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD24FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD24FNCSEL`"]
pub type PAD24FNCSEL_R = crate::R<u8, PAD24FNCSEL_A>;
impl PAD24FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD24FNCSEL_A {
        match self.bits {
            0 => PAD24FNCSEL_A::UART1TX,
            1 => PAD24FNCSEL_A::NCE24,
            2 => PAD24FNCSEL_A::MSPI8,
            3 => PAD24FNCSEL_A::GPIO24,
            4 => PAD24FNCSEL_A::UA0CTS,
            5 => PAD24FNCSEL_A::CT21,
            6 => PAD24FNCSEL_A::_32KHZXT,
            7 => PAD24FNCSEL_A::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD24FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `NCE24`"]
    #[inline(always)]
    pub fn is_nce24(&self) -> bool {
        *self == PAD24FNCSEL_A::NCE24
    }
    #[doc = "Checks if the value of the field is `MSPI8`"]
    #[inline(always)]
    pub fn is_mspi8(&self) -> bool {
        *self == PAD24FNCSEL_A::MSPI8
    }
    #[doc = "Checks if the value of the field is `GPIO24`"]
    #[inline(always)]
    pub fn is_gpio24(&self) -> bool {
        *self == PAD24FNCSEL_A::GPIO24
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD24FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `CT21`"]
    #[inline(always)]
    pub fn is_ct21(&self) -> bool {
        *self == PAD24FNCSEL_A::CT21
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline(always)]
    pub fn is_32k_hz_xt(&self) -> bool {
        *self == PAD24FNCSEL_A::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD24FNCSEL_A::SWO
    }
}
#[doc = "Write proxy for field `PAD24FNCSEL`"]
pub struct PAD24FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as UART1 TX output signal"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::UART1TX)
    }
    #[doc = "IOM/MSPI nCE group 24"]
    #[inline(always)]
    pub fn nce24(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::NCE24)
    }
    #[doc = "MSPI data connection 8"]
    #[inline(always)]
    pub fn mspi8(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::MSPI8)
    }
    #[doc = "Configure as GPIO24"]
    #[inline(always)]
    pub fn gpio24(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::GPIO24)
    }
    #[doc = "Configure as UART0 CTS input signal"]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::UA0CTS)
    }
    #[doc = "CTIMER connection 21"]
    #[inline(always)]
    pub fn ct21(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::CT21)
    }
    #[doc = "Configure as the 32kHz crystal output signal"]
    #[inline(always)]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::_32KHZXT)
    }
    #[doc = "Configure as the serial trace data output signal"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD24FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 24 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD24STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD24STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD24STRNG`"]
pub type PAD24STRNG_R = crate::R<bool, PAD24STRNG_A>;
impl PAD24STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD24STRNG_A {
        match self.bits {
            false => PAD24STRNG_A::LOW,
            true => PAD24STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD24STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD24STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD24STRNG`"]
pub struct PAD24STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD24STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD24STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Pad 24 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD24INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD24INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD24INPEN`"]
pub type PAD24INPEN_R = crate::R<bool, PAD24INPEN_A>;
impl PAD24INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD24INPEN_A {
        match self.bits {
            false => PAD24INPEN_A::DIS,
            true => PAD24INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD24INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD24INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD24INPEN`"]
pub struct PAD24INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD24INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD24INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Pad 24 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD24PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD24PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD24PULL`"]
pub type PAD24PULL_R = crate::R<bool, PAD24PULL_A>;
impl PAD24PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD24PULL_A {
        match self.bits {
            false => PAD24PULL_A::DIS,
            true => PAD24PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD24PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD24PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD24PULL`"]
pub struct PAD24PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD24PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD24PULL_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Pad 27 pullup resistor selection."]
    #[inline(always)]
    pub fn pad27rsel(&self) -> PAD27RSEL_R {
        PAD27RSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - Pad 27 function select"]
    #[inline(always)]
    pub fn pad27fncsel(&self) -> PAD27FNCSEL_R {
        PAD27FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 27 drive strength"]
    #[inline(always)]
    pub fn pad27strng(&self) -> PAD27STRNG_R {
        PAD27STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 27 input enable"]
    #[inline(always)]
    pub fn pad27inpen(&self) -> PAD27INPEN_R {
        PAD27INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 27 pullup enable"]
    #[inline(always)]
    pub fn pad27pull(&self) -> PAD27PULL_R {
        PAD27PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 26 function select"]
    #[inline(always)]
    pub fn pad26fncsel(&self) -> PAD26FNCSEL_R {
        PAD26FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 26 drive strength"]
    #[inline(always)]
    pub fn pad26strng(&self) -> PAD26STRNG_R {
        PAD26STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 26 input enable"]
    #[inline(always)]
    pub fn pad26inpen(&self) -> PAD26INPEN_R {
        PAD26INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 26 pullup enable"]
    #[inline(always)]
    pub fn pad26pull(&self) -> PAD26PULL_R {
        PAD26PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 25 pullup resistor selection."]
    #[inline(always)]
    pub fn pad25rsel(&self) -> PAD25RSEL_R {
        PAD25RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 25 function select"]
    #[inline(always)]
    pub fn pad25fncsel(&self) -> PAD25FNCSEL_R {
        PAD25FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 25 drive strength"]
    #[inline(always)]
    pub fn pad25strng(&self) -> PAD25STRNG_R {
        PAD25STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 25 input enable"]
    #[inline(always)]
    pub fn pad25inpen(&self) -> PAD25INPEN_R {
        PAD25INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 25 pullup enable"]
    #[inline(always)]
    pub fn pad25pull(&self) -> PAD25PULL_R {
        PAD25PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 24 function select"]
    #[inline(always)]
    pub fn pad24fncsel(&self) -> PAD24FNCSEL_R {
        PAD24FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 24 drive strength"]
    #[inline(always)]
    pub fn pad24strng(&self) -> PAD24STRNG_R {
        PAD24STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 24 input enable"]
    #[inline(always)]
    pub fn pad24inpen(&self) -> PAD24INPEN_R {
        PAD24INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 24 pullup enable"]
    #[inline(always)]
    pub fn pad24pull(&self) -> PAD24PULL_R {
        PAD24PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Pad 27 pullup resistor selection."]
    #[inline(always)]
    pub fn pad27rsel(&mut self) -> PAD27RSEL_W {
        PAD27RSEL_W { w: self }
    }
    #[doc = "Bits 27:29 - Pad 27 function select"]
    #[inline(always)]
    pub fn pad27fncsel(&mut self) -> PAD27FNCSEL_W {
        PAD27FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 27 drive strength"]
    #[inline(always)]
    pub fn pad27strng(&mut self) -> PAD27STRNG_W {
        PAD27STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 27 input enable"]
    #[inline(always)]
    pub fn pad27inpen(&mut self) -> PAD27INPEN_W {
        PAD27INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 27 pullup enable"]
    #[inline(always)]
    pub fn pad27pull(&mut self) -> PAD27PULL_W {
        PAD27PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 26 function select"]
    #[inline(always)]
    pub fn pad26fncsel(&mut self) -> PAD26FNCSEL_W {
        PAD26FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 26 drive strength"]
    #[inline(always)]
    pub fn pad26strng(&mut self) -> PAD26STRNG_W {
        PAD26STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 26 input enable"]
    #[inline(always)]
    pub fn pad26inpen(&mut self) -> PAD26INPEN_W {
        PAD26INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 26 pullup enable"]
    #[inline(always)]
    pub fn pad26pull(&mut self) -> PAD26PULL_W {
        PAD26PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 25 pullup resistor selection."]
    #[inline(always)]
    pub fn pad25rsel(&mut self) -> PAD25RSEL_W {
        PAD25RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 25 function select"]
    #[inline(always)]
    pub fn pad25fncsel(&mut self) -> PAD25FNCSEL_W {
        PAD25FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 25 drive strength"]
    #[inline(always)]
    pub fn pad25strng(&mut self) -> PAD25STRNG_W {
        PAD25STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 25 input enable"]
    #[inline(always)]
    pub fn pad25inpen(&mut self) -> PAD25INPEN_W {
        PAD25INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 25 pullup enable"]
    #[inline(always)]
    pub fn pad25pull(&mut self) -> PAD25PULL_W {
        PAD25PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 24 function select"]
    #[inline(always)]
    pub fn pad24fncsel(&mut self) -> PAD24FNCSEL_W {
        PAD24FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 24 drive strength"]
    #[inline(always)]
    pub fn pad24strng(&mut self) -> PAD24STRNG_W {
        PAD24STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 24 input enable"]
    #[inline(always)]
    pub fn pad24inpen(&mut self) -> PAD24INPEN_W {
        PAD24INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 24 pullup enable"]
    #[inline(always)]
    pub fn pad24pull(&mut self) -> PAD24PULL_W {
        PAD24PULL_W { w: self }
    }
}
