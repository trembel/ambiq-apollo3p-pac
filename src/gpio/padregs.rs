#[doc = "Reader of register PADREGS"]
pub type R = crate::R<u32, super::PADREGS>;
#[doc = "Writer for register PADREGS"]
pub type W = crate::W<u32, super::PADREGS>;
#[doc = "Register PADREGS `reset()`'s with value 0x1818"]
impl crate::ResetValue for super::PADREGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818
    }
}
#[doc = "Pad 73 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD73FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 73"]
    NCE73 = 1,
    #[doc = "2: CTIMER connection 23"]
    CT23 = 2,
    #[doc = "3: Configure as GPIO73"]
    GPIO73 = 3,
    #[doc = "4: Configure as the UART0 CTS input"]
    UA0CTS = 4,
    #[doc = "5: Configure as the UART0 RTS output"]
    UA0RTS = 5,
    #[doc = "6: Configure as the UART1 CTS input"]
    UA1CTS = 6,
    #[doc = "7: Configure as the UART1 RTS output"]
    UA1RTS = 7,
}
impl From<PAD73FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD73FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD73FNCSEL`"]
pub type PAD73FNCSEL_R = crate::R<u8, PAD73FNCSEL_A>;
impl PAD73FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD73FNCSEL_A {
        match self.bits {
            0 => PAD73FNCSEL_A::SWO,
            1 => PAD73FNCSEL_A::NCE73,
            2 => PAD73FNCSEL_A::CT23,
            3 => PAD73FNCSEL_A::GPIO73,
            4 => PAD73FNCSEL_A::UA0CTS,
            5 => PAD73FNCSEL_A::UA0RTS,
            6 => PAD73FNCSEL_A::UA1CTS,
            7 => PAD73FNCSEL_A::UA1RTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD73FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE73`"]
    #[inline(always)]
    pub fn is_nce73(&self) -> bool {
        *self == PAD73FNCSEL_A::NCE73
    }
    #[doc = "Checks if the value of the field is `CT23`"]
    #[inline(always)]
    pub fn is_ct23(&self) -> bool {
        *self == PAD73FNCSEL_A::CT23
    }
    #[doc = "Checks if the value of the field is `GPIO73`"]
    #[inline(always)]
    pub fn is_gpio73(&self) -> bool {
        *self == PAD73FNCSEL_A::GPIO73
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        *self == PAD73FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        *self == PAD73FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        *self == PAD73FNCSEL_A::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        *self == PAD73FNCSEL_A::UA1RTS
    }
}
#[doc = "Write proxy for field `PAD73FNCSEL`"]
pub struct PAD73FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD73FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 73"]
    #[inline(always)]
    pub fn nce73(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::NCE73)
    }
    #[doc = "CTIMER connection 23"]
    #[inline(always)]
    pub fn ct23(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::CT23)
    }
    #[doc = "Configure as GPIO73"]
    #[inline(always)]
    pub fn gpio73(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::GPIO73)
    }
    #[doc = "Configure as the UART0 CTS input"]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the UART0 RTS output"]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the UART1 CTS input"]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::UA1CTS)
    }
    #[doc = "Configure as the UART1 RTS output"]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD73FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 73 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD73STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD73STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD73STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD73STRNG`"]
pub type PAD73STRNG_R = crate::R<bool, PAD73STRNG_A>;
impl PAD73STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD73STRNG_A {
        match self.bits {
            false => PAD73STRNG_A::LOW,
            true => PAD73STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD73STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD73STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD73STRNG`"]
pub struct PAD73STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD73STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD73STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD73STRNG_A::HIGH)
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
#[doc = "Pad 73 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD73INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD73INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD73INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD73INPEN`"]
pub type PAD73INPEN_R = crate::R<bool, PAD73INPEN_A>;
impl PAD73INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD73INPEN_A {
        match self.bits {
            false => PAD73INPEN_A::DIS,
            true => PAD73INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD73INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD73INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD73INPEN`"]
pub struct PAD73INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD73INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD73INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD73INPEN_A::EN)
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
#[doc = "Pad 73 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD73PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD73PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD73PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD73PULL`"]
pub type PAD73PULL_R = crate::R<bool, PAD73PULL_A>;
impl PAD73PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD73PULL_A {
        match self.bits {
            false => PAD73PULL_A::DIS,
            true => PAD73PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD73PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD73PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD73PULL`"]
pub struct PAD73PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD73PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD73PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD73PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD73PULL_A::EN)
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
#[doc = "Pad 72 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD72FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 72"]
    NCE72 = 1,
    #[doc = "2: CTIMER connection 22"]
    CT22 = 2,
    #[doc = "3: Configure as GPIO72"]
    GPIO72 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD72FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD72FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD72FNCSEL`"]
pub type PAD72FNCSEL_R = crate::R<u8, PAD72FNCSEL_A>;
impl PAD72FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD72FNCSEL_A {
        match self.bits {
            0 => PAD72FNCSEL_A::SWO,
            1 => PAD72FNCSEL_A::NCE72,
            2 => PAD72FNCSEL_A::CT22,
            3 => PAD72FNCSEL_A::GPIO72,
            4 => PAD72FNCSEL_A::UART0TX,
            5 => PAD72FNCSEL_A::UART0RX,
            6 => PAD72FNCSEL_A::UART1TX,
            7 => PAD72FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD72FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE72`"]
    #[inline(always)]
    pub fn is_nce72(&self) -> bool {
        *self == PAD72FNCSEL_A::NCE72
    }
    #[doc = "Checks if the value of the field is `CT22`"]
    #[inline(always)]
    pub fn is_ct22(&self) -> bool {
        *self == PAD72FNCSEL_A::CT22
    }
    #[doc = "Checks if the value of the field is `GPIO72`"]
    #[inline(always)]
    pub fn is_gpio72(&self) -> bool {
        *self == PAD72FNCSEL_A::GPIO72
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD72FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD72FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD72FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD72FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD72FNCSEL`"]
pub struct PAD72FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD72FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 72"]
    #[inline(always)]
    pub fn nce72(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::NCE72)
    }
    #[doc = "CTIMER connection 22"]
    #[inline(always)]
    pub fn ct22(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::CT22)
    }
    #[doc = "Configure as GPIO72"]
    #[inline(always)]
    pub fn gpio72(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::GPIO72)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD72FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 72 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD72STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD72STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD72STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD72STRNG`"]
pub type PAD72STRNG_R = crate::R<bool, PAD72STRNG_A>;
impl PAD72STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD72STRNG_A {
        match self.bits {
            false => PAD72STRNG_A::LOW,
            true => PAD72STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD72STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD72STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD72STRNG`"]
pub struct PAD72STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD72STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD72STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD72STRNG_A::HIGH)
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
#[doc = "Pad 72 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD72INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD72INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD72INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD72INPEN`"]
pub type PAD72INPEN_R = crate::R<bool, PAD72INPEN_A>;
impl PAD72INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD72INPEN_A {
        match self.bits {
            false => PAD72INPEN_A::DIS,
            true => PAD72INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD72INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD72INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD72INPEN`"]
pub struct PAD72INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD72INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD72INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD72INPEN_A::EN)
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
#[doc = "Pad 72 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD72PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD72PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD72PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD72PULL`"]
pub type PAD72PULL_R = crate::R<bool, PAD72PULL_A>;
impl PAD72PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD72PULL_A {
        match self.bits {
            false => PAD72PULL_A::DIS,
            true => PAD72PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD72PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD72PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD72PULL`"]
pub struct PAD72PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD72PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD72PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD72PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD72PULL_A::EN)
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
    #[doc = "Bits 11:13 - Pad 73 function select"]
    #[inline(always)]
    pub fn pad73fncsel(&self) -> PAD73FNCSEL_R {
        PAD73FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 73 drive strength"]
    #[inline(always)]
    pub fn pad73strng(&self) -> PAD73STRNG_R {
        PAD73STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 73 input enable"]
    #[inline(always)]
    pub fn pad73inpen(&self) -> PAD73INPEN_R {
        PAD73INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 73 pullup enable"]
    #[inline(always)]
    pub fn pad73pull(&self) -> PAD73PULL_R {
        PAD73PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 72 function select"]
    #[inline(always)]
    pub fn pad72fncsel(&self) -> PAD72FNCSEL_R {
        PAD72FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 72 drive strength"]
    #[inline(always)]
    pub fn pad72strng(&self) -> PAD72STRNG_R {
        PAD72STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 72 input enable"]
    #[inline(always)]
    pub fn pad72inpen(&self) -> PAD72INPEN_R {
        PAD72INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 72 pullup enable"]
    #[inline(always)]
    pub fn pad72pull(&self) -> PAD72PULL_R {
        PAD72PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:13 - Pad 73 function select"]
    #[inline(always)]
    pub fn pad73fncsel(&mut self) -> PAD73FNCSEL_W {
        PAD73FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 73 drive strength"]
    #[inline(always)]
    pub fn pad73strng(&mut self) -> PAD73STRNG_W {
        PAD73STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 73 input enable"]
    #[inline(always)]
    pub fn pad73inpen(&mut self) -> PAD73INPEN_W {
        PAD73INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 73 pullup enable"]
    #[inline(always)]
    pub fn pad73pull(&mut self) -> PAD73PULL_W {
        PAD73PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 72 function select"]
    #[inline(always)]
    pub fn pad72fncsel(&mut self) -> PAD72FNCSEL_W {
        PAD72FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 72 drive strength"]
    #[inline(always)]
    pub fn pad72strng(&mut self) -> PAD72STRNG_W {
        PAD72STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 72 input enable"]
    #[inline(always)]
    pub fn pad72inpen(&mut self) -> PAD72INPEN_W {
        PAD72INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 72 pullup enable"]
    #[inline(always)]
    pub fn pad72pull(&mut self) -> PAD72PULL_W {
        PAD72PULL_W { w: self }
    }
}
