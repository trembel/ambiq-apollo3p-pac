#[doc = "Reader of register PADREGM"]
pub type R = crate::R<u32, super::PADREGM>;
#[doc = "Writer for register PADREGM"]
pub type W = crate::W<u32, super::PADREGM>;
#[doc = "Register PADREGM `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 51 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD51FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 0 signal"]
    MSPI1_0 = 0,
    #[doc = "1: IOM/MSPI nCE group 51"]
    NCE51 = 1,
    #[doc = "2: CTIMER connection 1"]
    CT1 = 2,
    #[doc = "3: Configure as GPIO51"]
    GPIO51 = 3,
}
impl From<PAD51FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD51FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD51FNCSEL`"]
pub type PAD51FNCSEL_R = crate::R<u8, PAD51FNCSEL_A>;
impl PAD51FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD51FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD51FNCSEL_A::MSPI1_0),
            1 => Val(PAD51FNCSEL_A::NCE51),
            2 => Val(PAD51FNCSEL_A::CT1),
            3 => Val(PAD51FNCSEL_A::GPIO51),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_0`"]
    #[inline(always)]
    pub fn is_mspi1_0(&self) -> bool {
        *self == PAD51FNCSEL_A::MSPI1_0
    }
    #[doc = "Checks if the value of the field is `NCE51`"]
    #[inline(always)]
    pub fn is_nce51(&self) -> bool {
        *self == PAD51FNCSEL_A::NCE51
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline(always)]
    pub fn is_ct1(&self) -> bool {
        *self == PAD51FNCSEL_A::CT1
    }
    #[doc = "Checks if the value of the field is `GPIO51`"]
    #[inline(always)]
    pub fn is_gpio51(&self) -> bool {
        *self == PAD51FNCSEL_A::GPIO51
    }
}
#[doc = "Write proxy for field `PAD51FNCSEL`"]
pub struct PAD51FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD51FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 0 signal"]
    #[inline(always)]
    pub fn mspi1_0(self) -> &'a mut W {
        self.variant(PAD51FNCSEL_A::MSPI1_0)
    }
    #[doc = "IOM/MSPI nCE group 51"]
    #[inline(always)]
    pub fn nce51(self) -> &'a mut W {
        self.variant(PAD51FNCSEL_A::NCE51)
    }
    #[doc = "CTIMER connection 1"]
    #[inline(always)]
    pub fn ct1(self) -> &'a mut W {
        self.variant(PAD51FNCSEL_A::CT1)
    }
    #[doc = "Configure as GPIO51"]
    #[inline(always)]
    pub fn gpio51(self) -> &'a mut W {
        self.variant(PAD51FNCSEL_A::GPIO51)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 51 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD51STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD51STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD51STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD51STRNG`"]
pub type PAD51STRNG_R = crate::R<bool, PAD51STRNG_A>;
impl PAD51STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD51STRNG_A {
        match self.bits {
            false => PAD51STRNG_A::LOW,
            true => PAD51STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD51STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD51STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD51STRNG`"]
pub struct PAD51STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD51STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD51STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD51STRNG_A::HIGH)
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
#[doc = "Pad 51 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD51INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD51INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD51INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD51INPEN`"]
pub type PAD51INPEN_R = crate::R<bool, PAD51INPEN_A>;
impl PAD51INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD51INPEN_A {
        match self.bits {
            false => PAD51INPEN_A::DIS,
            true => PAD51INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD51INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD51INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD51INPEN`"]
pub struct PAD51INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD51INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD51INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD51INPEN_A::EN)
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
#[doc = "Pad 51 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD51PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD51PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD51PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD51PULL`"]
pub type PAD51PULL_R = crate::R<bool, PAD51PULL_A>;
impl PAD51PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD51PULL_A {
        match self.bits {
            false => PAD51PULL_A::DIS,
            true => PAD51PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD51PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD51PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD51PULL`"]
pub struct PAD51PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD51PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD51PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD51PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD51PULL_A::EN)
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
#[doc = "Pad 50 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD50FNCSEL_A {
    #[doc = "0: Configure as the SWO output"]
    SWO = 0,
    #[doc = "1: IOM/MSPI nCE group 50"]
    NCE50 = 1,
    #[doc = "2: CTIMER connection 0"]
    CT0 = 2,
    #[doc = "3: Configure as GPIO50"]
    GPIO50 = 3,
    #[doc = "4: Configure as the UART0 TX output"]
    UART0TX = 4,
    #[doc = "5: Configure as the UART0 RX input"]
    UART0RX = 5,
    #[doc = "6: Configure as the UART1 TX output"]
    UART1TX = 6,
    #[doc = "7: Configure as the UART1 RX input"]
    UART1RX = 7,
}
impl From<PAD50FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD50FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD50FNCSEL`"]
pub type PAD50FNCSEL_R = crate::R<u8, PAD50FNCSEL_A>;
impl PAD50FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD50FNCSEL_A {
        match self.bits {
            0 => PAD50FNCSEL_A::SWO,
            1 => PAD50FNCSEL_A::NCE50,
            2 => PAD50FNCSEL_A::CT0,
            3 => PAD50FNCSEL_A::GPIO50,
            4 => PAD50FNCSEL_A::UART0TX,
            5 => PAD50FNCSEL_A::UART0RX,
            6 => PAD50FNCSEL_A::UART1TX,
            7 => PAD50FNCSEL_A::UART1RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        *self == PAD50FNCSEL_A::SWO
    }
    #[doc = "Checks if the value of the field is `NCE50`"]
    #[inline(always)]
    pub fn is_nce50(&self) -> bool {
        *self == PAD50FNCSEL_A::NCE50
    }
    #[doc = "Checks if the value of the field is `CT0`"]
    #[inline(always)]
    pub fn is_ct0(&self) -> bool {
        *self == PAD50FNCSEL_A::CT0
    }
    #[doc = "Checks if the value of the field is `GPIO50`"]
    #[inline(always)]
    pub fn is_gpio50(&self) -> bool {
        *self == PAD50FNCSEL_A::GPIO50
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD50FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD50FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        *self == PAD50FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        *self == PAD50FNCSEL_A::UART1RX
    }
}
#[doc = "Write proxy for field `PAD50FNCSEL`"]
pub struct PAD50FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD50FNCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Configure as the SWO output"]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::SWO)
    }
    #[doc = "IOM/MSPI nCE group 50"]
    #[inline(always)]
    pub fn nce50(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::NCE50)
    }
    #[doc = "CTIMER connection 0"]
    #[inline(always)]
    pub fn ct0(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::CT0)
    }
    #[doc = "Configure as GPIO50"]
    #[inline(always)]
    pub fn gpio50(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::GPIO50)
    }
    #[doc = "Configure as the UART0 TX output"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as the UART0 RX input"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the UART1 TX output"]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the UART1 RX input"]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD50FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 50 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD50STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD50STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD50STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD50STRNG`"]
pub type PAD50STRNG_R = crate::R<bool, PAD50STRNG_A>;
impl PAD50STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD50STRNG_A {
        match self.bits {
            false => PAD50STRNG_A::LOW,
            true => PAD50STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD50STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD50STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD50STRNG`"]
pub struct PAD50STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD50STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD50STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD50STRNG_A::HIGH)
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
#[doc = "Pad 50 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD50INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD50INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD50INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD50INPEN`"]
pub type PAD50INPEN_R = crate::R<bool, PAD50INPEN_A>;
impl PAD50INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD50INPEN_A {
        match self.bits {
            false => PAD50INPEN_A::DIS,
            true => PAD50INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD50INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD50INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD50INPEN`"]
pub struct PAD50INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD50INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD50INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD50INPEN_A::EN)
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
#[doc = "Pad 50 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD50PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD50PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD50PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD50PULL`"]
pub type PAD50PULL_R = crate::R<bool, PAD50PULL_A>;
impl PAD50PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD50PULL_A {
        match self.bits {
            false => PAD50PULL_A::DIS,
            true => PAD50PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD50PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD50PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD50PULL`"]
pub struct PAD50PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD50PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD50PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD50PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD50PULL_A::EN)
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
#[doc = "Pad 49 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD49RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD49RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD49RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD49RSEL`"]
pub type PAD49RSEL_R = crate::R<u8, PAD49RSEL_A>;
impl PAD49RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49RSEL_A {
        match self.bits {
            0 => PAD49RSEL_A::PULL1_5K,
            1 => PAD49RSEL_A::PULL6K,
            2 => PAD49RSEL_A::PULL12K,
            3 => PAD49RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD49RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD49RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD49RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD49RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD49RSEL`"]
pub struct PAD49RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD49RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD49RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD49RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD49RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 49 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD49FNCSEL_A {
    #[doc = "0: Configure as the UART0 RX input signal"]
    UART0RX = 0,
    #[doc = "1: IOM/MSPPI nCE group 49"]
    NCE49 = 1,
    #[doc = "2: CTIMER connection 30"]
    CT30 = 2,
    #[doc = "3: Configure as GPIO49"]
    GPIO49 = 3,
    #[doc = "4: Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal"]
    M5SDAWIR3 = 4,
    #[doc = "5: Configure as the IOMSTR5 SPI MISO input signal"]
    M5MISO = 5,
}
impl From<PAD49FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD49FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD49FNCSEL`"]
pub type PAD49FNCSEL_R = crate::R<u8, PAD49FNCSEL_A>;
impl PAD49FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD49FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD49FNCSEL_A::UART0RX),
            1 => Val(PAD49FNCSEL_A::NCE49),
            2 => Val(PAD49FNCSEL_A::CT30),
            3 => Val(PAD49FNCSEL_A::GPIO49),
            4 => Val(PAD49FNCSEL_A::M5SDAWIR3),
            5 => Val(PAD49FNCSEL_A::M5MISO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        *self == PAD49FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `NCE49`"]
    #[inline(always)]
    pub fn is_nce49(&self) -> bool {
        *self == PAD49FNCSEL_A::NCE49
    }
    #[doc = "Checks if the value of the field is `CT30`"]
    #[inline(always)]
    pub fn is_ct30(&self) -> bool {
        *self == PAD49FNCSEL_A::CT30
    }
    #[doc = "Checks if the value of the field is `GPIO49`"]
    #[inline(always)]
    pub fn is_gpio49(&self) -> bool {
        *self == PAD49FNCSEL_A::GPIO49
    }
    #[doc = "Checks if the value of the field is `M5SDAWIR3`"]
    #[inline(always)]
    pub fn is_m5sdawir3(&self) -> bool {
        *self == PAD49FNCSEL_A::M5SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M5MISO`"]
    #[inline(always)]
    pub fn is_m5miso(&self) -> bool {
        *self == PAD49FNCSEL_A::M5MISO
    }
}
#[doc = "Write proxy for field `PAD49FNCSEL`"]
pub struct PAD49FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART0 RX input signal"]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::UART0RX)
    }
    #[doc = "IOM/MSPPI nCE group 49"]
    #[inline(always)]
    pub fn nce49(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::NCE49)
    }
    #[doc = "CTIMER connection 30"]
    #[inline(always)]
    pub fn ct30(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::CT30)
    }
    #[doc = "Configure as GPIO49"]
    #[inline(always)]
    pub fn gpio49(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::GPIO49)
    }
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal"]
    #[inline(always)]
    pub fn m5sdawir3(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::M5SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal"]
    #[inline(always)]
    pub fn m5miso(self) -> &'a mut W {
        self.variant(PAD49FNCSEL_A::M5MISO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 49 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD49STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49STRNG`"]
pub type PAD49STRNG_R = crate::R<bool, PAD49STRNG_A>;
impl PAD49STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49STRNG_A {
        match self.bits {
            false => PAD49STRNG_A::LOW,
            true => PAD49STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD49STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD49STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD49STRNG`"]
pub struct PAD49STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD49STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD49STRNG_A::HIGH)
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
#[doc = "Pad 49 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD49INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49INPEN`"]
pub type PAD49INPEN_R = crate::R<bool, PAD49INPEN_A>;
impl PAD49INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49INPEN_A {
        match self.bits {
            false => PAD49INPEN_A::DIS,
            true => PAD49INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD49INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD49INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD49INPEN`"]
pub struct PAD49INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49INPEN_A::EN)
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
#[doc = "Pad 49 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD49PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD49PULL`"]
pub type PAD49PULL_R = crate::R<bool, PAD49PULL_A>;
impl PAD49PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD49PULL_A {
        match self.bits {
            false => PAD49PULL_A::DIS,
            true => PAD49PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD49PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD49PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD49PULL`"]
pub struct PAD49PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD49PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD49PULL_A::EN)
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
#[doc = "Pad 48 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD48RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms"]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms"]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms"]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms"]
    PULL24K = 3,
}
impl From<PAD48RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD48RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD48RSEL`"]
pub type PAD48RSEL_R = crate::R<u8, PAD48RSEL_A>;
impl PAD48RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48RSEL_A {
        match self.bits {
            0 => PAD48RSEL_A::PULL1_5K,
            1 => PAD48RSEL_A::PULL6K,
            2 => PAD48RSEL_A::PULL12K,
            3 => PAD48RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        *self == PAD48RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        *self == PAD48RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        *self == PAD48RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        *self == PAD48RSEL_A::PULL24K
    }
}
#[doc = "Write proxy for field `PAD48RSEL`"]
pub struct PAD48RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pullup is ~1.5 KOhms"]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD48RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms"]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD48RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms"]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD48RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms"]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD48RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 48 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD48FNCSEL_A {
    #[doc = "0: Configure as the UART0 TX output signal"]
    UART0TX = 0,
    #[doc = "1: IOM/MSPI nCE group 48"]
    NCE48 = 1,
    #[doc = "2: CTIMER connection 28"]
    CT28 = 2,
    #[doc = "3: Configure as GPIO48"]
    GPIO48 = 3,
    #[doc = "4: Configure as the IOMSTR5 I2C SCL clock I/O signal"]
    M5SCL = 4,
    #[doc = "5: Configure as the IOMSTR5 SPI SCK output"]
    M5SCK = 5,
}
impl From<PAD48FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD48FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD48FNCSEL`"]
pub type PAD48FNCSEL_R = crate::R<u8, PAD48FNCSEL_A>;
impl PAD48FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD48FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD48FNCSEL_A::UART0TX),
            1 => Val(PAD48FNCSEL_A::NCE48),
            2 => Val(PAD48FNCSEL_A::CT28),
            3 => Val(PAD48FNCSEL_A::GPIO48),
            4 => Val(PAD48FNCSEL_A::M5SCL),
            5 => Val(PAD48FNCSEL_A::M5SCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        *self == PAD48FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `NCE48`"]
    #[inline(always)]
    pub fn is_nce48(&self) -> bool {
        *self == PAD48FNCSEL_A::NCE48
    }
    #[doc = "Checks if the value of the field is `CT28`"]
    #[inline(always)]
    pub fn is_ct28(&self) -> bool {
        *self == PAD48FNCSEL_A::CT28
    }
    #[doc = "Checks if the value of the field is `GPIO48`"]
    #[inline(always)]
    pub fn is_gpio48(&self) -> bool {
        *self == PAD48FNCSEL_A::GPIO48
    }
    #[doc = "Checks if the value of the field is `M5SCL`"]
    #[inline(always)]
    pub fn is_m5scl(&self) -> bool {
        *self == PAD48FNCSEL_A::M5SCL
    }
    #[doc = "Checks if the value of the field is `M5SCK`"]
    #[inline(always)]
    pub fn is_m5sck(&self) -> bool {
        *self == PAD48FNCSEL_A::M5SCK
    }
}
#[doc = "Write proxy for field `PAD48FNCSEL`"]
pub struct PAD48FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART0 TX output signal"]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::UART0TX)
    }
    #[doc = "IOM/MSPI nCE group 48"]
    #[inline(always)]
    pub fn nce48(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::NCE48)
    }
    #[doc = "CTIMER connection 28"]
    #[inline(always)]
    pub fn ct28(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::CT28)
    }
    #[doc = "Configure as GPIO48"]
    #[inline(always)]
    pub fn gpio48(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::GPIO48)
    }
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal"]
    #[inline(always)]
    pub fn m5scl(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::M5SCL)
    }
    #[doc = "Configure as the IOMSTR5 SPI SCK output"]
    #[inline(always)]
    pub fn m5sck(self) -> &'a mut W {
        self.variant(PAD48FNCSEL_A::M5SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 48 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD48STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48STRNG`"]
pub type PAD48STRNG_R = crate::R<bool, PAD48STRNG_A>;
impl PAD48STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48STRNG_A {
        match self.bits {
            false => PAD48STRNG_A::LOW,
            true => PAD48STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD48STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD48STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD48STRNG`"]
pub struct PAD48STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD48STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD48STRNG_A::HIGH)
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
#[doc = "Pad 48 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD48INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48INPEN`"]
pub type PAD48INPEN_R = crate::R<bool, PAD48INPEN_A>;
impl PAD48INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48INPEN_A {
        match self.bits {
            false => PAD48INPEN_A::DIS,
            true => PAD48INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD48INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD48INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD48INPEN`"]
pub struct PAD48INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48INPEN_A::EN)
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
#[doc = "Pad 48 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD48PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD48PULL`"]
pub type PAD48PULL_R = crate::R<bool, PAD48PULL_A>;
impl PAD48PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD48PULL_A {
        match self.bits {
            false => PAD48PULL_A::DIS,
            true => PAD48PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD48PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD48PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD48PULL`"]
pub struct PAD48PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD48PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD48PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 51 function select"]
    #[inline(always)]
    pub fn pad51fncsel(&self) -> PAD51FNCSEL_R {
        PAD51FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 51 drive strength"]
    #[inline(always)]
    pub fn pad51strng(&self) -> PAD51STRNG_R {
        PAD51STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 51 input enable"]
    #[inline(always)]
    pub fn pad51inpen(&self) -> PAD51INPEN_R {
        PAD51INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 51 pullup enable"]
    #[inline(always)]
    pub fn pad51pull(&self) -> PAD51PULL_R {
        PAD51PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 50 function select"]
    #[inline(always)]
    pub fn pad50fncsel(&self) -> PAD50FNCSEL_R {
        PAD50FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 50 drive strength"]
    #[inline(always)]
    pub fn pad50strng(&self) -> PAD50STRNG_R {
        PAD50STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 50 input enable"]
    #[inline(always)]
    pub fn pad50inpen(&self) -> PAD50INPEN_R {
        PAD50INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 50 pullup enable"]
    #[inline(always)]
    pub fn pad50pull(&self) -> PAD50PULL_R {
        PAD50PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 49 pullup resistor selection."]
    #[inline(always)]
    pub fn pad49rsel(&self) -> PAD49RSEL_R {
        PAD49RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 49 function select"]
    #[inline(always)]
    pub fn pad49fncsel(&self) -> PAD49FNCSEL_R {
        PAD49FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline(always)]
    pub fn pad49strng(&self) -> PAD49STRNG_R {
        PAD49STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline(always)]
    pub fn pad49inpen(&self) -> PAD49INPEN_R {
        PAD49INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline(always)]
    pub fn pad49pull(&self) -> PAD49PULL_R {
        PAD49PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 48 pullup resistor selection."]
    #[inline(always)]
    pub fn pad48rsel(&self) -> PAD48RSEL_R {
        PAD48RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 48 function select"]
    #[inline(always)]
    pub fn pad48fncsel(&self) -> PAD48FNCSEL_R {
        PAD48FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline(always)]
    pub fn pad48strng(&self) -> PAD48STRNG_R {
        PAD48STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline(always)]
    pub fn pad48inpen(&self) -> PAD48INPEN_R {
        PAD48INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline(always)]
    pub fn pad48pull(&self) -> PAD48PULL_R {
        PAD48PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 51 function select"]
    #[inline(always)]
    pub fn pad51fncsel(&mut self) -> PAD51FNCSEL_W {
        PAD51FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 51 drive strength"]
    #[inline(always)]
    pub fn pad51strng(&mut self) -> PAD51STRNG_W {
        PAD51STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 51 input enable"]
    #[inline(always)]
    pub fn pad51inpen(&mut self) -> PAD51INPEN_W {
        PAD51INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 51 pullup enable"]
    #[inline(always)]
    pub fn pad51pull(&mut self) -> PAD51PULL_W {
        PAD51PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 50 function select"]
    #[inline(always)]
    pub fn pad50fncsel(&mut self) -> PAD50FNCSEL_W {
        PAD50FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 50 drive strength"]
    #[inline(always)]
    pub fn pad50strng(&mut self) -> PAD50STRNG_W {
        PAD50STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 50 input enable"]
    #[inline(always)]
    pub fn pad50inpen(&mut self) -> PAD50INPEN_W {
        PAD50INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 50 pullup enable"]
    #[inline(always)]
    pub fn pad50pull(&mut self) -> PAD50PULL_W {
        PAD50PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 49 pullup resistor selection."]
    #[inline(always)]
    pub fn pad49rsel(&mut self) -> PAD49RSEL_W {
        PAD49RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 49 function select"]
    #[inline(always)]
    pub fn pad49fncsel(&mut self) -> PAD49FNCSEL_W {
        PAD49FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 49 drive strength"]
    #[inline(always)]
    pub fn pad49strng(&mut self) -> PAD49STRNG_W {
        PAD49STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 49 input enable"]
    #[inline(always)]
    pub fn pad49inpen(&mut self) -> PAD49INPEN_W {
        PAD49INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 49 pullup enable"]
    #[inline(always)]
    pub fn pad49pull(&mut self) -> PAD49PULL_W {
        PAD49PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 48 pullup resistor selection."]
    #[inline(always)]
    pub fn pad48rsel(&mut self) -> PAD48RSEL_W {
        PAD48RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 48 function select"]
    #[inline(always)]
    pub fn pad48fncsel(&mut self) -> PAD48FNCSEL_W {
        PAD48FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 48 drive strength"]
    #[inline(always)]
    pub fn pad48strng(&mut self) -> PAD48STRNG_W {
        PAD48STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 48 input enable"]
    #[inline(always)]
    pub fn pad48inpen(&mut self) -> PAD48INPEN_W {
        PAD48INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 48 pullup enable"]
    #[inline(always)]
    pub fn pad48pull(&mut self) -> PAD48PULL_W {
        PAD48PULL_W { w: self }
    }
}
