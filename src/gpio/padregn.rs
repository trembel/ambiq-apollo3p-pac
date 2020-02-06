#[doc = "Reader of register PADREGN"]
pub type R = crate::R<u32, super::PADREGN>;
#[doc = "Writer for register PADREGN"]
pub type W = crate::W<u32, super::PADREGN>;
#[doc = "Register PADREGN `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 55 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD55FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 4 signal"]
    MSPI1_4 = 0,
    #[doc = "1: IOM/MSPI nCE group 55"]
    NCE55 = 1,
    #[doc = "2: CTIMER connection 5"]
    CT5 = 2,
    #[doc = "3: Configure as GPIO55"]
    GPIO55 = 3,
}
impl From<PAD55FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD55FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD55FNCSEL`"]
pub type PAD55FNCSEL_R = crate::R<u8, PAD55FNCSEL_A>;
impl PAD55FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD55FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD55FNCSEL_A::MSPI1_4),
            1 => Val(PAD55FNCSEL_A::NCE55),
            2 => Val(PAD55FNCSEL_A::CT5),
            3 => Val(PAD55FNCSEL_A::GPIO55),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_4`"]
    #[inline(always)]
    pub fn is_mspi1_4(&self) -> bool {
        *self == PAD55FNCSEL_A::MSPI1_4
    }
    #[doc = "Checks if the value of the field is `NCE55`"]
    #[inline(always)]
    pub fn is_nce55(&self) -> bool {
        *self == PAD55FNCSEL_A::NCE55
    }
    #[doc = "Checks if the value of the field is `CT5`"]
    #[inline(always)]
    pub fn is_ct5(&self) -> bool {
        *self == PAD55FNCSEL_A::CT5
    }
    #[doc = "Checks if the value of the field is `GPIO55`"]
    #[inline(always)]
    pub fn is_gpio55(&self) -> bool {
        *self == PAD55FNCSEL_A::GPIO55
    }
}
#[doc = "Write proxy for field `PAD55FNCSEL`"]
pub struct PAD55FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD55FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 4 signal"]
    #[inline(always)]
    pub fn mspi1_4(self) -> &'a mut W {
        self.variant(PAD55FNCSEL_A::MSPI1_4)
    }
    #[doc = "IOM/MSPI nCE group 55"]
    #[inline(always)]
    pub fn nce55(self) -> &'a mut W {
        self.variant(PAD55FNCSEL_A::NCE55)
    }
    #[doc = "CTIMER connection 5"]
    #[inline(always)]
    pub fn ct5(self) -> &'a mut W {
        self.variant(PAD55FNCSEL_A::CT5)
    }
    #[doc = "Configure as GPIO55"]
    #[inline(always)]
    pub fn gpio55(self) -> &'a mut W {
        self.variant(PAD55FNCSEL_A::GPIO55)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 55 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD55STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD55STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD55STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD55STRNG`"]
pub type PAD55STRNG_R = crate::R<bool, PAD55STRNG_A>;
impl PAD55STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD55STRNG_A {
        match self.bits {
            false => PAD55STRNG_A::LOW,
            true => PAD55STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD55STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD55STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD55STRNG`"]
pub struct PAD55STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD55STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD55STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD55STRNG_A::HIGH)
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
#[doc = "Pad 55 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD55INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD55INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD55INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD55INPEN`"]
pub type PAD55INPEN_R = crate::R<bool, PAD55INPEN_A>;
impl PAD55INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD55INPEN_A {
        match self.bits {
            false => PAD55INPEN_A::DIS,
            true => PAD55INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD55INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD55INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD55INPEN`"]
pub struct PAD55INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD55INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD55INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD55INPEN_A::EN)
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
#[doc = "Pad 55 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD55PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD55PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD55PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD55PULL`"]
pub type PAD55PULL_R = crate::R<bool, PAD55PULL_A>;
impl PAD55PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD55PULL_A {
        match self.bits {
            false => PAD55PULL_A::DIS,
            true => PAD55PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD55PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD55PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD55PULL`"]
pub struct PAD55PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD55PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD55PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD55PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD55PULL_A::EN)
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
#[doc = "Pad 54 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD54FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 3 signal"]
    MSPI1_3 = 0,
    #[doc = "1: IOM/MSPI nCE group 54"]
    NCE54 = 1,
    #[doc = "2: CTIMER connection 4"]
    CT4 = 2,
    #[doc = "3: Configure as GPIO54"]
    GPIO54 = 3,
}
impl From<PAD54FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD54FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD54FNCSEL`"]
pub type PAD54FNCSEL_R = crate::R<u8, PAD54FNCSEL_A>;
impl PAD54FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD54FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD54FNCSEL_A::MSPI1_3),
            1 => Val(PAD54FNCSEL_A::NCE54),
            2 => Val(PAD54FNCSEL_A::CT4),
            3 => Val(PAD54FNCSEL_A::GPIO54),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_3`"]
    #[inline(always)]
    pub fn is_mspi1_3(&self) -> bool {
        *self == PAD54FNCSEL_A::MSPI1_3
    }
    #[doc = "Checks if the value of the field is `NCE54`"]
    #[inline(always)]
    pub fn is_nce54(&self) -> bool {
        *self == PAD54FNCSEL_A::NCE54
    }
    #[doc = "Checks if the value of the field is `CT4`"]
    #[inline(always)]
    pub fn is_ct4(&self) -> bool {
        *self == PAD54FNCSEL_A::CT4
    }
    #[doc = "Checks if the value of the field is `GPIO54`"]
    #[inline(always)]
    pub fn is_gpio54(&self) -> bool {
        *self == PAD54FNCSEL_A::GPIO54
    }
}
#[doc = "Write proxy for field `PAD54FNCSEL`"]
pub struct PAD54FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD54FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 3 signal"]
    #[inline(always)]
    pub fn mspi1_3(self) -> &'a mut W {
        self.variant(PAD54FNCSEL_A::MSPI1_3)
    }
    #[doc = "IOM/MSPI nCE group 54"]
    #[inline(always)]
    pub fn nce54(self) -> &'a mut W {
        self.variant(PAD54FNCSEL_A::NCE54)
    }
    #[doc = "CTIMER connection 4"]
    #[inline(always)]
    pub fn ct4(self) -> &'a mut W {
        self.variant(PAD54FNCSEL_A::CT4)
    }
    #[doc = "Configure as GPIO54"]
    #[inline(always)]
    pub fn gpio54(self) -> &'a mut W {
        self.variant(PAD54FNCSEL_A::GPIO54)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 54 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD54STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD54STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD54STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD54STRNG`"]
pub type PAD54STRNG_R = crate::R<bool, PAD54STRNG_A>;
impl PAD54STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD54STRNG_A {
        match self.bits {
            false => PAD54STRNG_A::LOW,
            true => PAD54STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD54STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD54STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD54STRNG`"]
pub struct PAD54STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD54STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD54STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD54STRNG_A::HIGH)
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
#[doc = "Pad 54 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD54INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD54INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD54INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD54INPEN`"]
pub type PAD54INPEN_R = crate::R<bool, PAD54INPEN_A>;
impl PAD54INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD54INPEN_A {
        match self.bits {
            false => PAD54INPEN_A::DIS,
            true => PAD54INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD54INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD54INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD54INPEN`"]
pub struct PAD54INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD54INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD54INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD54INPEN_A::EN)
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
#[doc = "Pad 54 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD54PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD54PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD54PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD54PULL`"]
pub type PAD54PULL_R = crate::R<bool, PAD54PULL_A>;
impl PAD54PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD54PULL_A {
        match self.bits {
            false => PAD54PULL_A::DIS,
            true => PAD54PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD54PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD54PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD54PULL`"]
pub struct PAD54PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD54PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD54PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD54PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD54PULL_A::EN)
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
#[doc = "Pad 53 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD53FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 2 signal"]
    MSPI1_2 = 0,
    #[doc = "1: IOM/MSPI nCE group 53"]
    NCE53 = 1,
    #[doc = "2: CTIMER connection 3"]
    CT3 = 2,
    #[doc = "3: Configure as GPIO53"]
    GPIO53 = 3,
}
impl From<PAD53FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD53FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD53FNCSEL`"]
pub type PAD53FNCSEL_R = crate::R<u8, PAD53FNCSEL_A>;
impl PAD53FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD53FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD53FNCSEL_A::MSPI1_2),
            1 => Val(PAD53FNCSEL_A::NCE53),
            2 => Val(PAD53FNCSEL_A::CT3),
            3 => Val(PAD53FNCSEL_A::GPIO53),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_2`"]
    #[inline(always)]
    pub fn is_mspi1_2(&self) -> bool {
        *self == PAD53FNCSEL_A::MSPI1_2
    }
    #[doc = "Checks if the value of the field is `NCE53`"]
    #[inline(always)]
    pub fn is_nce53(&self) -> bool {
        *self == PAD53FNCSEL_A::NCE53
    }
    #[doc = "Checks if the value of the field is `CT3`"]
    #[inline(always)]
    pub fn is_ct3(&self) -> bool {
        *self == PAD53FNCSEL_A::CT3
    }
    #[doc = "Checks if the value of the field is `GPIO53`"]
    #[inline(always)]
    pub fn is_gpio53(&self) -> bool {
        *self == PAD53FNCSEL_A::GPIO53
    }
}
#[doc = "Write proxy for field `PAD53FNCSEL`"]
pub struct PAD53FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD53FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 2 signal"]
    #[inline(always)]
    pub fn mspi1_2(self) -> &'a mut W {
        self.variant(PAD53FNCSEL_A::MSPI1_2)
    }
    #[doc = "IOM/MSPI nCE group 53"]
    #[inline(always)]
    pub fn nce53(self) -> &'a mut W {
        self.variant(PAD53FNCSEL_A::NCE53)
    }
    #[doc = "CTIMER connection 3"]
    #[inline(always)]
    pub fn ct3(self) -> &'a mut W {
        self.variant(PAD53FNCSEL_A::CT3)
    }
    #[doc = "Configure as GPIO53"]
    #[inline(always)]
    pub fn gpio53(self) -> &'a mut W {
        self.variant(PAD53FNCSEL_A::GPIO53)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 53 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD53STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD53STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD53STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD53STRNG`"]
pub type PAD53STRNG_R = crate::R<bool, PAD53STRNG_A>;
impl PAD53STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD53STRNG_A {
        match self.bits {
            false => PAD53STRNG_A::LOW,
            true => PAD53STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD53STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD53STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD53STRNG`"]
pub struct PAD53STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD53STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD53STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD53STRNG_A::HIGH)
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
#[doc = "Pad 53 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD53INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD53INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD53INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD53INPEN`"]
pub type PAD53INPEN_R = crate::R<bool, PAD53INPEN_A>;
impl PAD53INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD53INPEN_A {
        match self.bits {
            false => PAD53INPEN_A::DIS,
            true => PAD53INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD53INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD53INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD53INPEN`"]
pub struct PAD53INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD53INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD53INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD53INPEN_A::EN)
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
#[doc = "Pad 53 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD53PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD53PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD53PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD53PULL`"]
pub type PAD53PULL_R = crate::R<bool, PAD53PULL_A>;
impl PAD53PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD53PULL_A {
        match self.bits {
            false => PAD53PULL_A::DIS,
            true => PAD53PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD53PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD53PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD53PULL`"]
pub struct PAD53PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD53PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD53PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD53PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD53PULL_A::EN)
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
#[doc = "Pad 52 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD52FNCSEL_A {
    #[doc = "0: Configure as the MSPI1 1 signal"]
    MSPI1_1 = 0,
    #[doc = "1: IOM/MSPI nCE group 52"]
    NCE52 = 1,
    #[doc = "2: CTIMER connection 2"]
    CT2 = 2,
    #[doc = "3: Configure as GPIO52"]
    GPIO52 = 3,
}
impl From<PAD52FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD52FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD52FNCSEL`"]
pub type PAD52FNCSEL_R = crate::R<u8, PAD52FNCSEL_A>;
impl PAD52FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD52FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD52FNCSEL_A::MSPI1_1),
            1 => Val(PAD52FNCSEL_A::NCE52),
            2 => Val(PAD52FNCSEL_A::CT2),
            3 => Val(PAD52FNCSEL_A::GPIO52),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI1_1`"]
    #[inline(always)]
    pub fn is_mspi1_1(&self) -> bool {
        *self == PAD52FNCSEL_A::MSPI1_1
    }
    #[doc = "Checks if the value of the field is `NCE52`"]
    #[inline(always)]
    pub fn is_nce52(&self) -> bool {
        *self == PAD52FNCSEL_A::NCE52
    }
    #[doc = "Checks if the value of the field is `CT2`"]
    #[inline(always)]
    pub fn is_ct2(&self) -> bool {
        *self == PAD52FNCSEL_A::CT2
    }
    #[doc = "Checks if the value of the field is `GPIO52`"]
    #[inline(always)]
    pub fn is_gpio52(&self) -> bool {
        *self == PAD52FNCSEL_A::GPIO52
    }
}
#[doc = "Write proxy for field `PAD52FNCSEL`"]
pub struct PAD52FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD52FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI1 1 signal"]
    #[inline(always)]
    pub fn mspi1_1(self) -> &'a mut W {
        self.variant(PAD52FNCSEL_A::MSPI1_1)
    }
    #[doc = "IOM/MSPI nCE group 52"]
    #[inline(always)]
    pub fn nce52(self) -> &'a mut W {
        self.variant(PAD52FNCSEL_A::NCE52)
    }
    #[doc = "CTIMER connection 2"]
    #[inline(always)]
    pub fn ct2(self) -> &'a mut W {
        self.variant(PAD52FNCSEL_A::CT2)
    }
    #[doc = "Configure as GPIO52"]
    #[inline(always)]
    pub fn gpio52(self) -> &'a mut W {
        self.variant(PAD52FNCSEL_A::GPIO52)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 52 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD52STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD52STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD52STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD52STRNG`"]
pub type PAD52STRNG_R = crate::R<bool, PAD52STRNG_A>;
impl PAD52STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD52STRNG_A {
        match self.bits {
            false => PAD52STRNG_A::LOW,
            true => PAD52STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD52STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD52STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD52STRNG`"]
pub struct PAD52STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD52STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD52STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD52STRNG_A::HIGH)
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
#[doc = "Pad 52 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD52INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD52INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD52INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD52INPEN`"]
pub type PAD52INPEN_R = crate::R<bool, PAD52INPEN_A>;
impl PAD52INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD52INPEN_A {
        match self.bits {
            false => PAD52INPEN_A::DIS,
            true => PAD52INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD52INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD52INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD52INPEN`"]
pub struct PAD52INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD52INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD52INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD52INPEN_A::EN)
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
#[doc = "Pad 52 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD52PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD52PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD52PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD52PULL`"]
pub type PAD52PULL_R = crate::R<bool, PAD52PULL_A>;
impl PAD52PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD52PULL_A {
        match self.bits {
            false => PAD52PULL_A::DIS,
            true => PAD52PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD52PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD52PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD52PULL`"]
pub struct PAD52PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD52PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD52PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD52PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD52PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 55 function select"]
    #[inline(always)]
    pub fn pad55fncsel(&self) -> PAD55FNCSEL_R {
        PAD55FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 55 drive strength"]
    #[inline(always)]
    pub fn pad55strng(&self) -> PAD55STRNG_R {
        PAD55STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 55 input enable"]
    #[inline(always)]
    pub fn pad55inpen(&self) -> PAD55INPEN_R {
        PAD55INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 55 pullup enable"]
    #[inline(always)]
    pub fn pad55pull(&self) -> PAD55PULL_R {
        PAD55PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 54 function select"]
    #[inline(always)]
    pub fn pad54fncsel(&self) -> PAD54FNCSEL_R {
        PAD54FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 54 drive strength"]
    #[inline(always)]
    pub fn pad54strng(&self) -> PAD54STRNG_R {
        PAD54STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 54 input enable"]
    #[inline(always)]
    pub fn pad54inpen(&self) -> PAD54INPEN_R {
        PAD54INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 54 pullup enable"]
    #[inline(always)]
    pub fn pad54pull(&self) -> PAD54PULL_R {
        PAD54PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 53 function select"]
    #[inline(always)]
    pub fn pad53fncsel(&self) -> PAD53FNCSEL_R {
        PAD53FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 53 drive strength"]
    #[inline(always)]
    pub fn pad53strng(&self) -> PAD53STRNG_R {
        PAD53STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 53 input enable"]
    #[inline(always)]
    pub fn pad53inpen(&self) -> PAD53INPEN_R {
        PAD53INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 53 pullup enable"]
    #[inline(always)]
    pub fn pad53pull(&self) -> PAD53PULL_R {
        PAD53PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 52 function select"]
    #[inline(always)]
    pub fn pad52fncsel(&self) -> PAD52FNCSEL_R {
        PAD52FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 52 drive strength"]
    #[inline(always)]
    pub fn pad52strng(&self) -> PAD52STRNG_R {
        PAD52STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 52 input enable"]
    #[inline(always)]
    pub fn pad52inpen(&self) -> PAD52INPEN_R {
        PAD52INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 52 pullup enable"]
    #[inline(always)]
    pub fn pad52pull(&self) -> PAD52PULL_R {
        PAD52PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 55 function select"]
    #[inline(always)]
    pub fn pad55fncsel(&mut self) -> PAD55FNCSEL_W {
        PAD55FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 55 drive strength"]
    #[inline(always)]
    pub fn pad55strng(&mut self) -> PAD55STRNG_W {
        PAD55STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 55 input enable"]
    #[inline(always)]
    pub fn pad55inpen(&mut self) -> PAD55INPEN_W {
        PAD55INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 55 pullup enable"]
    #[inline(always)]
    pub fn pad55pull(&mut self) -> PAD55PULL_W {
        PAD55PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 54 function select"]
    #[inline(always)]
    pub fn pad54fncsel(&mut self) -> PAD54FNCSEL_W {
        PAD54FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 54 drive strength"]
    #[inline(always)]
    pub fn pad54strng(&mut self) -> PAD54STRNG_W {
        PAD54STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 54 input enable"]
    #[inline(always)]
    pub fn pad54inpen(&mut self) -> PAD54INPEN_W {
        PAD54INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 54 pullup enable"]
    #[inline(always)]
    pub fn pad54pull(&mut self) -> PAD54PULL_W {
        PAD54PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 53 function select"]
    #[inline(always)]
    pub fn pad53fncsel(&mut self) -> PAD53FNCSEL_W {
        PAD53FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 53 drive strength"]
    #[inline(always)]
    pub fn pad53strng(&mut self) -> PAD53STRNG_W {
        PAD53STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 53 input enable"]
    #[inline(always)]
    pub fn pad53inpen(&mut self) -> PAD53INPEN_W {
        PAD53INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 53 pullup enable"]
    #[inline(always)]
    pub fn pad53pull(&mut self) -> PAD53PULL_W {
        PAD53PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 52 function select"]
    #[inline(always)]
    pub fn pad52fncsel(&mut self) -> PAD52FNCSEL_W {
        PAD52FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 52 drive strength"]
    #[inline(always)]
    pub fn pad52strng(&mut self) -> PAD52STRNG_W {
        PAD52STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 52 input enable"]
    #[inline(always)]
    pub fn pad52inpen(&mut self) -> PAD52INPEN_W {
        PAD52INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 52 pullup enable"]
    #[inline(always)]
    pub fn pad52pull(&mut self) -> PAD52PULL_W {
        PAD52PULL_W { w: self }
    }
}
