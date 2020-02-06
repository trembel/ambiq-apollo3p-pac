#[doc = "Reader of register PADREGR"]
pub type R = crate::R<u32, super::PADREGR>;
#[doc = "Writer for register PADREGR"]
pub type W = crate::W<u32, super::PADREGR>;
#[doc = "Register PADREGR `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 71 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD71FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 71"]
    NCE71 = 1,
    #[doc = "2: CTIMER connection 21"]
    CT21 = 2,
    #[doc = "3: Configure as GPIO71"]
    GPIO71 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD71FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD71FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD71FNCSEL`"]
pub type PAD71FNCSEL_R = crate::R<u8, PAD71FNCSEL_A>;
impl PAD71FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD71FNCSEL_A {
        match self.bits {
            0 => PAD71FNCSEL_A::SWO,
            1 => PAD71FNCSEL_A::NCE71,
            2 => PAD71FNCSEL_A::CT21,
            3 => PAD71FNCSEL_A::GPIO71,
            4 => PAD71FNCSEL_A::UART0TX,
            5 => PAD71FNCSEL_A::UART0RX,
            6 => PAD71FNCSEL_A::UART1TX,
            7 => PAD71FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD71FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE71`"]
    #[inline(always)]
    pub fn is_nce71(&self) -> bool {
        *self == PAD71FNCSEL_A::NCE71
    }
    #[doc = "Checks if the value of the field is `CT21`"]
    #[inline(always)]
    pub fn is_ct21(&self) -> bool {
        *self == PAD71FNCSEL_A::CT21
    }
    #[doc = "Checks if the value of the field is `GPIO71`"]
    #[inline(always)]
    pub fn is_gpio71(&self) -> bool {
        *self == PAD71FNCSEL_A::GPIO71
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD71FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD71FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD71FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD71FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD71FNCSEL`"]
pub struct PAD71FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD71FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 71"]
    #[inline(always)]
    pub fn nce71(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::NCE71)
    }
    #[doc = "CTIMER connection 21"]
    #[inline(always)]
    pub fn ct21(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::CT21)
    }
    #[doc = "Configure as GPIO71"]
    #[inline(always)]
    pub fn gpio71(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::GPIO71)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD71FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 71 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD71STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD71STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD71STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD71STRNG`"]
pub type PAD71STRNG_R = crate::R<bool, PAD71STRNG_A>;
impl PAD71STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD71STRNG_A {
        match self.bits {
            false => PAD71STRNG_A::LOW,
            true => PAD71STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD71STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD71STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD71STRNG`"]
pub struct PAD71STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD71STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD71STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD71STRNG_A::HIGH)
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
#[doc = "Pad 71 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD71INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD71INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD71INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD71INPEN`"]
pub type PAD71INPEN_R = crate::R<bool, PAD71INPEN_A>;
impl PAD71INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD71INPEN_A {
        match self.bits {
            false => PAD71INPEN_A::DIS,
            true => PAD71INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD71INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD71INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD71INPEN`"]
pub struct PAD71INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD71INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD71INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD71INPEN_A::EN)
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
#[doc = "Pad 71 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD71PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD71PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD71PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD71PULL`"]
pub type PAD71PULL_R = crate::R<bool, PAD71PULL_A>;
impl PAD71PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD71PULL_A {
        match self.bits {
            false => PAD71PULL_A::DIS,
            true => PAD71PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD71PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD71PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD71PULL`"]
pub struct PAD71PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD71PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD71PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD71PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD71PULL_A::EN)
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
#[doc = "Pad 70 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD70FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 70"]
    NCE70 = 1,
    #[doc = "2: CTIMER connection 20"]
    CT20 = 2,
    #[doc = "3: Configure as GPIO70"]
    GPIO70 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD70FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD70FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD70FNCSEL`"]
pub type PAD70FNCSEL_R = crate::R<u8, PAD70FNCSEL_A>;
impl PAD70FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD70FNCSEL_A {
        match self.bits {
            0 => PAD70FNCSEL_A::SWO,
            1 => PAD70FNCSEL_A::NCE70,
            2 => PAD70FNCSEL_A::CT20,
            3 => PAD70FNCSEL_A::GPIO70,
            4 => PAD70FNCSEL_A::UART0TX,
            5 => PAD70FNCSEL_A::UART0RX,
            6 => PAD70FNCSEL_A::UART1TX,
            7 => PAD70FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD70FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE70`"]
    #[inline(always)]
    pub fn is_nce70(&self) -> bool {
        *self == PAD70FNCSEL_A::NCE70
    }
    #[doc = "Checks if the value of the field is `CT20`"]
    #[inline(always)]
    pub fn is_ct20(&self) -> bool {
        *self == PAD70FNCSEL_A::CT20
    }
    #[doc = "Checks if the value of the field is `GPIO70`"]
    #[inline(always)]
    pub fn is_gpio70(&self) -> bool {
        *self == PAD70FNCSEL_A::GPIO70
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD70FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD70FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD70FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD70FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD70FNCSEL`"]
pub struct PAD70FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD70FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 70"]
    #[inline(always)]
    pub fn nce70(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::NCE70)
    }
    #[doc = "CTIMER connection 20"]
    #[inline(always)]
    pub fn ct20(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::CT20)
    }
    #[doc = "Configure as GPIO70"]
    #[inline(always)]
    pub fn gpio70(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::GPIO70)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD70FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 70 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD70STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD70STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD70STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD70STRNG`"]
pub type PAD70STRNG_R = crate::R<bool, PAD70STRNG_A>;
impl PAD70STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD70STRNG_A {
        match self.bits {
            false => PAD70STRNG_A::LOW,
            true => PAD70STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD70STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD70STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD70STRNG`"]
pub struct PAD70STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD70STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD70STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD70STRNG_A::HIGH)
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
#[doc = "Pad 70 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD70INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD70INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD70INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD70INPEN`"]
pub type PAD70INPEN_R = crate::R<bool, PAD70INPEN_A>;
impl PAD70INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD70INPEN_A {
        match self.bits {
            false => PAD70INPEN_A::DIS,
            true => PAD70INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD70INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD70INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD70INPEN`"]
pub struct PAD70INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD70INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD70INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD70INPEN_A::EN)
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
#[doc = "Pad 70 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD70PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD70PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD70PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD70PULL`"]
pub type PAD70PULL_R = crate::R<bool, PAD70PULL_A>;
impl PAD70PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD70PULL_A {
        match self.bits {
            false => PAD70PULL_A::DIS,
            true => PAD70PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD70PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD70PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD70PULL`"]
pub struct PAD70PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD70PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD70PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD70PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD70PULL_A::EN)
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
#[doc = "Pad 69 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD69FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 69"]
    NCE69 = 1,
    #[doc = "2: CTIMER connection 19"]
    CT19 = 2,
    #[doc = "3: Configure as GPIO69"]
    GPIO69 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD69FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD69FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD69FNCSEL`"]
pub type PAD69FNCSEL_R = crate::R<u8, PAD69FNCSEL_A>;
impl PAD69FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD69FNCSEL_A {
        match self.bits {
            0 => PAD69FNCSEL_A::SWO,
            1 => PAD69FNCSEL_A::NCE69,
            2 => PAD69FNCSEL_A::CT19,
            3 => PAD69FNCSEL_A::GPIO69,
            4 => PAD69FNCSEL_A::UART0TX,
            5 => PAD69FNCSEL_A::UART0RX,
            6 => PAD69FNCSEL_A::UART1TX,
            7 => PAD69FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD69FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE69`"]
    #[inline(always)]
    pub fn is_nce69(&self) -> bool {
        *self == PAD69FNCSEL_A::NCE69
    }
    #[doc = "Checks if the value of the field is `CT19`"]
    #[inline(always)]
    pub fn is_ct19(&self) -> bool {
        *self == PAD69FNCSEL_A::CT19
    }
    #[doc = "Checks if the value of the field is `GPIO69`"]
    #[inline(always)]
    pub fn is_gpio69(&self) -> bool {
        *self == PAD69FNCSEL_A::GPIO69
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD69FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD69FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD69FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD69FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD69FNCSEL`"]
pub struct PAD69FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD69FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 69"]
    #[inline(always)]
    pub fn nce69(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::NCE69)
    }
    #[doc = "CTIMER connection 19"]
    #[inline(always)]
    pub fn ct19(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::CT19)
    }
    #[doc = "Configure as GPIO69"]
    #[inline(always)]
    pub fn gpio69(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::GPIO69)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD69FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 69 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD69STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD69STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD69STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD69STRNG`"]
pub type PAD69STRNG_R = crate::R<bool, PAD69STRNG_A>;
impl PAD69STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD69STRNG_A {
        match self.bits {
            false => PAD69STRNG_A::LOW,
            true => PAD69STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD69STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD69STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD69STRNG`"]
pub struct PAD69STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD69STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD69STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD69STRNG_A::HIGH)
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
#[doc = "Pad 69 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD69INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD69INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD69INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD69INPEN`"]
pub type PAD69INPEN_R = crate::R<bool, PAD69INPEN_A>;
impl PAD69INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD69INPEN_A {
        match self.bits {
            false => PAD69INPEN_A::DIS,
            true => PAD69INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD69INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD69INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD69INPEN`"]
pub struct PAD69INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD69INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD69INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD69INPEN_A::EN)
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
#[doc = "Pad 69 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD69PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD69PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD69PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD69PULL`"]
pub type PAD69PULL_R = crate::R<bool, PAD69PULL_A>;
impl PAD69PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD69PULL_A {
        match self.bits {
            false => PAD69PULL_A::DIS,
            true => PAD69PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD69PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD69PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD69PULL`"]
pub struct PAD69PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD69PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD69PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD69PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD69PULL_A::EN)
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
#[doc = "Pad 68 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD68FNCSEL_A {
    #[doc = "0: Configure as the MSPI2 4 signal"]
    MSPI2_4 = 0,
    #[doc = "1: IOM/MSPI nCE group 68"]
    NCE68 = 1,
    #[doc = "2: CTIMER connection 18"]
    CT18 = 2,
    #[doc = "3: Configure as GPIO68"]
    GPIO68 = 3,
}
impl From<PAD68FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD68FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD68FNCSEL`"]
pub type PAD68FNCSEL_R = crate::R<u8, PAD68FNCSEL_A>;
impl PAD68FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD68FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD68FNCSEL_A::MSPI2_4),
            1 => Val(PAD68FNCSEL_A::NCE68),
            2 => Val(PAD68FNCSEL_A::CT18),
            3 => Val(PAD68FNCSEL_A::GPIO68),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI2_4`"]
    #[inline(always)]
    pub fn is_mspi2_4(&self) -> bool {
        *self == PAD68FNCSEL_A::MSPI2_4
    }
    #[doc = "Checks if the value of the field is `NCE68`"]
    #[inline(always)]
    pub fn is_nce68(&self) -> bool {
        *self == PAD68FNCSEL_A::NCE68
    }
    #[doc = "Checks if the value of the field is `CT18`"]
    #[inline(always)]
    pub fn is_ct18(&self) -> bool {
        *self == PAD68FNCSEL_A::CT18
    }
    #[doc = "Checks if the value of the field is `GPIO68`"]
    #[inline(always)]
    pub fn is_gpio68(&self) -> bool {
        *self == PAD68FNCSEL_A::GPIO68
    }
}
#[doc = "Write proxy for field `PAD68FNCSEL`"]
pub struct PAD68FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD68FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI2 4 signal"]
    #[inline(always)]
    pub fn mspi2_4(self) -> &'a mut W {
        self.variant(PAD68FNCSEL_A::MSPI2_4)
    }
    #[doc = "IOM/MSPI nCE group 68"]
    #[inline(always)]
    pub fn nce68(self) -> &'a mut W {
        self.variant(PAD68FNCSEL_A::NCE68)
    }
    #[doc = "CTIMER connection 18"]
    #[inline(always)]
    pub fn ct18(self) -> &'a mut W {
        self.variant(PAD68FNCSEL_A::CT18)
    }
    #[doc = "Configure as GPIO68"]
    #[inline(always)]
    pub fn gpio68(self) -> &'a mut W {
        self.variant(PAD68FNCSEL_A::GPIO68)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 68 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD68STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD68STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD68STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD68STRNG`"]
pub type PAD68STRNG_R = crate::R<bool, PAD68STRNG_A>;
impl PAD68STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD68STRNG_A {
        match self.bits {
            false => PAD68STRNG_A::LOW,
            true => PAD68STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD68STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD68STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD68STRNG`"]
pub struct PAD68STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD68STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD68STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD68STRNG_A::HIGH)
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
#[doc = "Pad 68 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD68INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD68INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD68INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD68INPEN`"]
pub type PAD68INPEN_R = crate::R<bool, PAD68INPEN_A>;
impl PAD68INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD68INPEN_A {
        match self.bits {
            false => PAD68INPEN_A::DIS,
            true => PAD68INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD68INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD68INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD68INPEN`"]
pub struct PAD68INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD68INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD68INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD68INPEN_A::EN)
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
#[doc = "Pad 68 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD68PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD68PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD68PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD68PULL`"]
pub type PAD68PULL_R = crate::R<bool, PAD68PULL_A>;
impl PAD68PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD68PULL_A {
        match self.bits {
            false => PAD68PULL_A::DIS,
            true => PAD68PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD68PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD68PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD68PULL`"]
pub struct PAD68PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD68PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD68PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD68PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD68PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 71 function select"]
    #[inline(always)]
    pub fn pad71fncsel(&self) -> PAD71FNCSEL_R {
        PAD71FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 71 drive strength"]
    #[inline(always)]
    pub fn pad71strng(&self) -> PAD71STRNG_R {
        PAD71STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 71 input enable"]
    #[inline(always)]
    pub fn pad71inpen(&self) -> PAD71INPEN_R {
        PAD71INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 71 pullup enable"]
    #[inline(always)]
    pub fn pad71pull(&self) -> PAD71PULL_R {
        PAD71PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 70 function select"]
    #[inline(always)]
    pub fn pad70fncsel(&self) -> PAD70FNCSEL_R {
        PAD70FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 70 drive strength"]
    #[inline(always)]
    pub fn pad70strng(&self) -> PAD70STRNG_R {
        PAD70STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 70 input enable"]
    #[inline(always)]
    pub fn pad70inpen(&self) -> PAD70INPEN_R {
        PAD70INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 70 pullup enable"]
    #[inline(always)]
    pub fn pad70pull(&self) -> PAD70PULL_R {
        PAD70PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 69 function select"]
    #[inline(always)]
    pub fn pad69fncsel(&self) -> PAD69FNCSEL_R {
        PAD69FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 69 drive strength"]
    #[inline(always)]
    pub fn pad69strng(&self) -> PAD69STRNG_R {
        PAD69STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 69 input enable"]
    #[inline(always)]
    pub fn pad69inpen(&self) -> PAD69INPEN_R {
        PAD69INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 69 pullup enable"]
    #[inline(always)]
    pub fn pad69pull(&self) -> PAD69PULL_R {
        PAD69PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 68 function select"]
    #[inline(always)]
    pub fn pad68fncsel(&self) -> PAD68FNCSEL_R {
        PAD68FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 68 drive strength"]
    #[inline(always)]
    pub fn pad68strng(&self) -> PAD68STRNG_R {
        PAD68STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 68 input enable"]
    #[inline(always)]
    pub fn pad68inpen(&self) -> PAD68INPEN_R {
        PAD68INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 68 pullup enable"]
    #[inline(always)]
    pub fn pad68pull(&self) -> PAD68PULL_R {
        PAD68PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 71 function select"]
    #[inline(always)]
    pub fn pad71fncsel(&mut self) -> PAD71FNCSEL_W {
        PAD71FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 71 drive strength"]
    #[inline(always)]
    pub fn pad71strng(&mut self) -> PAD71STRNG_W {
        PAD71STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 71 input enable"]
    #[inline(always)]
    pub fn pad71inpen(&mut self) -> PAD71INPEN_W {
        PAD71INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 71 pullup enable"]
    #[inline(always)]
    pub fn pad71pull(&mut self) -> PAD71PULL_W {
        PAD71PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 70 function select"]
    #[inline(always)]
    pub fn pad70fncsel(&mut self) -> PAD70FNCSEL_W {
        PAD70FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 70 drive strength"]
    #[inline(always)]
    pub fn pad70strng(&mut self) -> PAD70STRNG_W {
        PAD70STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 70 input enable"]
    #[inline(always)]
    pub fn pad70inpen(&mut self) -> PAD70INPEN_W {
        PAD70INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 70 pullup enable"]
    #[inline(always)]
    pub fn pad70pull(&mut self) -> PAD70PULL_W {
        PAD70PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 69 function select"]
    #[inline(always)]
    pub fn pad69fncsel(&mut self) -> PAD69FNCSEL_W {
        PAD69FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 69 drive strength"]
    #[inline(always)]
    pub fn pad69strng(&mut self) -> PAD69STRNG_W {
        PAD69STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 69 input enable"]
    #[inline(always)]
    pub fn pad69inpen(&mut self) -> PAD69INPEN_W {
        PAD69INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 69 pullup enable"]
    #[inline(always)]
    pub fn pad69pull(&mut self) -> PAD69PULL_W {
        PAD69PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 68 function select"]
    #[inline(always)]
    pub fn pad68fncsel(&mut self) -> PAD68FNCSEL_W {
        PAD68FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 68 drive strength"]
    #[inline(always)]
    pub fn pad68strng(&mut self) -> PAD68STRNG_W {
        PAD68STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 68 input enable"]
    #[inline(always)]
    pub fn pad68inpen(&mut self) -> PAD68INPEN_W {
        PAD68INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 68 pullup enable"]
    #[inline(always)]
    pub fn pad68pull(&mut self) -> PAD68PULL_W {
        PAD68PULL_W { w: self }
    }
}
