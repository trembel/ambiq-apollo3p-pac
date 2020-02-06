#[doc = "Reader of register PADREGP"]
pub type R = crate::R<u32, super::PADREGP>;
#[doc = "Writer for register PADREGP"]
pub type W = crate::W<u32, super::PADREGP>;
#[doc = "Register PADREGP `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 63 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD63FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 63"]
    NCE63 = 1,
    #[doc = "2: CTIMER connection 13"]
    CT13 = 2,
    #[doc = "3: Configure as GPIO63"]
    GPIO63 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD63FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD63FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD63FNCSEL`"]
pub type PAD63FNCSEL_R = crate::R<u8, PAD63FNCSEL_A>;
impl PAD63FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD63FNCSEL_A {
        match self.bits {
            0 => PAD63FNCSEL_A::SWO,
            1 => PAD63FNCSEL_A::NCE63,
            2 => PAD63FNCSEL_A::CT13,
            3 => PAD63FNCSEL_A::GPIO63,
            4 => PAD63FNCSEL_A::UART0TX,
            5 => PAD63FNCSEL_A::UART0RX,
            6 => PAD63FNCSEL_A::UART1TX,
            7 => PAD63FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD63FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE63`"]
    #[inline(always)]
    pub fn is_nce63(&self) -> bool {
        *self == PAD63FNCSEL_A::NCE63
    }
    #[doc = "Checks if the value of the field is `CT13`"]
    #[inline(always)]
    pub fn is_ct13(&self) -> bool {
        *self == PAD63FNCSEL_A::CT13
    }
    #[doc = "Checks if the value of the field is `GPIO63`"]
    #[inline(always)]
    pub fn is_gpio63(&self) -> bool {
        *self == PAD63FNCSEL_A::GPIO63
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD63FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD63FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD63FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD63FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD63FNCSEL`"]
pub struct PAD63FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD63FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 63"]
    #[inline(always)]
    pub fn nce63(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::NCE63)
    }
    #[doc = "CTIMER connection 13"]
    #[inline(always)]
    pub fn ct13(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::CT13)
    }
    #[doc = "Configure as GPIO63"]
    #[inline(always)]
    pub fn gpio63(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::GPIO63)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD63FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 63 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD63STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD63STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD63STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD63STRNG`"]
pub type PAD63STRNG_R = crate::R<bool, PAD63STRNG_A>;
impl PAD63STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD63STRNG_A {
        match self.bits {
            false => PAD63STRNG_A::LOW,
            true => PAD63STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD63STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD63STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD63STRNG`"]
pub struct PAD63STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD63STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD63STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD63STRNG_A::HIGH)
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
#[doc = "Pad 63 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD63INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD63INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD63INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD63INPEN`"]
pub type PAD63INPEN_R = crate::R<bool, PAD63INPEN_A>;
impl PAD63INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD63INPEN_A {
        match self.bits {
            false => PAD63INPEN_A::DIS,
            true => PAD63INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD63INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD63INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD63INPEN`"]
pub struct PAD63INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD63INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD63INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD63INPEN_A::EN)
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
#[doc = "Pad 63 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD63PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD63PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD63PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD63PULL`"]
pub type PAD63PULL_R = crate::R<bool, PAD63PULL_A>;
impl PAD63PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD63PULL_A {
        match self.bits {
            false => PAD63PULL_A::DIS,
            true => PAD63PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD63PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD63PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD63PULL`"]
pub struct PAD63PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD63PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD63PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD63PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD63PULL_A::EN)
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
#[doc = "Pad 62 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD62FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 62"]
    NCE62 = 1,
    #[doc = "2: CTIMER connection 12"]
    CT12 = 2,
    #[doc = "3: Configure as GPIO62"]
    GPIO62 = 3,
    #[doc = "4: Configure as the UART0 CTS input"]
    UA0CTS = 4,
    #[doc = "5: Configure as the UART0 RTS output"]
    UA0RTS = 5,
    #[doc = "6: Configure as the UART1 CTS input"]
    UA1CTS = 6,
    #[doc = "7: Configure as the UART1 RTS output"]
    UA1RTS = 7,
}
impl From<PAD62FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD62FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD62FNCSEL`"]
pub type PAD62FNCSEL_R = crate::R<u8, PAD62FNCSEL_A>;
impl PAD62FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD62FNCSEL_A {
        match self.bits {
            0 => PAD62FNCSEL_A::SWO,
            1 => PAD62FNCSEL_A::NCE62,
            2 => PAD62FNCSEL_A::CT12,
            3 => PAD62FNCSEL_A::GPIO62,
            4 => PAD62FNCSEL_A::UA0CTS,
            5 => PAD62FNCSEL_A::UA0RTS,
            6 => PAD62FNCSEL_A::UA1CTS,
            7 => PAD62FNCSEL_A::UA1RTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD62FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE62`"]
    #[inline(always)]
    pub fn is_nce62(&self) -> bool {
        *self == PAD62FNCSEL_A::NCE62
    }
    #[doc = "Checks if the value of the field is `CT12`"]
    #[inline(always)]
    pub fn is_ct12(&self) -> bool {
        *self == PAD62FNCSEL_A::CT12
    }
    #[doc = "Checks if the value of the field is `GPIO62`"]
    #[inline(always)]
    pub fn is_gpio62(&self) -> bool {
        *self == PAD62FNCSEL_A::GPIO62
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD62FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD62FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD62FNCSEL_A::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD62FNCSEL_A::UA1RTS
    }
}
#[doc = "Write proxy for field `PAD62FNCSEL`"]
pub struct PAD62FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD62FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 62"]
    #[inline(always)]
    pub fn nce62(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::NCE62)
    }
    #[doc = "CTIMER connection 12"]
    #[inline(always)]
    pub fn ct12(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::CT12)
    }
    #[doc = "Configure as GPIO62"]
    #[inline(always)]
    pub fn gpio62(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::GPIO62)
    }
    #[doc = "Configure as the UART0 CTS input"]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the UART0 RTS output"]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the UART1 CTS input"]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::UA1CTS)
    }
    #[doc = "Configure as the UART1 RTS output"]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD62FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 62 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD62STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD62STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD62STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD62STRNG`"]
pub type PAD62STRNG_R = crate::R<bool, PAD62STRNG_A>;
impl PAD62STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD62STRNG_A {
        match self.bits {
            false => PAD62STRNG_A::LOW,
            true => PAD62STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD62STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD62STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD62STRNG`"]
pub struct PAD62STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD62STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD62STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD62STRNG_A::HIGH)
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
#[doc = "Pad 62 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD62INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD62INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD62INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD62INPEN`"]
pub type PAD62INPEN_R = crate::R<bool, PAD62INPEN_A>;
impl PAD62INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD62INPEN_A {
        match self.bits {
            false => PAD62INPEN_A::DIS,
            true => PAD62INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD62INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD62INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD62INPEN`"]
pub struct PAD62INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD62INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD62INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD62INPEN_A::EN)
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
#[doc = "Pad 62 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD62PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD62PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD62PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD62PULL`"]
pub type PAD62PULL_R = crate::R<bool, PAD62PULL_A>;
impl PAD62PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD62PULL_A {
        match self.bits {
            false => PAD62PULL_A::DIS,
            true => PAD62PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD62PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD62PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD62PULL`"]
pub struct PAD62PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD62PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD62PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD62PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD62PULL_A::EN)
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
#[doc = "Pad 61 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD61FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 61"]
    NCE61 = 1,
    #[doc = "2: CTIMER connection 11"]
    CT11 = 2,
    #[doc = "3: Configure as GPIO61"]
    GPIO61 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD61FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD61FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD61FNCSEL`"]
pub type PAD61FNCSEL_R = crate::R<u8, PAD61FNCSEL_A>;
impl PAD61FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD61FNCSEL_A {
        match self.bits {
            0 => PAD61FNCSEL_A::SWO,
            1 => PAD61FNCSEL_A::NCE61,
            2 => PAD61FNCSEL_A::CT11,
            3 => PAD61FNCSEL_A::GPIO61,
            4 => PAD61FNCSEL_A::UART0TX,
            5 => PAD61FNCSEL_A::UART0RX,
            6 => PAD61FNCSEL_A::UART1TX,
            7 => PAD61FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD61FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE61`"]
    #[inline(always)]
    pub fn is_nce61(&self) -> bool {
        *self == PAD61FNCSEL_A::NCE61
    }
    #[doc = "Checks if the value of the field is `CT11`"]
    #[inline(always)]
    pub fn is_ct11(&self) -> bool {
        *self == PAD61FNCSEL_A::CT11
    }
    #[doc = "Checks if the value of the field is `GPIO61`"]
    #[inline(always)]
    pub fn is_gpio61(&self) -> bool {
        *self == PAD61FNCSEL_A::GPIO61
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD61FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD61FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD61FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD61FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD61FNCSEL`"]
pub struct PAD61FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD61FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 61"]
    #[inline(always)]
    pub fn nce61(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::NCE61)
    }
    #[doc = "CTIMER connection 11"]
    #[inline(always)]
    pub fn ct11(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::CT11)
    }
    #[doc = "Configure as GPIO61"]
    #[inline(always)]
    pub fn gpio61(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::GPIO61)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD61FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 61 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD61STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD61STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD61STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD61STRNG`"]
pub type PAD61STRNG_R = crate::R<bool, PAD61STRNG_A>;
impl PAD61STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD61STRNG_A {
        match self.bits {
            false => PAD61STRNG_A::LOW,
            true => PAD61STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD61STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD61STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD61STRNG`"]
pub struct PAD61STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD61STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD61STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD61STRNG_A::HIGH)
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
#[doc = "Pad 61 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD61INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD61INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD61INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD61INPEN`"]
pub type PAD61INPEN_R = crate::R<bool, PAD61INPEN_A>;
impl PAD61INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD61INPEN_A {
        match self.bits {
            false => PAD61INPEN_A::DIS,
            true => PAD61INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD61INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD61INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD61INPEN`"]
pub struct PAD61INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD61INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD61INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD61INPEN_A::EN)
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
#[doc = "Pad 61 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD61PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD61PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD61PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD61PULL`"]
pub type PAD61PULL_R = crate::R<bool, PAD61PULL_A>;
impl PAD61PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD61PULL_A {
        match self.bits {
            false => PAD61PULL_A::DIS,
            true => PAD61PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD61PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD61PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD61PULL`"]
pub struct PAD61PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD61PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD61PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD61PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD61PULL_A::EN)
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
#[doc = "Pad 60 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD60FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 9 signal"]
    MSPI1_9 = 0,
    #[doc = "1: IOM/MSPI nCE group 60"]
    NCE60 = 1,
    #[doc = "2: CTIMER connection 10"]
    CT10 = 2,
    #[doc = "3: Configure as GPIO60"]
    GPIO60 = 3,
}
impl From<PAD60FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD60FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD60FNCSEL`"]
pub type PAD60FNCSEL_R = crate::R<u8, PAD60FNCSEL_A>;
impl PAD60FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD60FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD60FNCSEL_A::MSPI1_9),
            1 => Val(PAD60FNCSEL_A::NCE60),
            2 => Val(PAD60FNCSEL_A::CT10),
            3 => Val(PAD60FNCSEL_A::GPIO60),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_9`"]
    #[inline(always)]
    pub fn is_mspi1_9(&self) -> bool {
        *self == PAD60FNCSEL_A::MSPI1_9
    }
    #[doc = "Checks if the value of the field is `NCE60`"]
    #[inline(always)]
    pub fn is_nce60(&self) -> bool {
        *self == PAD60FNCSEL_A::NCE60
    }
    #[doc = "Checks if the value of the field is `CT10`"]
    #[inline(always)]
    pub fn is_ct10(&self) -> bool {
        *self == PAD60FNCSEL_A::CT10
    }
    #[doc = "Checks if the value of the field is `GPIO60`"]
    #[inline(always)]
    pub fn is_gpio60(&self) -> bool {
        *self == PAD60FNCSEL_A::GPIO60
    }
}
#[doc = "Write proxy for field `PAD60FNCSEL`"]
pub struct PAD60FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD60FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 9 signal"]
    #[inline(always)]
    pub fn mspi1_9(self) -> &'a mut W {
        self.variant(PAD60FNCSEL_A::MSPI1_9)
    }
    #[doc = "IOM/MSPI nCE group 60"]
    #[inline(always)]
    pub fn nce60(self) -> &'a mut W {
        self.variant(PAD60FNCSEL_A::NCE60)
    }
    #[doc = "CTIMER connection 10"]
    #[inline(always)]
    pub fn ct10(self) -> &'a mut W {
        self.variant(PAD60FNCSEL_A::CT10)
    }
    #[doc = "Configure as GPIO60"]
    #[inline(always)]
    pub fn gpio60(self) -> &'a mut W {
        self.variant(PAD60FNCSEL_A::GPIO60)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 60 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD60STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD60STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD60STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD60STRNG`"]
pub type PAD60STRNG_R = crate::R<bool, PAD60STRNG_A>;
impl PAD60STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD60STRNG_A {
        match self.bits {
            false => PAD60STRNG_A::LOW,
            true => PAD60STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD60STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD60STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD60STRNG`"]
pub struct PAD60STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD60STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD60STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD60STRNG_A::HIGH)
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
#[doc = "Pad 60 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD60INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD60INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD60INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD60INPEN`"]
pub type PAD60INPEN_R = crate::R<bool, PAD60INPEN_A>;
impl PAD60INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD60INPEN_A {
        match self.bits {
            false => PAD60INPEN_A::DIS,
            true => PAD60INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD60INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD60INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD60INPEN`"]
pub struct PAD60INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD60INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD60INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD60INPEN_A::EN)
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
#[doc = "Pad 60 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD60PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD60PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD60PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD60PULL`"]
pub type PAD60PULL_R = crate::R<bool, PAD60PULL_A>;
impl PAD60PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD60PULL_A {
        match self.bits {
            false => PAD60PULL_A::DIS,
            true => PAD60PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD60PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD60PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD60PULL`"]
pub struct PAD60PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD60PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD60PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD60PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD60PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 63 function select"]
    #[inline(always)]
    pub fn pad63fncsel(&self) -> PAD63FNCSEL_R {
        PAD63FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 63 drive strength"]
    #[inline(always)]
    pub fn pad63strng(&self) -> PAD63STRNG_R {
        PAD63STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 63 input enable"]
    #[inline(always)]
    pub fn pad63inpen(&self) -> PAD63INPEN_R {
        PAD63INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 63 pullup enable"]
    #[inline(always)]
    pub fn pad63pull(&self) -> PAD63PULL_R {
        PAD63PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 62 function select"]
    #[inline(always)]
    pub fn pad62fncsel(&self) -> PAD62FNCSEL_R {
        PAD62FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 62 drive strength"]
    #[inline(always)]
    pub fn pad62strng(&self) -> PAD62STRNG_R {
        PAD62STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 62 input enable"]
    #[inline(always)]
    pub fn pad62inpen(&self) -> PAD62INPEN_R {
        PAD62INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 62 pullup enable"]
    #[inline(always)]
    pub fn pad62pull(&self) -> PAD62PULL_R {
        PAD62PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 61 function select"]
    #[inline(always)]
    pub fn pad61fncsel(&self) -> PAD61FNCSEL_R {
        PAD61FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 61 drive strength"]
    #[inline(always)]
    pub fn pad61strng(&self) -> PAD61STRNG_R {
        PAD61STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 61 input enable"]
    #[inline(always)]
    pub fn pad61inpen(&self) -> PAD61INPEN_R {
        PAD61INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 61 pullup enable"]
    #[inline(always)]
    pub fn pad61pull(&self) -> PAD61PULL_R {
        PAD61PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 60 function select"]
    #[inline(always)]
    pub fn pad60fncsel(&self) -> PAD60FNCSEL_R {
        PAD60FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 60 drive strength"]
    #[inline(always)]
    pub fn pad60strng(&self) -> PAD60STRNG_R {
        PAD60STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 60 input enable"]
    #[inline(always)]
    pub fn pad60inpen(&self) -> PAD60INPEN_R {
        PAD60INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 60 pullup enable"]
    #[inline(always)]
    pub fn pad60pull(&self) -> PAD60PULL_R {
        PAD60PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 63 function select"]
    #[inline(always)]
    pub fn pad63fncsel(&mut self) -> PAD63FNCSEL_W {
        PAD63FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 63 drive strength"]
    #[inline(always)]
    pub fn pad63strng(&mut self) -> PAD63STRNG_W {
        PAD63STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 63 input enable"]
    #[inline(always)]
    pub fn pad63inpen(&mut self) -> PAD63INPEN_W {
        PAD63INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 63 pullup enable"]
    #[inline(always)]
    pub fn pad63pull(&mut self) -> PAD63PULL_W {
        PAD63PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 62 function select"]
    #[inline(always)]
    pub fn pad62fncsel(&mut self) -> PAD62FNCSEL_W {
        PAD62FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 62 drive strength"]
    #[inline(always)]
    pub fn pad62strng(&mut self) -> PAD62STRNG_W {
        PAD62STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 62 input enable"]
    #[inline(always)]
    pub fn pad62inpen(&mut self) -> PAD62INPEN_W {
        PAD62INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 62 pullup enable"]
    #[inline(always)]
    pub fn pad62pull(&mut self) -> PAD62PULL_W {
        PAD62PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 61 function select"]
    #[inline(always)]
    pub fn pad61fncsel(&mut self) -> PAD61FNCSEL_W {
        PAD61FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 61 drive strength"]
    #[inline(always)]
    pub fn pad61strng(&mut self) -> PAD61STRNG_W {
        PAD61STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 61 input enable"]
    #[inline(always)]
    pub fn pad61inpen(&mut self) -> PAD61INPEN_W {
        PAD61INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 61 pullup enable"]
    #[inline(always)]
    pub fn pad61pull(&mut self) -> PAD61PULL_W {
        PAD61PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 60 function select"]
    #[inline(always)]
    pub fn pad60fncsel(&mut self) -> PAD60FNCSEL_W {
        PAD60FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 60 drive strength"]
    #[inline(always)]
    pub fn pad60strng(&mut self) -> PAD60STRNG_W {
        PAD60STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 60 input enable"]
    #[inline(always)]
    pub fn pad60inpen(&mut self) -> PAD60INPEN_W {
        PAD60INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 60 pullup enable"]
    #[inline(always)]
    pub fn pad60pull(&mut self) -> PAD60PULL_W {
        PAD60PULL_W { w: self }
    }
}
