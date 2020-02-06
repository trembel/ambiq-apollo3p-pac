#[doc = "Reader of register PADREGO"]
pub type R = crate::R<u32, super::PADREGO>;
#[doc = "Writer for register PADREGO"]
pub type W = crate::W<u32, super::PADREGO>;
#[doc = "Register PADREGO `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 59 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD59FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 8 signal"]
    MSPI1_8 = 0,
    #[doc = "1: IOM/MSPI nCE group 59"]
    NCE59 = 1,
    #[doc = "2: CTIMER connection 9"]
    CT9 = 2,
    #[doc = "3: Configure as GPIO59"]
    GPIO59 = 3,
}
impl From<PAD59FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD59FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD59FNCSEL`"]
pub type PAD59FNCSEL_R = crate::R<u8, PAD59FNCSEL_A>;
impl PAD59FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD59FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD59FNCSEL_A::MSPI1_8),
            1 => Val(PAD59FNCSEL_A::NCE59),
            2 => Val(PAD59FNCSEL_A::CT9),
            3 => Val(PAD59FNCSEL_A::GPIO59),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_8`"]
    #[inline(always)]
    pub fn is_mspi1_8(&self) -> bool {
        *self == PAD59FNCSEL_A::MSPI1_8
    }
    #[doc = "Checks if the value of the field is `NCE59`"]
    #[inline(always)]
    pub fn is_nce59(&self) -> bool {
        *self == PAD59FNCSEL_A::NCE59
    }
    #[doc = "Checks if the value of the field is `CT9`"]
    #[inline(always)]
    pub fn is_ct9(&self) -> bool {
        *self == PAD59FNCSEL_A::CT9
    }
    #[doc = "Checks if the value of the field is `GPIO59`"]
    #[inline(always)]
    pub fn is_gpio59(&self) -> bool {
        *self == PAD59FNCSEL_A::GPIO59
    }
}
#[doc = "Write proxy for field `PAD59FNCSEL`"]
pub struct PAD59FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD59FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 8 signal"]
    #[inline(always)]
    pub fn mspi1_8(self) -> &'a mut W {
        self.variant(PAD59FNCSEL_A::MSPI1_8)
    }
    #[doc = "IOM/MSPI nCE group 59"]
    #[inline(always)]
    pub fn nce59(self) -> &'a mut W {
        self.variant(PAD59FNCSEL_A::NCE59)
    }
    #[doc = "CTIMER connection 9"]
    #[inline(always)]
    pub fn ct9(self) -> &'a mut W {
        self.variant(PAD59FNCSEL_A::CT9)
    }
    #[doc = "Configure as GPIO59"]
    #[inline(always)]
    pub fn gpio59(self) -> &'a mut W {
        self.variant(PAD59FNCSEL_A::GPIO59)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 59 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD59STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD59STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD59STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD59STRNG`"]
pub type PAD59STRNG_R = crate::R<bool, PAD59STRNG_A>;
impl PAD59STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD59STRNG_A {
        match self.bits {
            false => PAD59STRNG_A::LOW,
            true => PAD59STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD59STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD59STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD59STRNG`"]
pub struct PAD59STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD59STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD59STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD59STRNG_A::HIGH)
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
#[doc = "Pad 59 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD59INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD59INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD59INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD59INPEN`"]
pub type PAD59INPEN_R = crate::R<bool, PAD59INPEN_A>;
impl PAD59INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD59INPEN_A {
        match self.bits {
            false => PAD59INPEN_A::DIS,
            true => PAD59INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD59INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD59INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD59INPEN`"]
pub struct PAD59INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD59INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD59INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD59INPEN_A::EN)
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
#[doc = "Pad 59 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD59PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD59PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD59PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD59PULL`"]
pub type PAD59PULL_R = crate::R<bool, PAD59PULL_A>;
impl PAD59PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD59PULL_A {
        match self.bits {
            false => PAD59PULL_A::DIS,
            true => PAD59PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD59PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD59PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD59PULL`"]
pub struct PAD59PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD59PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD59PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD59PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD59PULL_A::EN)
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
#[doc = "Pad 58 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD58FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 7 signal"]
    MSPI1_7 = 0,
    #[doc = "1: IOM/MSPI nCE group 58"]
    NCE58 = 1,
    #[doc = "2: CTIMER connection 8"]
    CT8 = 2,
    #[doc = "3: Configure as GPIO58"]
    GPIO58 = 3,
}
impl From<PAD58FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD58FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD58FNCSEL`"]
pub type PAD58FNCSEL_R = crate::R<u8, PAD58FNCSEL_A>;
impl PAD58FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD58FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD58FNCSEL_A::MSPI1_7),
            1 => Val(PAD58FNCSEL_A::NCE58),
            2 => Val(PAD58FNCSEL_A::CT8),
            3 => Val(PAD58FNCSEL_A::GPIO58),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_7`"]
    #[inline(always)]
    pub fn is_mspi1_7(&self) -> bool {
        *self == PAD58FNCSEL_A::MSPI1_7
    }
    #[doc = "Checks if the value of the field is `NCE58`"]
    #[inline(always)]
    pub fn is_nce58(&self) -> bool {
        *self == PAD58FNCSEL_A::NCE58
    }
    #[doc = "Checks if the value of the field is `CT8`"]
    #[inline(always)]
    pub fn is_ct8(&self) -> bool {
        *self == PAD58FNCSEL_A::CT8
    }
    #[doc = "Checks if the value of the field is `GPIO58`"]
    #[inline(always)]
    pub fn is_gpio58(&self) -> bool {
        *self == PAD58FNCSEL_A::GPIO58
    }
}
#[doc = "Write proxy for field `PAD58FNCSEL`"]
pub struct PAD58FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD58FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 7 signal"]
    #[inline(always)]
    pub fn mspi1_7(self) -> &'a mut W {
        self.variant(PAD58FNCSEL_A::MSPI1_7)
    }
    #[doc = "IOM/MSPI nCE group 58"]
    #[inline(always)]
    pub fn nce58(self) -> &'a mut W {
        self.variant(PAD58FNCSEL_A::NCE58)
    }
    #[doc = "CTIMER connection 8"]
    #[inline(always)]
    pub fn ct8(self) -> &'a mut W {
        self.variant(PAD58FNCSEL_A::CT8)
    }
    #[doc = "Configure as GPIO58"]
    #[inline(always)]
    pub fn gpio58(self) -> &'a mut W {
        self.variant(PAD58FNCSEL_A::GPIO58)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 58 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD58STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD58STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD58STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD58STRNG`"]
pub type PAD58STRNG_R = crate::R<bool, PAD58STRNG_A>;
impl PAD58STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD58STRNG_A {
        match self.bits {
            false => PAD58STRNG_A::LOW,
            true => PAD58STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD58STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD58STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD58STRNG`"]
pub struct PAD58STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD58STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD58STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD58STRNG_A::HIGH)
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
#[doc = "Pad 58 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD58INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD58INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD58INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD58INPEN`"]
pub type PAD58INPEN_R = crate::R<bool, PAD58INPEN_A>;
impl PAD58INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD58INPEN_A {
        match self.bits {
            false => PAD58INPEN_A::DIS,
            true => PAD58INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD58INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD58INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD58INPEN`"]
pub struct PAD58INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD58INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD58INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD58INPEN_A::EN)
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
#[doc = "Pad 58 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD58PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD58PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD58PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD58PULL`"]
pub type PAD58PULL_R = crate::R<bool, PAD58PULL_A>;
impl PAD58PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD58PULL_A {
        match self.bits {
            false => PAD58PULL_A::DIS,
            true => PAD58PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD58PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD58PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD58PULL`"]
pub struct PAD58PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD58PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD58PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD58PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD58PULL_A::EN)
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
#[doc = "Pad 57 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD57FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 6 signal"]
    MSPI1_6 = 0,
    #[doc = "1: IOM/MSPI nCE group 57"]
    NCE57 = 1,
    #[doc = "2: CTIMER connection 7"]
    CT7 = 2,
    #[doc = "3: Configure as GPIO57"]
    GPIO57 = 3,
}
impl From<PAD57FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD57FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD57FNCSEL`"]
pub type PAD57FNCSEL_R = crate::R<u8, PAD57FNCSEL_A>;
impl PAD57FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD57FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD57FNCSEL_A::MSPI1_6),
            1 => Val(PAD57FNCSEL_A::NCE57),
            2 => Val(PAD57FNCSEL_A::CT7),
            3 => Val(PAD57FNCSEL_A::GPIO57),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_6`"]
    #[inline(always)]
    pub fn is_mspi1_6(&self) -> bool {
        *self == PAD57FNCSEL_A::MSPI1_6
    }
    #[doc = "Checks if the value of the field is `NCE57`"]
    #[inline(always)]
    pub fn is_nce57(&self) -> bool {
        *self == PAD57FNCSEL_A::NCE57
    }
    #[doc = "Checks if the value of the field is `CT7`"]
    #[inline(always)]
    pub fn is_ct7(&self) -> bool {
        *self == PAD57FNCSEL_A::CT7
    }
    #[doc = "Checks if the value of the field is `GPIO57`"]
    #[inline(always)]
    pub fn is_gpio57(&self) -> bool {
        *self == PAD57FNCSEL_A::GPIO57
    }
}
#[doc = "Write proxy for field `PAD57FNCSEL`"]
pub struct PAD57FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD57FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 6 signal"]
    #[inline(always)]
    pub fn mspi1_6(self) -> &'a mut W {
        self.variant(PAD57FNCSEL_A::MSPI1_6)
    }
    #[doc = "IOM/MSPI nCE group 57"]
    #[inline(always)]
    pub fn nce57(self) -> &'a mut W {
        self.variant(PAD57FNCSEL_A::NCE57)
    }
    #[doc = "CTIMER connection 7"]
    #[inline(always)]
    pub fn ct7(self) -> &'a mut W {
        self.variant(PAD57FNCSEL_A::CT7)
    }
    #[doc = "Configure as GPIO57"]
    #[inline(always)]
    pub fn gpio57(self) -> &'a mut W {
        self.variant(PAD57FNCSEL_A::GPIO57)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 57 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD57STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD57STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD57STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD57STRNG`"]
pub type PAD57STRNG_R = crate::R<bool, PAD57STRNG_A>;
impl PAD57STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD57STRNG_A {
        match self.bits {
            false => PAD57STRNG_A::LOW,
            true => PAD57STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD57STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD57STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD57STRNG`"]
pub struct PAD57STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD57STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD57STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD57STRNG_A::HIGH)
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
#[doc = "Pad 57 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD57INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD57INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD57INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD57INPEN`"]
pub type PAD57INPEN_R = crate::R<bool, PAD57INPEN_A>;
impl PAD57INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD57INPEN_A {
        match self.bits {
            false => PAD57INPEN_A::DIS,
            true => PAD57INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD57INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD57INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD57INPEN`"]
pub struct PAD57INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD57INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD57INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD57INPEN_A::EN)
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
#[doc = "Pad 57 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD57PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD57PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD57PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD57PULL`"]
pub type PAD57PULL_R = crate::R<bool, PAD57PULL_A>;
impl PAD57PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD57PULL_A {
        match self.bits {
            false => PAD57PULL_A::DIS,
            true => PAD57PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD57PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD57PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD57PULL`"]
pub struct PAD57PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD57PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD57PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD57PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD57PULL_A::EN)
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
#[doc = "Pad 56 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD56FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 5 signal"]
    MSPI1_5 = 0,
    #[doc = "1: IOM/MSPI nCE group 56"]
    NCE56 = 1,
    #[doc = "2: CTIMER connection 6"]
    CT6 = 2,
    #[doc = "3: Configure as GPIO56"]
    GPIO56 = 3,
}
impl From<PAD56FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD56FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD56FNCSEL`"]
pub type PAD56FNCSEL_R = crate::R<u8, PAD56FNCSEL_A>;
impl PAD56FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD56FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD56FNCSEL_A::MSPI1_5),
            1 => Val(PAD56FNCSEL_A::NCE56),
            2 => Val(PAD56FNCSEL_A::CT6),
            3 => Val(PAD56FNCSEL_A::GPIO56),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_5`"]
    #[inline(always)]
    pub fn is_mspi1_5(&self) -> bool {
        *self == PAD56FNCSEL_A::MSPI1_5
    }
    #[doc = "Checks if the value of the field is `NCE56`"]
    #[inline(always)]
    pub fn is_nce56(&self) -> bool {
        *self == PAD56FNCSEL_A::NCE56
    }
    #[doc = "Checks if the value of the field is `CT6`"]
    #[inline(always)]
    pub fn is_ct6(&self) -> bool {
        *self == PAD56FNCSEL_A::CT6
    }
    #[doc = "Checks if the value of the field is `GPIO56`"]
    #[inline(always)]
    pub fn is_gpio56(&self) -> bool {
        *self == PAD56FNCSEL_A::GPIO56
    }
}
#[doc = "Write proxy for field `PAD56FNCSEL`"]
pub struct PAD56FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD56FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 5 signal"]
    #[inline(always)]
    pub fn mspi1_5(self) -> &'a mut W {
        self.variant(PAD56FNCSEL_A::MSPI1_5)
    }
    #[doc = "IOM/MSPI nCE group 56"]
    #[inline(always)]
    pub fn nce56(self) -> &'a mut W {
        self.variant(PAD56FNCSEL_A::NCE56)
    }
    #[doc = "CTIMER connection 6"]
    #[inline(always)]
    pub fn ct6(self) -> &'a mut W {
        self.variant(PAD56FNCSEL_A::CT6)
    }
    #[doc = "Configure as GPIO56"]
    #[inline(always)]
    pub fn gpio56(self) -> &'a mut W {
        self.variant(PAD56FNCSEL_A::GPIO56)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 56 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD56STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD56STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD56STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD56STRNG`"]
pub type PAD56STRNG_R = crate::R<bool, PAD56STRNG_A>;
impl PAD56STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD56STRNG_A {
        match self.bits {
            false => PAD56STRNG_A::LOW,
            true => PAD56STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD56STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD56STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD56STRNG`"]
pub struct PAD56STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD56STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD56STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD56STRNG_A::HIGH)
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
#[doc = "Pad 56 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD56INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD56INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD56INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD56INPEN`"]
pub type PAD56INPEN_R = crate::R<bool, PAD56INPEN_A>;
impl PAD56INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD56INPEN_A {
        match self.bits {
            false => PAD56INPEN_A::DIS,
            true => PAD56INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD56INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD56INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD56INPEN`"]
pub struct PAD56INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD56INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD56INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD56INPEN_A::EN)
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
#[doc = "Pad 56 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD56PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD56PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD56PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD56PULL`"]
pub type PAD56PULL_R = crate::R<bool, PAD56PULL_A>;
impl PAD56PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD56PULL_A {
        match self.bits {
            false => PAD56PULL_A::DIS,
            true => PAD56PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD56PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD56PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD56PULL`"]
pub struct PAD56PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD56PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD56PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD56PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD56PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 59 function select"]
    #[inline(always)]
    pub fn pad59fncsel(&self) -> PAD59FNCSEL_R {
        PAD59FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 59 drive strength"]
    #[inline(always)]
    pub fn pad59strng(&self) -> PAD59STRNG_R {
        PAD59STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 59 input enable"]
    #[inline(always)]
    pub fn pad59inpen(&self) -> PAD59INPEN_R {
        PAD59INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 59 pullup enable"]
    #[inline(always)]
    pub fn pad59pull(&self) -> PAD59PULL_R {
        PAD59PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 58 function select"]
    #[inline(always)]
    pub fn pad58fncsel(&self) -> PAD58FNCSEL_R {
        PAD58FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 58 drive strength"]
    #[inline(always)]
    pub fn pad58strng(&self) -> PAD58STRNG_R {
        PAD58STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 58 input enable"]
    #[inline(always)]
    pub fn pad58inpen(&self) -> PAD58INPEN_R {
        PAD58INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 58 pullup enable"]
    #[inline(always)]
    pub fn pad58pull(&self) -> PAD58PULL_R {
        PAD58PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 57 function select"]
    #[inline(always)]
    pub fn pad57fncsel(&self) -> PAD57FNCSEL_R {
        PAD57FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 57 drive strength"]
    #[inline(always)]
    pub fn pad57strng(&self) -> PAD57STRNG_R {
        PAD57STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 57 input enable"]
    #[inline(always)]
    pub fn pad57inpen(&self) -> PAD57INPEN_R {
        PAD57INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 57 pullup enable"]
    #[inline(always)]
    pub fn pad57pull(&self) -> PAD57PULL_R {
        PAD57PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 56 function select"]
    #[inline(always)]
    pub fn pad56fncsel(&self) -> PAD56FNCSEL_R {
        PAD56FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 56 drive strength"]
    #[inline(always)]
    pub fn pad56strng(&self) -> PAD56STRNG_R {
        PAD56STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 56 input enable"]
    #[inline(always)]
    pub fn pad56inpen(&self) -> PAD56INPEN_R {
        PAD56INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 56 pullup enable"]
    #[inline(always)]
    pub fn pad56pull(&self) -> PAD56PULL_R {
        PAD56PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 59 function select"]
    #[inline(always)]
    pub fn pad59fncsel(&mut self) -> PAD59FNCSEL_W {
        PAD59FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 59 drive strength"]
    #[inline(always)]
    pub fn pad59strng(&mut self) -> PAD59STRNG_W {
        PAD59STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 59 input enable"]
    #[inline(always)]
    pub fn pad59inpen(&mut self) -> PAD59INPEN_W {
        PAD59INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 59 pullup enable"]
    #[inline(always)]
    pub fn pad59pull(&mut self) -> PAD59PULL_W {
        PAD59PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 58 function select"]
    #[inline(always)]
    pub fn pad58fncsel(&mut self) -> PAD58FNCSEL_W {
        PAD58FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 58 drive strength"]
    #[inline(always)]
    pub fn pad58strng(&mut self) -> PAD58STRNG_W {
        PAD58STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 58 input enable"]
    #[inline(always)]
    pub fn pad58inpen(&mut self) -> PAD58INPEN_W {
        PAD58INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 58 pullup enable"]
    #[inline(always)]
    pub fn pad58pull(&mut self) -> PAD58PULL_W {
        PAD58PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 57 function select"]
    #[inline(always)]
    pub fn pad57fncsel(&mut self) -> PAD57FNCSEL_W {
        PAD57FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 57 drive strength"]
    #[inline(always)]
    pub fn pad57strng(&mut self) -> PAD57STRNG_W {
        PAD57STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 57 input enable"]
    #[inline(always)]
    pub fn pad57inpen(&mut self) -> PAD57INPEN_W {
        PAD57INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 57 pullup enable"]
    #[inline(always)]
    pub fn pad57pull(&mut self) -> PAD57PULL_W {
        PAD57PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 56 function select"]
    #[inline(always)]
    pub fn pad56fncsel(&mut self) -> PAD56FNCSEL_W {
        PAD56FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 56 drive strength"]
    #[inline(always)]
    pub fn pad56strng(&mut self) -> PAD56STRNG_W {
        PAD56STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 56 input enable"]
    #[inline(always)]
    pub fn pad56inpen(&mut self) -> PAD56INPEN_W {
        PAD56INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 56 pullup enable"]
    #[inline(always)]
    pub fn pad56pull(&mut self) -> PAD56PULL_W {
        PAD56PULL_W { w: self }
    }
}
