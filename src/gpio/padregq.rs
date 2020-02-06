#[doc = "Reader of register PADREGQ"]
pub type R = crate::R<u32, super::PADREGQ>;
#[doc = "Writer for register PADREGQ"]
pub type W = crate::W<u32, super::PADREGQ>;
#[doc = "Register PADREGQ `reset()`'s with value 0x1818_1818"]
impl crate::ResetValue for super::PADREGQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1818_1818
    }
}
#[doc = "Pad 67 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD67FNCSEL_A {
    #[doc = "0: Configure as the MSPI2 3 signal"]
    MSPI2_3 = 0,
    #[doc = "1: IOM/MSPI nCE group 67"]
    NCE67 = 1,
    #[doc = "2: CTIMER connection 17"]
    CT17 = 2,
    #[doc = "3: Configure as GPIO67"]
    GPIO67 = 3,
}
impl From<PAD67FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD67FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD67FNCSEL`"]
pub type PAD67FNCSEL_R = crate::R<u8, PAD67FNCSEL_A>;
impl PAD67FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD67FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD67FNCSEL_A::MSPI2_3),
            1 => Val(PAD67FNCSEL_A::NCE67),
            2 => Val(PAD67FNCSEL_A::CT17),
            3 => Val(PAD67FNCSEL_A::GPIO67),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI2_3`"]
    #[inline(always)]
    pub fn is_mspi2_3(&self) -> bool {
        *self == PAD67FNCSEL_A::MSPI2_3
    }
    #[doc = "Checks if the value of the field is `NCE67`"]
    #[inline(always)]
    pub fn is_nce67(&self) -> bool {
        *self == PAD67FNCSEL_A::NCE67
    }
    #[doc = "Checks if the value of the field is `CT17`"]
    #[inline(always)]
    pub fn is_ct17(&self) -> bool {
        *self == PAD67FNCSEL_A::CT17
    }
    #[doc = "Checks if the value of the field is `GPIO67`"]
    #[inline(always)]
    pub fn is_gpio67(&self) -> bool {
        *self == PAD67FNCSEL_A::GPIO67
    }
}
#[doc = "Write proxy for field `PAD67FNCSEL`"]
pub struct PAD67FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD67FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI2 3 signal"]
    #[inline(always)]
    pub fn mspi2_3(self) -> &'a mut W {
        self.variant(PAD67FNCSEL_A::MSPI2_3)
    }
    #[doc = "IOM/MSPI nCE group 67"]
    #[inline(always)]
    pub fn nce67(self) -> &'a mut W {
        self.variant(PAD67FNCSEL_A::NCE67)
    }
    #[doc = "CTIMER connection 17"]
    #[inline(always)]
    pub fn ct17(self) -> &'a mut W {
        self.variant(PAD67FNCSEL_A::CT17)
    }
    #[doc = "Configure as GPIO67"]
    #[inline(always)]
    pub fn gpio67(self) -> &'a mut W {
        self.variant(PAD67FNCSEL_A::GPIO67)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 67 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD67STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD67STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD67STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD67STRNG`"]
pub type PAD67STRNG_R = crate::R<bool, PAD67STRNG_A>;
impl PAD67STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD67STRNG_A {
        match self.bits {
            false => PAD67STRNG_A::LOW,
            true => PAD67STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD67STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD67STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD67STRNG`"]
pub struct PAD67STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD67STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD67STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD67STRNG_A::HIGH)
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
#[doc = "Pad 67 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD67INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD67INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD67INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD67INPEN`"]
pub type PAD67INPEN_R = crate::R<bool, PAD67INPEN_A>;
impl PAD67INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD67INPEN_A {
        match self.bits {
            false => PAD67INPEN_A::DIS,
            true => PAD67INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD67INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD67INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD67INPEN`"]
pub struct PAD67INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD67INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD67INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD67INPEN_A::EN)
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
#[doc = "Pad 67 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD67PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD67PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD67PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD67PULL`"]
pub type PAD67PULL_R = crate::R<bool, PAD67PULL_A>;
impl PAD67PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD67PULL_A {
        match self.bits {
            false => PAD67PULL_A::DIS,
            true => PAD67PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD67PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD67PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD67PULL`"]
pub struct PAD67PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD67PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD67PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD67PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD67PULL_A::EN)
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
#[doc = "Pad 66 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD66FNCSEL_A {
    #[doc = "0: Configure as the MSPI2 2 signal"]
    MSPI2_2 = 0,
    #[doc = "1: IOM/MSPI nCE group 66"]
    NCE66 = 1,
    #[doc = "2: CTIMER connection 16"]
    CT16 = 2,
    #[doc = "3: Configure as GPIO66"]
    GPIO66 = 3,
}
impl From<PAD66FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD66FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD66FNCSEL`"]
pub type PAD66FNCSEL_R = crate::R<u8, PAD66FNCSEL_A>;
impl PAD66FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD66FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD66FNCSEL_A::MSPI2_2),
            1 => Val(PAD66FNCSEL_A::NCE66),
            2 => Val(PAD66FNCSEL_A::CT16),
            3 => Val(PAD66FNCSEL_A::GPIO66),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI2_2`"]
    #[inline(always)]
    pub fn is_mspi2_2(&self) -> bool {
        *self == PAD66FNCSEL_A::MSPI2_2
    }
    #[doc = "Checks if the value of the field is `NCE66`"]
    #[inline(always)]
    pub fn is_nce66(&self) -> bool {
        *self == PAD66FNCSEL_A::NCE66
    }
    #[doc = "Checks if the value of the field is `CT16`"]
    #[inline(always)]
    pub fn is_ct16(&self) -> bool {
        *self == PAD66FNCSEL_A::CT16
    }
    #[doc = "Checks if the value of the field is `GPIO66`"]
    #[inline(always)]
    pub fn is_gpio66(&self) -> bool {
        *self == PAD66FNCSEL_A::GPIO66
    }
}
#[doc = "Write proxy for field `PAD66FNCSEL`"]
pub struct PAD66FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD66FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI2 2 signal"]
    #[inline(always)]
    pub fn mspi2_2(self) -> &'a mut W {
        self.variant(PAD66FNCSEL_A::MSPI2_2)
    }
    #[doc = "IOM/MSPI nCE group 66"]
    #[inline(always)]
    pub fn nce66(self) -> &'a mut W {
        self.variant(PAD66FNCSEL_A::NCE66)
    }
    #[doc = "CTIMER connection 16"]
    #[inline(always)]
    pub fn ct16(self) -> &'a mut W {
        self.variant(PAD66FNCSEL_A::CT16)
    }
    #[doc = "Configure as GPIO66"]
    #[inline(always)]
    pub fn gpio66(self) -> &'a mut W {
        self.variant(PAD66FNCSEL_A::GPIO66)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 66 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD66STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD66STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD66STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD66STRNG`"]
pub type PAD66STRNG_R = crate::R<bool, PAD66STRNG_A>;
impl PAD66STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD66STRNG_A {
        match self.bits {
            false => PAD66STRNG_A::LOW,
            true => PAD66STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD66STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD66STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD66STRNG`"]
pub struct PAD66STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD66STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD66STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD66STRNG_A::HIGH)
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
#[doc = "Pad 66 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD66INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD66INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD66INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD66INPEN`"]
pub type PAD66INPEN_R = crate::R<bool, PAD66INPEN_A>;
impl PAD66INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD66INPEN_A {
        match self.bits {
            false => PAD66INPEN_A::DIS,
            true => PAD66INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD66INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD66INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD66INPEN`"]
pub struct PAD66INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD66INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD66INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD66INPEN_A::EN)
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
#[doc = "Pad 66 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD66PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD66PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD66PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD66PULL`"]
pub type PAD66PULL_R = crate::R<bool, PAD66PULL_A>;
impl PAD66PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD66PULL_A {
        match self.bits {
            false => PAD66PULL_A::DIS,
            true => PAD66PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD66PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD66PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD66PULL`"]
pub struct PAD66PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD66PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD66PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD66PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD66PULL_A::EN)
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
#[doc = "Pad 65 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD65FNCSEL_A {
    #[doc = "0: Configure as the MSPI2 1 signal"]
    MSPI2_1 = 0,
    #[doc = "1: IOM/MSPI nCE group 65"]
    NCE65 = 1,
    #[doc = "2: CTIMER connection 15"]
    CT15 = 2,
    #[doc = "3: Configure as GPIO65"]
    GPIO65 = 3,
}
impl From<PAD65FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD65FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD65FNCSEL`"]
pub type PAD65FNCSEL_R = crate::R<u8, PAD65FNCSEL_A>;
impl PAD65FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD65FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD65FNCSEL_A::MSPI2_1),
            1 => Val(PAD65FNCSEL_A::NCE65),
            2 => Val(PAD65FNCSEL_A::CT15),
            3 => Val(PAD65FNCSEL_A::GPIO65),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI2_1`"]
    #[inline(always)]
    pub fn is_mspi2_1(&self) -> bool {
        *self == PAD65FNCSEL_A::MSPI2_1
    }
    #[doc = "Checks if the value of the field is `NCE65`"]
    #[inline(always)]
    pub fn is_nce65(&self) -> bool {
        *self == PAD65FNCSEL_A::NCE65
    }
    #[doc = "Checks if the value of the field is `CT15`"]
    #[inline(always)]
    pub fn is_ct15(&self) -> bool {
        *self == PAD65FNCSEL_A::CT15
    }
    #[doc = "Checks if the value of the field is `GPIO65`"]
    #[inline(always)]
    pub fn is_gpio65(&self) -> bool {
        *self == PAD65FNCSEL_A::GPIO65
    }
}
#[doc = "Write proxy for field `PAD65FNCSEL`"]
pub struct PAD65FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD65FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI2 1 signal"]
    #[inline(always)]
    pub fn mspi2_1(self) -> &'a mut W {
        self.variant(PAD65FNCSEL_A::MSPI2_1)
    }
    #[doc = "IOM/MSPI nCE group 65"]
    #[inline(always)]
    pub fn nce65(self) -> &'a mut W {
        self.variant(PAD65FNCSEL_A::NCE65)
    }
    #[doc = "CTIMER connection 15"]
    #[inline(always)]
    pub fn ct15(self) -> &'a mut W {
        self.variant(PAD65FNCSEL_A::CT15)
    }
    #[doc = "Configure as GPIO65"]
    #[inline(always)]
    pub fn gpio65(self) -> &'a mut W {
        self.variant(PAD65FNCSEL_A::GPIO65)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 65 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD65STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD65STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD65STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD65STRNG`"]
pub type PAD65STRNG_R = crate::R<bool, PAD65STRNG_A>;
impl PAD65STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD65STRNG_A {
        match self.bits {
            false => PAD65STRNG_A::LOW,
            true => PAD65STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD65STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD65STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD65STRNG`"]
pub struct PAD65STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD65STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD65STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD65STRNG_A::HIGH)
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
#[doc = "Pad 65 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD65INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD65INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD65INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD65INPEN`"]
pub type PAD65INPEN_R = crate::R<bool, PAD65INPEN_A>;
impl PAD65INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD65INPEN_A {
        match self.bits {
            false => PAD65INPEN_A::DIS,
            true => PAD65INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD65INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD65INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD65INPEN`"]
pub struct PAD65INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD65INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD65INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD65INPEN_A::EN)
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
#[doc = "Pad 65 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD65PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD65PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD65PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD65PULL`"]
pub type PAD65PULL_R = crate::R<bool, PAD65PULL_A>;
impl PAD65PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD65PULL_A {
        match self.bits {
            false => PAD65PULL_A::DIS,
            true => PAD65PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD65PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD65PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD65PULL`"]
pub struct PAD65PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD65PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD65PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD65PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD65PULL_A::EN)
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
#[doc = "Pad 64 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD64FNCSEL_A {
    #[doc = "0: Configure as the MSPI2 0 signal"]
    MSPI2_0 = 0,
    #[doc = "1: IOM/MSPI nCE group 64"]
    NCE64 = 1,
    #[doc = "2: CTIMER connection 14"]
    CT14 = 2,
    #[doc = "3: Configure as GPIO64"]
    GPIO64 = 3,
}
impl From<PAD64FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD64FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAD64FNCSEL`"]
pub type PAD64FNCSEL_R = crate::R<u8, PAD64FNCSEL_A>;
impl PAD64FNCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAD64FNCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAD64FNCSEL_A::MSPI2_0),
            1 => Val(PAD64FNCSEL_A::NCE64),
            2 => Val(PAD64FNCSEL_A::CT14),
            3 => Val(PAD64FNCSEL_A::GPIO64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI2_0`"]
    #[inline(always)]
    pub fn is_mspi2_0(&self) -> bool {
        *self == PAD64FNCSEL_A::MSPI2_0
    }
    #[doc = "Checks if the value of the field is `NCE64`"]
    #[inline(always)]
    pub fn is_nce64(&self) -> bool {
        *self == PAD64FNCSEL_A::NCE64
    }
    #[doc = "Checks if the value of the field is `CT14`"]
    #[inline(always)]
    pub fn is_ct14(&self) -> bool {
        *self == PAD64FNCSEL_A::CT14
    }
    #[doc = "Checks if the value of the field is `GPIO64`"]
    #[inline(always)]
    pub fn is_gpio64(&self) -> bool {
        *self == PAD64FNCSEL_A::GPIO64
    }
}
#[doc = "Write proxy for field `PAD64FNCSEL`"]
pub struct PAD64FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD64FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the MSPI2 0 signal"]
    #[inline(always)]
    pub fn mspi2_0(self) -> &'a mut W {
        self.variant(PAD64FNCSEL_A::MSPI2_0)
    }
    #[doc = "IOM/MSPI nCE group 64"]
    #[inline(always)]
    pub fn nce64(self) -> &'a mut W {
        self.variant(PAD64FNCSEL_A::NCE64)
    }
    #[doc = "CTIMER connection 14"]
    #[inline(always)]
    pub fn ct14(self) -> &'a mut W {
        self.variant(PAD64FNCSEL_A::CT14)
    }
    #[doc = "Configure as GPIO64"]
    #[inline(always)]
    pub fn gpio64(self) -> &'a mut W {
        self.variant(PAD64FNCSEL_A::GPIO64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 64 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD64STRNG_A {
    #[doc = "0: Low drive strength"]
    LOW = 0,
    #[doc = "1: High drive strength"]
    HIGH = 1,
}
impl From<PAD64STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD64STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD64STRNG`"]
pub type PAD64STRNG_R = crate::R<bool, PAD64STRNG_A>;
impl PAD64STRNG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD64STRNG_A {
        match self.bits {
            false => PAD64STRNG_A::LOW,
            true => PAD64STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PAD64STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PAD64STRNG_A::HIGH
    }
}
#[doc = "Write proxy for field `PAD64STRNG`"]
pub struct PAD64STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD64STRNG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD64STRNG_A::LOW)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD64STRNG_A::HIGH)
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
#[doc = "Pad 64 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD64INPEN_A {
    #[doc = "0: Pad input disabled"]
    DIS = 0,
    #[doc = "1: Pad input enabled"]
    EN = 1,
}
impl From<PAD64INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD64INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD64INPEN`"]
pub type PAD64INPEN_R = crate::R<bool, PAD64INPEN_A>;
impl PAD64INPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD64INPEN_A {
        match self.bits {
            false => PAD64INPEN_A::DIS,
            true => PAD64INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD64INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD64INPEN_A::EN
    }
}
#[doc = "Write proxy for field `PAD64INPEN`"]
pub struct PAD64INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD64INPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pad input disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD64INPEN_A::DIS)
    }
    #[doc = "Pad input enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD64INPEN_A::EN)
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
#[doc = "Pad 64 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD64PULL_A {
    #[doc = "0: Pullup disabled"]
    DIS = 0,
    #[doc = "1: Pullup enabled"]
    EN = 1,
}
impl From<PAD64PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD64PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PAD64PULL`"]
pub type PAD64PULL_R = crate::R<bool, PAD64PULL_A>;
impl PAD64PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD64PULL_A {
        match self.bits {
            false => PAD64PULL_A::DIS,
            true => PAD64PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PAD64PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PAD64PULL_A::EN
    }
}
#[doc = "Write proxy for field `PAD64PULL`"]
pub struct PAD64PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD64PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD64PULL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pullup disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD64PULL_A::DIS)
    }
    #[doc = "Pullup enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD64PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 67 function select"]
    #[inline(always)]
    pub fn pad67fncsel(&self) -> PAD67FNCSEL_R {
        PAD67FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 67 drive strength"]
    #[inline(always)]
    pub fn pad67strng(&self) -> PAD67STRNG_R {
        PAD67STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 67 input enable"]
    #[inline(always)]
    pub fn pad67inpen(&self) -> PAD67INPEN_R {
        PAD67INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 67 pullup enable"]
    #[inline(always)]
    pub fn pad67pull(&self) -> PAD67PULL_R {
        PAD67PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 66 function select"]
    #[inline(always)]
    pub fn pad66fncsel(&self) -> PAD66FNCSEL_R {
        PAD66FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 66 drive strength"]
    #[inline(always)]
    pub fn pad66strng(&self) -> PAD66STRNG_R {
        PAD66STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 66 input enable"]
    #[inline(always)]
    pub fn pad66inpen(&self) -> PAD66INPEN_R {
        PAD66INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 66 pullup enable"]
    #[inline(always)]
    pub fn pad66pull(&self) -> PAD66PULL_R {
        PAD66PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 65 function select"]
    #[inline(always)]
    pub fn pad65fncsel(&self) -> PAD65FNCSEL_R {
        PAD65FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 65 drive strength"]
    #[inline(always)]
    pub fn pad65strng(&self) -> PAD65STRNG_R {
        PAD65STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 65 input enable"]
    #[inline(always)]
    pub fn pad65inpen(&self) -> PAD65INPEN_R {
        PAD65INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 65 pullup enable"]
    #[inline(always)]
    pub fn pad65pull(&self) -> PAD65PULL_R {
        PAD65PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 64 function select"]
    #[inline(always)]
    pub fn pad64fncsel(&self) -> PAD64FNCSEL_R {
        PAD64FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 64 drive strength"]
    #[inline(always)]
    pub fn pad64strng(&self) -> PAD64STRNG_R {
        PAD64STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 64 input enable"]
    #[inline(always)]
    pub fn pad64inpen(&self) -> PAD64INPEN_R {
        PAD64INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 64 pullup enable"]
    #[inline(always)]
    pub fn pad64pull(&self) -> PAD64PULL_R {
        PAD64PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 67 function select"]
    #[inline(always)]
    pub fn pad67fncsel(&mut self) -> PAD67FNCSEL_W {
        PAD67FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 67 drive strength"]
    #[inline(always)]
    pub fn pad67strng(&mut self) -> PAD67STRNG_W {
        PAD67STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 67 input enable"]
    #[inline(always)]
    pub fn pad67inpen(&mut self) -> PAD67INPEN_W {
        PAD67INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 67 pullup enable"]
    #[inline(always)]
    pub fn pad67pull(&mut self) -> PAD67PULL_W {
        PAD67PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 66 function select"]
    #[inline(always)]
    pub fn pad66fncsel(&mut self) -> PAD66FNCSEL_W {
        PAD66FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 66 drive strength"]
    #[inline(always)]
    pub fn pad66strng(&mut self) -> PAD66STRNG_W {
        PAD66STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 66 input enable"]
    #[inline(always)]
    pub fn pad66inpen(&mut self) -> PAD66INPEN_W {
        PAD66INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 66 pullup enable"]
    #[inline(always)]
    pub fn pad66pull(&mut self) -> PAD66PULL_W {
        PAD66PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 65 function select"]
    #[inline(always)]
    pub fn pad65fncsel(&mut self) -> PAD65FNCSEL_W {
        PAD65FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 65 drive strength"]
    #[inline(always)]
    pub fn pad65strng(&mut self) -> PAD65STRNG_W {
        PAD65STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 65 input enable"]
    #[inline(always)]
    pub fn pad65inpen(&mut self) -> PAD65INPEN_W {
        PAD65INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 65 pullup enable"]
    #[inline(always)]
    pub fn pad65pull(&mut self) -> PAD65PULL_W {
        PAD65PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 64 function select"]
    #[inline(always)]
    pub fn pad64fncsel(&mut self) -> PAD64FNCSEL_W {
        PAD64FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 64 drive strength"]
    #[inline(always)]
    pub fn pad64strng(&mut self) -> PAD64STRNG_W {
        PAD64STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 64 input enable"]
    #[inline(always)]
    pub fn pad64inpen(&mut self) -> PAD64INPEN_W {
        PAD64INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 64 pullup enable"]
    #[inline(always)]
    pub fn pad64pull(&mut self) -> PAD64PULL_W {
        PAD64PULL_W { w: self }
    }
}
