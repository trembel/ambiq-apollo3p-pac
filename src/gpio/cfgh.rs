#[doc = "Reader of register CFGH"]
pub type R = crate::R<u32, super::CFGH>;
#[doc = "Writer for register CFGH"]
pub type W = crate::W<u32, super::CFGH>;
#[doc = "Register CFGH `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO63 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO63INTD_A {
    #[doc = "0: Applies when PAD63FNCSEL = NCE63 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD63FNCSEL = NCE63 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO63INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO63INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO63INTD`"]
pub type GPIO63INTD_R = crate::R<bool, GPIO63INTD_A>;
impl GPIO63INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO63INTD_A {
        match self.bits {
            false => GPIO63INTD_A::NCELOW,
            true => GPIO63INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO63INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO63INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO63INTD`"]
pub struct GPIO63INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO63INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO63INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD63FNCSEL = NCE63 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO63INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD63FNCSEL = NCE63 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO63INTD_A::NCEHIGH)
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
#[doc = "GPIO63 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO63OUTCFG_A {
    #[doc = "0: Applies when PAD63FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD63FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD63FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD63FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO63OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO63OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO63OUTCFG`"]
pub type GPIO63OUTCFG_R = crate::R<u8, GPIO63OUTCFG_A>;
impl GPIO63OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO63OUTCFG_A {
        match self.bits {
            0 => GPIO63OUTCFG_A::DIS,
            1 => GPIO63OUTCFG_A::PUSHPULL,
            2 => GPIO63OUTCFG_A::OD,
            3 => GPIO63OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO63OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO63OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO63OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO63OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO63OUTCFG`"]
pub struct GPIO63OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO63OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO63OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD63FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO63OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD63FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO63OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD63FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO63OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD63FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO63OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO63 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO63INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO63INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO63INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO63INCFG`"]
pub type GPIO63INCFG_R = crate::R<bool, GPIO63INCFG_A>;
impl GPIO63INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO63INCFG_A {
        match self.bits {
            false => GPIO63INCFG_A::READ,
            true => GPIO63INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO63INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO63INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO63INCFG`"]
pub struct GPIO63INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO63INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO63INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO63INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO63INCFG_A::RDZERO)
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
#[doc = "GPIO62 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO62INTD_A {
    #[doc = "0: Applies when PAD62FNCSEL = NCE62 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD62FNCSEL = NCE62 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO62INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO62INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO62INTD`"]
pub type GPIO62INTD_R = crate::R<bool, GPIO62INTD_A>;
impl GPIO62INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO62INTD_A {
        match self.bits {
            false => GPIO62INTD_A::NCELOW,
            true => GPIO62INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO62INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO62INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO62INTD`"]
pub struct GPIO62INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO62INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO62INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD62FNCSEL = NCE62 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO62INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD62FNCSEL = NCE62 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO62INTD_A::NCEHIGH)
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
#[doc = "GPIO62 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO62OUTCFG_A {
    #[doc = "0: Applies when PAD62FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD62FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD62FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD62FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO62OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO62OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO62OUTCFG`"]
pub type GPIO62OUTCFG_R = crate::R<u8, GPIO62OUTCFG_A>;
impl GPIO62OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO62OUTCFG_A {
        match self.bits {
            0 => GPIO62OUTCFG_A::DIS,
            1 => GPIO62OUTCFG_A::PUSHPULL,
            2 => GPIO62OUTCFG_A::OD,
            3 => GPIO62OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO62OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO62OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO62OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO62OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO62OUTCFG`"]
pub struct GPIO62OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO62OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO62OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD62FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO62OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD62FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO62OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD62FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO62OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD62FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO62OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO62 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO62INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO62INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO62INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO62INCFG`"]
pub type GPIO62INCFG_R = crate::R<bool, GPIO62INCFG_A>;
impl GPIO62INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO62INCFG_A {
        match self.bits {
            false => GPIO62INCFG_A::READ,
            true => GPIO62INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO62INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO62INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO62INCFG`"]
pub struct GPIO62INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO62INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO62INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO62INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO62INCFG_A::RDZERO)
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
#[doc = "GPIO61 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO61INTD_A {
    #[doc = "0: Applies when PAD61FNCSEL = NCE61 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD61FNCSEL = NCE61 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO61INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO61INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO61INTD`"]
pub type GPIO61INTD_R = crate::R<bool, GPIO61INTD_A>;
impl GPIO61INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO61INTD_A {
        match self.bits {
            false => GPIO61INTD_A::NCELOW,
            true => GPIO61INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO61INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO61INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO61INTD`"]
pub struct GPIO61INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO61INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO61INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD61FNCSEL = NCE61 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO61INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD61FNCSEL = NCE61 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO61INTD_A::NCEHIGH)
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
#[doc = "GPIO61 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO61OUTCFG_A {
    #[doc = "0: Applies when PAD61FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD61FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD61FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD61FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO61OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO61OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO61OUTCFG`"]
pub type GPIO61OUTCFG_R = crate::R<u8, GPIO61OUTCFG_A>;
impl GPIO61OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO61OUTCFG_A {
        match self.bits {
            0 => GPIO61OUTCFG_A::DIS,
            1 => GPIO61OUTCFG_A::PUSHPULL,
            2 => GPIO61OUTCFG_A::OD,
            3 => GPIO61OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO61OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO61OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO61OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO61OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO61OUTCFG`"]
pub struct GPIO61OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO61OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO61OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD61FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO61OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD61FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO61OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD61FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO61OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD61FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO61OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO61 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO61INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO61INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO61INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO61INCFG`"]
pub type GPIO61INCFG_R = crate::R<bool, GPIO61INCFG_A>;
impl GPIO61INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO61INCFG_A {
        match self.bits {
            false => GPIO61INCFG_A::READ,
            true => GPIO61INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO61INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO61INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO61INCFG`"]
pub struct GPIO61INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO61INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO61INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO61INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO61INCFG_A::RDZERO)
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
#[doc = "GPIO60 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO60INTD_A {
    #[doc = "0: Applies when PAD60FNCSEL = NCE60 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD60FNCSEL = NCE60 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO60INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO60INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO60INTD`"]
pub type GPIO60INTD_R = crate::R<bool, GPIO60INTD_A>;
impl GPIO60INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO60INTD_A {
        match self.bits {
            false => GPIO60INTD_A::NCELOW,
            true => GPIO60INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO60INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO60INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO60INTD`"]
pub struct GPIO60INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO60INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO60INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD60FNCSEL = NCE60 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO60INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD60FNCSEL = NCE60 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO60INTD_A::NCEHIGH)
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
#[doc = "GPIO60 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO60OUTCFG_A {
    #[doc = "0: Applies when PAD60FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD60FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD60FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD60FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO60OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO60OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO60OUTCFG`"]
pub type GPIO60OUTCFG_R = crate::R<u8, GPIO60OUTCFG_A>;
impl GPIO60OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO60OUTCFG_A {
        match self.bits {
            0 => GPIO60OUTCFG_A::DIS,
            1 => GPIO60OUTCFG_A::PUSHPULL,
            2 => GPIO60OUTCFG_A::OD,
            3 => GPIO60OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO60OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO60OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO60OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO60OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO60OUTCFG`"]
pub struct GPIO60OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO60OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO60OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD60FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO60OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD60FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO60OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD60FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO60OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD60FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO60OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO60 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO60INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO60INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO60INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO60INCFG`"]
pub type GPIO60INCFG_R = crate::R<bool, GPIO60INCFG_A>;
impl GPIO60INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO60INCFG_A {
        match self.bits {
            false => GPIO60INCFG_A::READ,
            true => GPIO60INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO60INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO60INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO60INCFG`"]
pub struct GPIO60INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO60INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO60INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO60INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO60INCFG_A::RDZERO)
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
#[doc = "GPIO59 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO59INTD_A {
    #[doc = "0: Applies when PAD59FNCSEL = NCE59 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD59FNCSEL = NCE59 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO59INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO59INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO59INTD`"]
pub type GPIO59INTD_R = crate::R<bool, GPIO59INTD_A>;
impl GPIO59INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO59INTD_A {
        match self.bits {
            false => GPIO59INTD_A::NCELOW,
            true => GPIO59INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO59INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO59INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO59INTD`"]
pub struct GPIO59INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO59INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO59INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD59FNCSEL = NCE59 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO59INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD59FNCSEL = NCE59 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO59INTD_A::NCEHIGH)
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
#[doc = "GPIO59 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO59OUTCFG_A {
    #[doc = "0: Applies when PAD59FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD59FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD59FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD59FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO59OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO59OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO59OUTCFG`"]
pub type GPIO59OUTCFG_R = crate::R<u8, GPIO59OUTCFG_A>;
impl GPIO59OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO59OUTCFG_A {
        match self.bits {
            0 => GPIO59OUTCFG_A::DIS,
            1 => GPIO59OUTCFG_A::PUSHPULL,
            2 => GPIO59OUTCFG_A::OD,
            3 => GPIO59OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO59OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO59OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO59OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO59OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO59OUTCFG`"]
pub struct GPIO59OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO59OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO59OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD59FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO59OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD59FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO59OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD59FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO59OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD59FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO59OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO59 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO59INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO59INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO59INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO59INCFG`"]
pub type GPIO59INCFG_R = crate::R<bool, GPIO59INCFG_A>;
impl GPIO59INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO59INCFG_A {
        match self.bits {
            false => GPIO59INCFG_A::READ,
            true => GPIO59INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO59INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO59INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO59INCFG`"]
pub struct GPIO59INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO59INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO59INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO59INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO59INCFG_A::RDZERO)
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
#[doc = "GPIO58 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO58INTD_A {
    #[doc = "0: Applies when PAD58FNCSEL = NCE58 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD58FNCSEL = NCE58 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO58INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO58INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO58INTD`"]
pub type GPIO58INTD_R = crate::R<bool, GPIO58INTD_A>;
impl GPIO58INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO58INTD_A {
        match self.bits {
            false => GPIO58INTD_A::NCELOW,
            true => GPIO58INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO58INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO58INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO58INTD`"]
pub struct GPIO58INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO58INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO58INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD58FNCSEL = NCE58 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO58INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD58FNCSEL = NCE58 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO58INTD_A::NCEHIGH)
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
#[doc = "GPIO58 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO58OUTCFG_A {
    #[doc = "0: Applies when PAD58FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD58FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD58FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD58FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO58OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO58OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO58OUTCFG`"]
pub type GPIO58OUTCFG_R = crate::R<u8, GPIO58OUTCFG_A>;
impl GPIO58OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO58OUTCFG_A {
        match self.bits {
            0 => GPIO58OUTCFG_A::DIS,
            1 => GPIO58OUTCFG_A::PUSHPULL,
            2 => GPIO58OUTCFG_A::OD,
            3 => GPIO58OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO58OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO58OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO58OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO58OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO58OUTCFG`"]
pub struct GPIO58OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO58OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO58OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD58FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO58OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD58FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO58OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD58FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO58OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD58FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO58OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO58 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO58INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO58INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO58INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO58INCFG`"]
pub type GPIO58INCFG_R = crate::R<bool, GPIO58INCFG_A>;
impl GPIO58INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO58INCFG_A {
        match self.bits {
            false => GPIO58INCFG_A::READ,
            true => GPIO58INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO58INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO58INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO58INCFG`"]
pub struct GPIO58INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO58INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO58INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO58INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO58INCFG_A::RDZERO)
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
#[doc = "GPIO57 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO57INTD_A {
    #[doc = "0: Applies when PAD57FNCSEL = NCE57 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD57FNCSEL = NCE57 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO57INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO57INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO57INTD`"]
pub type GPIO57INTD_R = crate::R<bool, GPIO57INTD_A>;
impl GPIO57INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO57INTD_A {
        match self.bits {
            false => GPIO57INTD_A::NCELOW,
            true => GPIO57INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO57INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO57INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO57INTD`"]
pub struct GPIO57INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO57INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO57INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD57FNCSEL = NCE57 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO57INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD57FNCSEL = NCE57 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO57INTD_A::NCEHIGH)
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
#[doc = "GPIO57 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO57OUTCFG_A {
    #[doc = "0: Applies when PAD57FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD57FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD57FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD57FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO57OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO57OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO57OUTCFG`"]
pub type GPIO57OUTCFG_R = crate::R<u8, GPIO57OUTCFG_A>;
impl GPIO57OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO57OUTCFG_A {
        match self.bits {
            0 => GPIO57OUTCFG_A::DIS,
            1 => GPIO57OUTCFG_A::PUSHPULL,
            2 => GPIO57OUTCFG_A::OD,
            3 => GPIO57OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO57OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO57OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO57OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO57OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO57OUTCFG`"]
pub struct GPIO57OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO57OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO57OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD57FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO57OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD57FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO57OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD57FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO57OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD57FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO57OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO57 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO57INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO57INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO57INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO57INCFG`"]
pub type GPIO57INCFG_R = crate::R<bool, GPIO57INCFG_A>;
impl GPIO57INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO57INCFG_A {
        match self.bits {
            false => GPIO57INCFG_A::READ,
            true => GPIO57INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO57INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO57INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO57INCFG`"]
pub struct GPIO57INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO57INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO57INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO57INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO57INCFG_A::RDZERO)
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
#[doc = "GPIO56 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO56INTD_A {
    #[doc = "0: Applies when PAD56FNCSEL = NCE56 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD56FNCSEL = NCE56 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO56INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO56INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO56INTD`"]
pub type GPIO56INTD_R = crate::R<bool, GPIO56INTD_A>;
impl GPIO56INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO56INTD_A {
        match self.bits {
            false => GPIO56INTD_A::NCELOW,
            true => GPIO56INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO56INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO56INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO56INTD`"]
pub struct GPIO56INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO56INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO56INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD56FNCSEL = NCE56 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO56INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD56FNCSEL = NCE56 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO56INTD_A::NCEHIGH)
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
#[doc = "GPIO56 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO56OUTCFG_A {
    #[doc = "0: Applies when PAD56FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD56FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD56FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD56FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO56OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO56OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO56OUTCFG`"]
pub type GPIO56OUTCFG_R = crate::R<u8, GPIO56OUTCFG_A>;
impl GPIO56OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO56OUTCFG_A {
        match self.bits {
            0 => GPIO56OUTCFG_A::DIS,
            1 => GPIO56OUTCFG_A::PUSHPULL,
            2 => GPIO56OUTCFG_A::OD,
            3 => GPIO56OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO56OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO56OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO56OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO56OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO56OUTCFG`"]
pub struct GPIO56OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO56OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO56OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD56FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO56OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD56FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO56OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD56FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO56OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD56FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO56OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO56 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO56INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO56INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO56INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO56INCFG`"]
pub type GPIO56INCFG_R = crate::R<bool, GPIO56INCFG_A>;
impl GPIO56INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO56INCFG_A {
        match self.bits {
            false => GPIO56INCFG_A::READ,
            true => GPIO56INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO56INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO56INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO56INCFG`"]
pub struct GPIO56INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO56INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO56INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO56INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO56INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO63 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio63intd(&self) -> GPIO63INTD_R {
        GPIO63INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO63 output configuration."]
    #[inline(always)]
    pub fn gpio63outcfg(&self) -> GPIO63OUTCFG_R {
        GPIO63OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO63 input enable."]
    #[inline(always)]
    pub fn gpio63incfg(&self) -> GPIO63INCFG_R {
        GPIO63INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO62 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio62intd(&self) -> GPIO62INTD_R {
        GPIO62INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO62 output configuration."]
    #[inline(always)]
    pub fn gpio62outcfg(&self) -> GPIO62OUTCFG_R {
        GPIO62OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO62 input enable."]
    #[inline(always)]
    pub fn gpio62incfg(&self) -> GPIO62INCFG_R {
        GPIO62INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO61 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio61intd(&self) -> GPIO61INTD_R {
        GPIO61INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO61 output configuration."]
    #[inline(always)]
    pub fn gpio61outcfg(&self) -> GPIO61OUTCFG_R {
        GPIO61OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO61 input enable."]
    #[inline(always)]
    pub fn gpio61incfg(&self) -> GPIO61INCFG_R {
        GPIO61INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO60 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio60intd(&self) -> GPIO60INTD_R {
        GPIO60INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO60 output configuration."]
    #[inline(always)]
    pub fn gpio60outcfg(&self) -> GPIO60OUTCFG_R {
        GPIO60OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO60 input enable."]
    #[inline(always)]
    pub fn gpio60incfg(&self) -> GPIO60INCFG_R {
        GPIO60INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO59 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio59intd(&self) -> GPIO59INTD_R {
        GPIO59INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO59 output configuration."]
    #[inline(always)]
    pub fn gpio59outcfg(&self) -> GPIO59OUTCFG_R {
        GPIO59OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO59 input enable."]
    #[inline(always)]
    pub fn gpio59incfg(&self) -> GPIO59INCFG_R {
        GPIO59INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO58 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio58intd(&self) -> GPIO58INTD_R {
        GPIO58INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO58 output configuration."]
    #[inline(always)]
    pub fn gpio58outcfg(&self) -> GPIO58OUTCFG_R {
        GPIO58OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO58 input enable."]
    #[inline(always)]
    pub fn gpio58incfg(&self) -> GPIO58INCFG_R {
        GPIO58INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO57 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio57intd(&self) -> GPIO57INTD_R {
        GPIO57INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO57 output configuration."]
    #[inline(always)]
    pub fn gpio57outcfg(&self) -> GPIO57OUTCFG_R {
        GPIO57OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO57 input enable."]
    #[inline(always)]
    pub fn gpio57incfg(&self) -> GPIO57INCFG_R {
        GPIO57INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO56 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio56intd(&self) -> GPIO56INTD_R {
        GPIO56INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO56 output configuration."]
    #[inline(always)]
    pub fn gpio56outcfg(&self) -> GPIO56OUTCFG_R {
        GPIO56OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO56 input enable."]
    #[inline(always)]
    pub fn gpio56incfg(&self) -> GPIO56INCFG_R {
        GPIO56INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO63 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio63intd(&mut self) -> GPIO63INTD_W {
        GPIO63INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO63 output configuration."]
    #[inline(always)]
    pub fn gpio63outcfg(&mut self) -> GPIO63OUTCFG_W {
        GPIO63OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO63 input enable."]
    #[inline(always)]
    pub fn gpio63incfg(&mut self) -> GPIO63INCFG_W {
        GPIO63INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO62 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio62intd(&mut self) -> GPIO62INTD_W {
        GPIO62INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO62 output configuration."]
    #[inline(always)]
    pub fn gpio62outcfg(&mut self) -> GPIO62OUTCFG_W {
        GPIO62OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO62 input enable."]
    #[inline(always)]
    pub fn gpio62incfg(&mut self) -> GPIO62INCFG_W {
        GPIO62INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO61 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio61intd(&mut self) -> GPIO61INTD_W {
        GPIO61INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO61 output configuration."]
    #[inline(always)]
    pub fn gpio61outcfg(&mut self) -> GPIO61OUTCFG_W {
        GPIO61OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO61 input enable."]
    #[inline(always)]
    pub fn gpio61incfg(&mut self) -> GPIO61INCFG_W {
        GPIO61INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO60 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio60intd(&mut self) -> GPIO60INTD_W {
        GPIO60INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO60 output configuration."]
    #[inline(always)]
    pub fn gpio60outcfg(&mut self) -> GPIO60OUTCFG_W {
        GPIO60OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO60 input enable."]
    #[inline(always)]
    pub fn gpio60incfg(&mut self) -> GPIO60INCFG_W {
        GPIO60INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO59 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio59intd(&mut self) -> GPIO59INTD_W {
        GPIO59INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO59 output configuration."]
    #[inline(always)]
    pub fn gpio59outcfg(&mut self) -> GPIO59OUTCFG_W {
        GPIO59OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO59 input enable."]
    #[inline(always)]
    pub fn gpio59incfg(&mut self) -> GPIO59INCFG_W {
        GPIO59INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO58 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio58intd(&mut self) -> GPIO58INTD_W {
        GPIO58INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO58 output configuration."]
    #[inline(always)]
    pub fn gpio58outcfg(&mut self) -> GPIO58OUTCFG_W {
        GPIO58OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO58 input enable."]
    #[inline(always)]
    pub fn gpio58incfg(&mut self) -> GPIO58INCFG_W {
        GPIO58INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO57 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio57intd(&mut self) -> GPIO57INTD_W {
        GPIO57INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO57 output configuration."]
    #[inline(always)]
    pub fn gpio57outcfg(&mut self) -> GPIO57OUTCFG_W {
        GPIO57OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO57 input enable."]
    #[inline(always)]
    pub fn gpio57incfg(&mut self) -> GPIO57INCFG_W {
        GPIO57INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO56 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio56intd(&mut self) -> GPIO56INTD_W {
        GPIO56INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO56 output configuration."]
    #[inline(always)]
    pub fn gpio56outcfg(&mut self) -> GPIO56OUTCFG_W {
        GPIO56OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO56 input enable."]
    #[inline(always)]
    pub fn gpio56incfg(&mut self) -> GPIO56INCFG_W {
        GPIO56INCFG_W { w: self }
    }
}
