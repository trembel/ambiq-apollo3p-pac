#[doc = "Reader of register CFGI"]
pub type R = crate::R<u32, super::CFGI>;
#[doc = "Writer for register CFGI"]
pub type W = crate::W<u32, super::CFGI>;
#[doc = "Register CFGI `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO71 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO71INTD_A {
    #[doc = "0: Applies when PAD71FNCSEL = NCE71 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD71FNCSEL = NCE71 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO71INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO71INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO71INTD`"]
pub type GPIO71INTD_R = crate::R<bool, GPIO71INTD_A>;
impl GPIO71INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO71INTD_A {
        match self.bits {
            false => GPIO71INTD_A::NCELOW,
            true => GPIO71INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO71INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO71INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO71INTD`"]
pub struct GPIO71INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO71INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO71INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD71FNCSEL = NCE71 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO71INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD71FNCSEL = NCE71 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO71INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "GPIO71 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO71OUTCFG_A {
    #[doc = "0: Applies when PAD71FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD71FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD71FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD71FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO71OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO71OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO71OUTCFG`"]
pub type GPIO71OUTCFG_R = crate::R<u8, GPIO71OUTCFG_A>;
impl GPIO71OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO71OUTCFG_A {
        match self.bits {
            0 => GPIO71OUTCFG_A::DIS,
            1 => GPIO71OUTCFG_A::PUSHPULL,
            2 => GPIO71OUTCFG_A::OD,
            3 => GPIO71OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO71OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO71OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO71OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO71OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO71OUTCFG`"]
pub struct GPIO71OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO71OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO71OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD71FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO71OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD71FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO71OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD71FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO71OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD71FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO71OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO71 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO71INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO71INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO71INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO71INCFG`"]
pub type GPIO71INCFG_R = crate::R<bool, GPIO71INCFG_A>;
impl GPIO71INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO71INCFG_A {
        match self.bits {
            false => GPIO71INCFG_A::READ,
            true => GPIO71INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO71INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO71INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO71INCFG`"]
pub struct GPIO71INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO71INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO71INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO71INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO71INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "GPIO70 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO70INTD_A {
    #[doc = "0: Applies when PAD70FNCSEL = NCE70 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD70FNCSEL = NCE70 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO70INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO70INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO70INTD`"]
pub type GPIO70INTD_R = crate::R<bool, GPIO70INTD_A>;
impl GPIO70INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO70INTD_A {
        match self.bits {
            false => GPIO70INTD_A::NCELOW,
            true => GPIO70INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO70INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO70INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO70INTD`"]
pub struct GPIO70INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO70INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO70INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD70FNCSEL = NCE70 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO70INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD70FNCSEL = NCE70 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO70INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "GPIO70 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO70OUTCFG_A {
    #[doc = "0: Applies when PAD70FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD70FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD70FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD70FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO70OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO70OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO70OUTCFG`"]
pub type GPIO70OUTCFG_R = crate::R<u8, GPIO70OUTCFG_A>;
impl GPIO70OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO70OUTCFG_A {
        match self.bits {
            0 => GPIO70OUTCFG_A::DIS,
            1 => GPIO70OUTCFG_A::PUSHPULL,
            2 => GPIO70OUTCFG_A::OD,
            3 => GPIO70OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO70OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO70OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO70OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO70OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO70OUTCFG`"]
pub struct GPIO70OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO70OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO70OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD70FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO70OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD70FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO70OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD70FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO70OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD70FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO70OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO70 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO70INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO70INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO70INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO70INCFG`"]
pub type GPIO70INCFG_R = crate::R<bool, GPIO70INCFG_A>;
impl GPIO70INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO70INCFG_A {
        match self.bits {
            false => GPIO70INCFG_A::READ,
            true => GPIO70INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO70INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO70INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO70INCFG`"]
pub struct GPIO70INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO70INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO70INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO70INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO70INCFG_A::RDZERO)
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
#[doc = "GPIO69 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO69INTD_A {
    #[doc = "0: Applies when PAD69FNCSEL = NCE69 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD69FNCSEL = NCE69 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO69INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO69INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO69INTD`"]
pub type GPIO69INTD_R = crate::R<bool, GPIO69INTD_A>;
impl GPIO69INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO69INTD_A {
        match self.bits {
            false => GPIO69INTD_A::NCELOW,
            true => GPIO69INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO69INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO69INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO69INTD`"]
pub struct GPIO69INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO69INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO69INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD69FNCSEL = NCE69 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO69INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD69FNCSEL = NCE69 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO69INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "GPIO69 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO69OUTCFG_A {
    #[doc = "0: Applies when PAD69FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD69FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD69FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD69FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO69OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO69OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO69OUTCFG`"]
pub type GPIO69OUTCFG_R = crate::R<u8, GPIO69OUTCFG_A>;
impl GPIO69OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO69OUTCFG_A {
        match self.bits {
            0 => GPIO69OUTCFG_A::DIS,
            1 => GPIO69OUTCFG_A::PUSHPULL,
            2 => GPIO69OUTCFG_A::OD,
            3 => GPIO69OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO69OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO69OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO69OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO69OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO69OUTCFG`"]
pub struct GPIO69OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO69OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO69OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD69FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO69OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD69FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO69OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD69FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO69OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD69FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO69OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO69 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO69INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO69INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO69INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO69INCFG`"]
pub type GPIO69INCFG_R = crate::R<bool, GPIO69INCFG_A>;
impl GPIO69INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO69INCFG_A {
        match self.bits {
            false => GPIO69INCFG_A::READ,
            true => GPIO69INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO69INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO69INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO69INCFG`"]
pub struct GPIO69INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO69INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO69INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO69INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO69INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "GPIO68 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO68INTD_A {
    #[doc = "0: Applies when PAD68FNCSEL = NCE68 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD68FNCSEL = NCE68 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO68INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO68INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO68INTD`"]
pub type GPIO68INTD_R = crate::R<bool, GPIO68INTD_A>;
impl GPIO68INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO68INTD_A {
        match self.bits {
            false => GPIO68INTD_A::NCELOW,
            true => GPIO68INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO68INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO68INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO68INTD`"]
pub struct GPIO68INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO68INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO68INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD68FNCSEL = NCE68 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO68INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD68FNCSEL = NCE68 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO68INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "GPIO68 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO68OUTCFG_A {
    #[doc = "0: Applies when PAD68FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD68FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD68FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD68FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO68OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO68OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO68OUTCFG`"]
pub type GPIO68OUTCFG_R = crate::R<u8, GPIO68OUTCFG_A>;
impl GPIO68OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO68OUTCFG_A {
        match self.bits {
            0 => GPIO68OUTCFG_A::DIS,
            1 => GPIO68OUTCFG_A::PUSHPULL,
            2 => GPIO68OUTCFG_A::OD,
            3 => GPIO68OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO68OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO68OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO68OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO68OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO68OUTCFG`"]
pub struct GPIO68OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO68OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO68OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD68FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO68OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD68FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO68OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD68FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO68OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD68FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO68OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO68 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO68INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO68INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO68INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO68INCFG`"]
pub type GPIO68INCFG_R = crate::R<bool, GPIO68INCFG_A>;
impl GPIO68INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO68INCFG_A {
        match self.bits {
            false => GPIO68INCFG_A::READ,
            true => GPIO68INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO68INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO68INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO68INCFG`"]
pub struct GPIO68INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO68INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO68INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO68INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO68INCFG_A::RDZERO)
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
#[doc = "GPIO67 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO67INTD_A {
    #[doc = "0: Applies when PAD67FNCSEL = NCE67 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD67FNCSEL = NCE67 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO67INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO67INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO67INTD`"]
pub type GPIO67INTD_R = crate::R<bool, GPIO67INTD_A>;
impl GPIO67INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO67INTD_A {
        match self.bits {
            false => GPIO67INTD_A::NCELOW,
            true => GPIO67INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO67INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO67INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO67INTD`"]
pub struct GPIO67INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO67INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO67INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD67FNCSEL = NCE67 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO67INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD67FNCSEL = NCE67 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO67INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "GPIO67 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO67OUTCFG_A {
    #[doc = "0: Applies when PAD67FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD67FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD67FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD67FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO67OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO67OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO67OUTCFG`"]
pub type GPIO67OUTCFG_R = crate::R<u8, GPIO67OUTCFG_A>;
impl GPIO67OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO67OUTCFG_A {
        match self.bits {
            0 => GPIO67OUTCFG_A::DIS,
            1 => GPIO67OUTCFG_A::PUSHPULL,
            2 => GPIO67OUTCFG_A::OD,
            3 => GPIO67OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO67OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO67OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO67OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO67OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO67OUTCFG`"]
pub struct GPIO67OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO67OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO67OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD67FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO67OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD67FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO67OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD67FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO67OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD67FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO67OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO67 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO67INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO67INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO67INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO67INCFG`"]
pub type GPIO67INCFG_R = crate::R<bool, GPIO67INCFG_A>;
impl GPIO67INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO67INCFG_A {
        match self.bits {
            false => GPIO67INCFG_A::READ,
            true => GPIO67INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO67INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO67INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO67INCFG`"]
pub struct GPIO67INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO67INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO67INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO67INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO67INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "GPIO66 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO66INTD_A {
    #[doc = "0: Applies when PAD66FNCSEL = NCE66 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD66FNCSEL = NCE66 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO66INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO66INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO66INTD`"]
pub type GPIO66INTD_R = crate::R<bool, GPIO66INTD_A>;
impl GPIO66INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO66INTD_A {
        match self.bits {
            false => GPIO66INTD_A::NCELOW,
            true => GPIO66INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO66INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO66INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO66INTD`"]
pub struct GPIO66INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO66INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO66INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD66FNCSEL = NCE66 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO66INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD66FNCSEL = NCE66 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO66INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "GPIO66 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO66OUTCFG_A {
    #[doc = "0: Applies when PAD66FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD66FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD66FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD66FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO66OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO66OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO66OUTCFG`"]
pub type GPIO66OUTCFG_R = crate::R<u8, GPIO66OUTCFG_A>;
impl GPIO66OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO66OUTCFG_A {
        match self.bits {
            0 => GPIO66OUTCFG_A::DIS,
            1 => GPIO66OUTCFG_A::PUSHPULL,
            2 => GPIO66OUTCFG_A::OD,
            3 => GPIO66OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO66OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO66OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO66OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO66OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO66OUTCFG`"]
pub struct GPIO66OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO66OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO66OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD66FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO66OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD66FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO66OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD66FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO66OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD66FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO66OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO66 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO66INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO66INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO66INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO66INCFG`"]
pub type GPIO66INCFG_R = crate::R<bool, GPIO66INCFG_A>;
impl GPIO66INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO66INCFG_A {
        match self.bits {
            false => GPIO66INCFG_A::READ,
            true => GPIO66INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO66INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO66INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO66INCFG`"]
pub struct GPIO66INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO66INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO66INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO66INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO66INCFG_A::RDZERO)
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
#[doc = "GPIO65 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO65INTD_A {
    #[doc = "0: Applies when PAD65FNCSEL = NCE65 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD65FNCSEL = NCE65 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO65INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO65INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO65INTD`"]
pub type GPIO65INTD_R = crate::R<bool, GPIO65INTD_A>;
impl GPIO65INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO65INTD_A {
        match self.bits {
            false => GPIO65INTD_A::NCELOW,
            true => GPIO65INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO65INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO65INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO65INTD`"]
pub struct GPIO65INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO65INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO65INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD65FNCSEL = NCE65 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO65INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD65FNCSEL = NCE65 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO65INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "GPIO65 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO65OUTCFG_A {
    #[doc = "0: Applies when PAD65FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD65FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD65FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD65FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO65OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO65OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO65OUTCFG`"]
pub type GPIO65OUTCFG_R = crate::R<u8, GPIO65OUTCFG_A>;
impl GPIO65OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO65OUTCFG_A {
        match self.bits {
            0 => GPIO65OUTCFG_A::DIS,
            1 => GPIO65OUTCFG_A::PUSHPULL,
            2 => GPIO65OUTCFG_A::OD,
            3 => GPIO65OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO65OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO65OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO65OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO65OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO65OUTCFG`"]
pub struct GPIO65OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO65OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO65OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD65FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO65OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD65FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO65OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD65FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO65OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD65FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO65OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO65 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO65INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO65INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO65INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO65INCFG`"]
pub type GPIO65INCFG_R = crate::R<bool, GPIO65INCFG_A>;
impl GPIO65INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO65INCFG_A {
        match self.bits {
            false => GPIO65INCFG_A::READ,
            true => GPIO65INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO65INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO65INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO65INCFG`"]
pub struct GPIO65INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO65INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO65INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO65INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO65INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIO64 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO64INTD_A {
    #[doc = "0: Applies when PAD64FNCSEL = NCE64 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD64FNCSEL = NCE64 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO64INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO64INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO64INTD`"]
pub type GPIO64INTD_R = crate::R<bool, GPIO64INTD_A>;
impl GPIO64INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO64INTD_A {
        match self.bits {
            false => GPIO64INTD_A::NCELOW,
            true => GPIO64INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO64INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO64INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO64INTD`"]
pub struct GPIO64INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO64INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO64INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD64FNCSEL = NCE64 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO64INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD64FNCSEL = NCE64 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO64INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "GPIO64 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO64OUTCFG_A {
    #[doc = "0: Applies when PAD64FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD64FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD64FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD64FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO64OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO64OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO64OUTCFG`"]
pub type GPIO64OUTCFG_R = crate::R<u8, GPIO64OUTCFG_A>;
impl GPIO64OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO64OUTCFG_A {
        match self.bits {
            0 => GPIO64OUTCFG_A::DIS,
            1 => GPIO64OUTCFG_A::PUSHPULL,
            2 => GPIO64OUTCFG_A::OD,
            3 => GPIO64OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO64OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO64OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO64OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO64OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO64OUTCFG`"]
pub struct GPIO64OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO64OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO64OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD64FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO64OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD64FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO64OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD64FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO64OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD64FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO64OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO64 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO64INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO64INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO64INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO64INCFG`"]
pub type GPIO64INCFG_R = crate::R<bool, GPIO64INCFG_A>;
impl GPIO64INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO64INCFG_A {
        match self.bits {
            false => GPIO64INCFG_A::READ,
            true => GPIO64INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO64INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO64INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO64INCFG`"]
pub struct GPIO64INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO64INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO64INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO64INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO64INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO71 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio71intd(&self) -> GPIO71INTD_R {
        GPIO71INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO71 output configuration."]
    #[inline(always)]
    pub fn gpio71outcfg(&self) -> GPIO71OUTCFG_R {
        GPIO71OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO71 input enable."]
    #[inline(always)]
    pub fn gpio71incfg(&self) -> GPIO71INCFG_R {
        GPIO71INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO70 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio70intd(&self) -> GPIO70INTD_R {
        GPIO70INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO70 output configuration."]
    #[inline(always)]
    pub fn gpio70outcfg(&self) -> GPIO70OUTCFG_R {
        GPIO70OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO70 input enable."]
    #[inline(always)]
    pub fn gpio70incfg(&self) -> GPIO70INCFG_R {
        GPIO70INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO69 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio69intd(&self) -> GPIO69INTD_R {
        GPIO69INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO69 output configuration."]
    #[inline(always)]
    pub fn gpio69outcfg(&self) -> GPIO69OUTCFG_R {
        GPIO69OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO69 input enable."]
    #[inline(always)]
    pub fn gpio69incfg(&self) -> GPIO69INCFG_R {
        GPIO69INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO68 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio68intd(&self) -> GPIO68INTD_R {
        GPIO68INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO68 output configuration."]
    #[inline(always)]
    pub fn gpio68outcfg(&self) -> GPIO68OUTCFG_R {
        GPIO68OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO68 input enable."]
    #[inline(always)]
    pub fn gpio68incfg(&self) -> GPIO68INCFG_R {
        GPIO68INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO67 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio67intd(&self) -> GPIO67INTD_R {
        GPIO67INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO67 output configuration."]
    #[inline(always)]
    pub fn gpio67outcfg(&self) -> GPIO67OUTCFG_R {
        GPIO67OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO67 input enable."]
    #[inline(always)]
    pub fn gpio67incfg(&self) -> GPIO67INCFG_R {
        GPIO67INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO66 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio66intd(&self) -> GPIO66INTD_R {
        GPIO66INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO66 output configuration."]
    #[inline(always)]
    pub fn gpio66outcfg(&self) -> GPIO66OUTCFG_R {
        GPIO66OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO66 input enable."]
    #[inline(always)]
    pub fn gpio66incfg(&self) -> GPIO66INCFG_R {
        GPIO66INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO65 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio65intd(&self) -> GPIO65INTD_R {
        GPIO65INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO65 output configuration."]
    #[inline(always)]
    pub fn gpio65outcfg(&self) -> GPIO65OUTCFG_R {
        GPIO65OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO65 input enable."]
    #[inline(always)]
    pub fn gpio65incfg(&self) -> GPIO65INCFG_R {
        GPIO65INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO64 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio64intd(&self) -> GPIO64INTD_R {
        GPIO64INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO64 output configuration."]
    #[inline(always)]
    pub fn gpio64outcfg(&self) -> GPIO64OUTCFG_R {
        GPIO64OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO64 input enable."]
    #[inline(always)]
    pub fn gpio64incfg(&self) -> GPIO64INCFG_R {
        GPIO64INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO71 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio71intd(&mut self) -> GPIO71INTD_W {
        GPIO71INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO71 output configuration."]
    #[inline(always)]
    pub fn gpio71outcfg(&mut self) -> GPIO71OUTCFG_W {
        GPIO71OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO71 input enable."]
    #[inline(always)]
    pub fn gpio71incfg(&mut self) -> GPIO71INCFG_W {
        GPIO71INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO70 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio70intd(&mut self) -> GPIO70INTD_W {
        GPIO70INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO70 output configuration."]
    #[inline(always)]
    pub fn gpio70outcfg(&mut self) -> GPIO70OUTCFG_W {
        GPIO70OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO70 input enable."]
    #[inline(always)]
    pub fn gpio70incfg(&mut self) -> GPIO70INCFG_W {
        GPIO70INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO69 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio69intd(&mut self) -> GPIO69INTD_W {
        GPIO69INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO69 output configuration."]
    #[inline(always)]
    pub fn gpio69outcfg(&mut self) -> GPIO69OUTCFG_W {
        GPIO69OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO69 input enable."]
    #[inline(always)]
    pub fn gpio69incfg(&mut self) -> GPIO69INCFG_W {
        GPIO69INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO68 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio68intd(&mut self) -> GPIO68INTD_W {
        GPIO68INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO68 output configuration."]
    #[inline(always)]
    pub fn gpio68outcfg(&mut self) -> GPIO68OUTCFG_W {
        GPIO68OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO68 input enable."]
    #[inline(always)]
    pub fn gpio68incfg(&mut self) -> GPIO68INCFG_W {
        GPIO68INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO67 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio67intd(&mut self) -> GPIO67INTD_W {
        GPIO67INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO67 output configuration."]
    #[inline(always)]
    pub fn gpio67outcfg(&mut self) -> GPIO67OUTCFG_W {
        GPIO67OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO67 input enable."]
    #[inline(always)]
    pub fn gpio67incfg(&mut self) -> GPIO67INCFG_W {
        GPIO67INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO66 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio66intd(&mut self) -> GPIO66INTD_W {
        GPIO66INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO66 output configuration."]
    #[inline(always)]
    pub fn gpio66outcfg(&mut self) -> GPIO66OUTCFG_W {
        GPIO66OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO66 input enable."]
    #[inline(always)]
    pub fn gpio66incfg(&mut self) -> GPIO66INCFG_W {
        GPIO66INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO65 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio65intd(&mut self) -> GPIO65INTD_W {
        GPIO65INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO65 output configuration."]
    #[inline(always)]
    pub fn gpio65outcfg(&mut self) -> GPIO65OUTCFG_W {
        GPIO65OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO65 input enable."]
    #[inline(always)]
    pub fn gpio65incfg(&mut self) -> GPIO65INCFG_W {
        GPIO65INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO64 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio64intd(&mut self) -> GPIO64INTD_W {
        GPIO64INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO64 output configuration."]
    #[inline(always)]
    pub fn gpio64outcfg(&mut self) -> GPIO64OUTCFG_W {
        GPIO64OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO64 input enable."]
    #[inline(always)]
    pub fn gpio64incfg(&mut self) -> GPIO64INCFG_W {
        GPIO64INCFG_W { w: self }
    }
}
