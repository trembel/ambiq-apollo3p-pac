#[doc = "Reader of register CFGG"]
pub type R = crate::R<u32, super::CFGG>;
#[doc = "Writer for register CFGG"]
pub type W = crate::W<u32, super::CFGG>;
#[doc = "Register CFGG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO55 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO55INTD_A {
    #[doc = "0: Applies when PAD55FNCSEL = NCE55 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD55FNCSEL = NCE55 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO55INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO55INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO55INTD`"]
pub type GPIO55INTD_R = crate::R<bool, GPIO55INTD_A>;
impl GPIO55INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO55INTD_A {
        match self.bits {
            false => GPIO55INTD_A::NCELOW,
            true => GPIO55INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO55INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO55INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO55INTD`"]
pub struct GPIO55INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO55INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO55INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD55FNCSEL = NCE55 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO55INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD55FNCSEL = NCE55 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO55INTD_A::NCEHIGH)
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
#[doc = "GPIO55 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO55OUTCFG_A {
    #[doc = "0: Applies when PAD55FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD55FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD55FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD55FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO55OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO55OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO55OUTCFG`"]
pub type GPIO55OUTCFG_R = crate::R<u8, GPIO55OUTCFG_A>;
impl GPIO55OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO55OUTCFG_A {
        match self.bits {
            0 => GPIO55OUTCFG_A::DIS,
            1 => GPIO55OUTCFG_A::PUSHPULL,
            2 => GPIO55OUTCFG_A::OD,
            3 => GPIO55OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO55OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO55OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO55OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO55OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO55OUTCFG`"]
pub struct GPIO55OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO55OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO55OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD55FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO55OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD55FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO55OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD55FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO55OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD55FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO55OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO55 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO55INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO55INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO55INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO55INCFG`"]
pub type GPIO55INCFG_R = crate::R<bool, GPIO55INCFG_A>;
impl GPIO55INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO55INCFG_A {
        match self.bits {
            false => GPIO55INCFG_A::READ,
            true => GPIO55INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO55INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO55INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO55INCFG`"]
pub struct GPIO55INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO55INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO55INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO55INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO55INCFG_A::RDZERO)
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
#[doc = "GPIO54 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO54INTD_A {
    #[doc = "0: Applies when PAD54FNCSEL = NCE54 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD54FNCSEL = NCE54 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO54INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO54INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO54INTD`"]
pub type GPIO54INTD_R = crate::R<bool, GPIO54INTD_A>;
impl GPIO54INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO54INTD_A {
        match self.bits {
            false => GPIO54INTD_A::NCELOW,
            true => GPIO54INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO54INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO54INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO54INTD`"]
pub struct GPIO54INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO54INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO54INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD54FNCSEL = NCE54 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO54INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD54FNCSEL = NCE54 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO54INTD_A::NCEHIGH)
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
#[doc = "GPIO54 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO54OUTCFG_A {
    #[doc = "0: Applies when PAD54FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD54FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD54FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD54FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO54OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO54OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO54OUTCFG`"]
pub type GPIO54OUTCFG_R = crate::R<u8, GPIO54OUTCFG_A>;
impl GPIO54OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO54OUTCFG_A {
        match self.bits {
            0 => GPIO54OUTCFG_A::DIS,
            1 => GPIO54OUTCFG_A::PUSHPULL,
            2 => GPIO54OUTCFG_A::OD,
            3 => GPIO54OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO54OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO54OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO54OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO54OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO54OUTCFG`"]
pub struct GPIO54OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO54OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO54OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD54FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO54OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD54FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO54OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD54FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO54OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD54FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO54OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO54 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO54INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO54INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO54INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO54INCFG`"]
pub type GPIO54INCFG_R = crate::R<bool, GPIO54INCFG_A>;
impl GPIO54INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO54INCFG_A {
        match self.bits {
            false => GPIO54INCFG_A::READ,
            true => GPIO54INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO54INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO54INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO54INCFG`"]
pub struct GPIO54INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO54INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO54INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO54INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO54INCFG_A::RDZERO)
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
#[doc = "GPIO53 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO53INTD_A {
    #[doc = "0: Applies when PAD53FNCSEL = NCE53 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD53FNCSEL = NCE53 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO53INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO53INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO53INTD`"]
pub type GPIO53INTD_R = crate::R<bool, GPIO53INTD_A>;
impl GPIO53INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO53INTD_A {
        match self.bits {
            false => GPIO53INTD_A::NCELOW,
            true => GPIO53INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO53INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO53INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO53INTD`"]
pub struct GPIO53INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO53INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO53INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD53FNCSEL = NCE53 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO53INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD53FNCSEL = NCE53 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO53INTD_A::NCEHIGH)
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
#[doc = "GPIO53 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO53OUTCFG_A {
    #[doc = "0: Applies when PAD53FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD53FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD53FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD53FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO53OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO53OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO53OUTCFG`"]
pub type GPIO53OUTCFG_R = crate::R<u8, GPIO53OUTCFG_A>;
impl GPIO53OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO53OUTCFG_A {
        match self.bits {
            0 => GPIO53OUTCFG_A::DIS,
            1 => GPIO53OUTCFG_A::PUSHPULL,
            2 => GPIO53OUTCFG_A::OD,
            3 => GPIO53OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO53OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO53OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO53OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO53OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO53OUTCFG`"]
pub struct GPIO53OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO53OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO53OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD53FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO53OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD53FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO53OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD53FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO53OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD53FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO53OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO53 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO53INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO53INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO53INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO53INCFG`"]
pub type GPIO53INCFG_R = crate::R<bool, GPIO53INCFG_A>;
impl GPIO53INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO53INCFG_A {
        match self.bits {
            false => GPIO53INCFG_A::READ,
            true => GPIO53INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO53INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO53INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO53INCFG`"]
pub struct GPIO53INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO53INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO53INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO53INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO53INCFG_A::RDZERO)
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
#[doc = "GPIO52 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO52INTD_A {
    #[doc = "0: Applies when PAD52FNCSEL = NCE52 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD52FNCSEL = NCE52 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO52INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO52INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO52INTD`"]
pub type GPIO52INTD_R = crate::R<bool, GPIO52INTD_A>;
impl GPIO52INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO52INTD_A {
        match self.bits {
            false => GPIO52INTD_A::NCELOW,
            true => GPIO52INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO52INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO52INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO52INTD`"]
pub struct GPIO52INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO52INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO52INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD52FNCSEL = NCE52 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO52INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD52FNCSEL = NCE52 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO52INTD_A::NCEHIGH)
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
#[doc = "GPIO52 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO52OUTCFG_A {
    #[doc = "0: Applies when PAD52FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD52FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD52FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD52FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO52OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO52OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO52OUTCFG`"]
pub type GPIO52OUTCFG_R = crate::R<u8, GPIO52OUTCFG_A>;
impl GPIO52OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO52OUTCFG_A {
        match self.bits {
            0 => GPIO52OUTCFG_A::DIS,
            1 => GPIO52OUTCFG_A::PUSHPULL,
            2 => GPIO52OUTCFG_A::OD,
            3 => GPIO52OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO52OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO52OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO52OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO52OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO52OUTCFG`"]
pub struct GPIO52OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO52OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO52OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD52FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO52OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD52FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO52OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD52FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO52OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD52FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO52OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO52 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO52INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO52INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO52INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO52INCFG`"]
pub type GPIO52INCFG_R = crate::R<bool, GPIO52INCFG_A>;
impl GPIO52INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO52INCFG_A {
        match self.bits {
            false => GPIO52INCFG_A::READ,
            true => GPIO52INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO52INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO52INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO52INCFG`"]
pub struct GPIO52INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO52INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO52INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO52INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO52INCFG_A::RDZERO)
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
#[doc = "GPIO51 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO51INTD_A {
    #[doc = "0: Applies when PAD51FNCSEL = NCE51 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD51FNCSEL = NCE51 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO51INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO51INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO51INTD`"]
pub type GPIO51INTD_R = crate::R<bool, GPIO51INTD_A>;
impl GPIO51INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO51INTD_A {
        match self.bits {
            false => GPIO51INTD_A::NCELOW,
            true => GPIO51INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO51INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO51INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO51INTD`"]
pub struct GPIO51INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO51INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO51INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD51FNCSEL = NCE51 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO51INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD51FNCSEL = NCE51 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO51INTD_A::NCEHIGH)
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
#[doc = "GPIO51 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO51OUTCFG_A {
    #[doc = "0: Applies when PAD51FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD51FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD51FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD51FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO51OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO51OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO51OUTCFG`"]
pub type GPIO51OUTCFG_R = crate::R<u8, GPIO51OUTCFG_A>;
impl GPIO51OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO51OUTCFG_A {
        match self.bits {
            0 => GPIO51OUTCFG_A::DIS,
            1 => GPIO51OUTCFG_A::PUSHPULL,
            2 => GPIO51OUTCFG_A::OD,
            3 => GPIO51OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO51OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO51OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO51OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO51OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO51OUTCFG`"]
pub struct GPIO51OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO51OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO51OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD51FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO51OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD51FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO51OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD51FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO51OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD51FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO51OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO51 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO51INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO51INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO51INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO51INCFG`"]
pub type GPIO51INCFG_R = crate::R<bool, GPIO51INCFG_A>;
impl GPIO51INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO51INCFG_A {
        match self.bits {
            false => GPIO51INCFG_A::READ,
            true => GPIO51INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO51INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO51INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO51INCFG`"]
pub struct GPIO51INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO51INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO51INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO51INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO51INCFG_A::RDZERO)
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
#[doc = "GPIO50 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO50INTD_A {
    #[doc = "0: Applies when PAD50FNCSEL = NCE50 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD50FNCSEL = NCE50 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO50INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO50INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO50INTD`"]
pub type GPIO50INTD_R = crate::R<bool, GPIO50INTD_A>;
impl GPIO50INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO50INTD_A {
        match self.bits {
            false => GPIO50INTD_A::NCELOW,
            true => GPIO50INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO50INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO50INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO50INTD`"]
pub struct GPIO50INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO50INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO50INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD50FNCSEL = NCE50 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO50INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD50FNCSEL = NCE50 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO50INTD_A::NCEHIGH)
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
#[doc = "GPIO50 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO50OUTCFG_A {
    #[doc = "0: Applies when PAD50FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD50FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD50FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD50FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO50OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO50OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO50OUTCFG`"]
pub type GPIO50OUTCFG_R = crate::R<u8, GPIO50OUTCFG_A>;
impl GPIO50OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO50OUTCFG_A {
        match self.bits {
            0 => GPIO50OUTCFG_A::DIS,
            1 => GPIO50OUTCFG_A::PUSHPULL,
            2 => GPIO50OUTCFG_A::OD,
            3 => GPIO50OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO50OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO50OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO50OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO50OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO50OUTCFG`"]
pub struct GPIO50OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO50OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO50OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD50FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO50OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD50FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO50OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD50FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO50OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD50FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO50OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO50 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO50INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO50INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO50INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO50INCFG`"]
pub type GPIO50INCFG_R = crate::R<bool, GPIO50INCFG_A>;
impl GPIO50INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO50INCFG_A {
        match self.bits {
            false => GPIO50INCFG_A::READ,
            true => GPIO50INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO50INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO50INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO50INCFG`"]
pub struct GPIO50INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO50INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO50INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO50INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO50INCFG_A::RDZERO)
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
#[doc = "GPIO49 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INTD_A {
    #[doc = "0: Applies when PAD49FNCSEL = NCE49 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD49FNCSEL = NCE49 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO49INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO49INTD`"]
pub type GPIO49INTD_R = crate::R<bool, GPIO49INTD_A>;
impl GPIO49INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INTD_A {
        match self.bits {
            false => GPIO49INTD_A::NCELOW,
            true => GPIO49INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO49INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO49INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO49INTD`"]
pub struct GPIO49INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD49FNCSEL = NCE49 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD49FNCSEL = NCE49 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::NCEHIGH)
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
#[doc = "GPIO49 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO49OUTCFG_A {
    #[doc = "0: Applies when PAD49FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD49FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD49FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD49FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO49OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO49OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO49OUTCFG`"]
pub type GPIO49OUTCFG_R = crate::R<u8, GPIO49OUTCFG_A>;
impl GPIO49OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49OUTCFG_A {
        match self.bits {
            0 => GPIO49OUTCFG_A::DIS,
            1 => GPIO49OUTCFG_A::PUSHPULL,
            2 => GPIO49OUTCFG_A::OD,
            3 => GPIO49OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO49OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO49OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO49OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO49OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO49OUTCFG`"]
pub struct GPIO49OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD49FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD49FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD49FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD49FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO49 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO49INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO49INCFG`"]
pub type GPIO49INCFG_R = crate::R<bool, GPIO49INCFG_A>;
impl GPIO49INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INCFG_A {
        match self.bits {
            false => GPIO49INCFG_A::READ,
            true => GPIO49INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO49INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO49INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO49INCFG`"]
pub struct GPIO49INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::RDZERO)
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
#[doc = "GPIO48 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INTD_A {
    #[doc = "0: Applies when PAD48FNCSEL = NCE48 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD48FNCSEL = NCE48 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO48INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO48INTD`"]
pub type GPIO48INTD_R = crate::R<bool, GPIO48INTD_A>;
impl GPIO48INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INTD_A {
        match self.bits {
            false => GPIO48INTD_A::NCELOW,
            true => GPIO48INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO48INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO48INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO48INTD`"]
pub struct GPIO48INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD48FNCSEL = NCE48 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD48FNCSEL = NCE48 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::NCEHIGH)
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
#[doc = "GPIO48 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO48OUTCFG_A {
    #[doc = "0: Applies when PAD48FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD48FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD48FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD48FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO48OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO48OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO48OUTCFG`"]
pub type GPIO48OUTCFG_R = crate::R<u8, GPIO48OUTCFG_A>;
impl GPIO48OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48OUTCFG_A {
        match self.bits {
            0 => GPIO48OUTCFG_A::DIS,
            1 => GPIO48OUTCFG_A::PUSHPULL,
            2 => GPIO48OUTCFG_A::OD,
            3 => GPIO48OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO48OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO48OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO48OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO48OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO48OUTCFG`"]
pub struct GPIO48OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD48FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD48FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD48FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD48FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO48 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO48INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO48INCFG`"]
pub type GPIO48INCFG_R = crate::R<bool, GPIO48INCFG_A>;
impl GPIO48INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INCFG_A {
        match self.bits {
            false => GPIO48INCFG_A::READ,
            true => GPIO48INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO48INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO48INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO48INCFG`"]
pub struct GPIO48INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO55 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio55intd(&self) -> GPIO55INTD_R {
        GPIO55INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO55 output configuration."]
    #[inline(always)]
    pub fn gpio55outcfg(&self) -> GPIO55OUTCFG_R {
        GPIO55OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO55 input enable."]
    #[inline(always)]
    pub fn gpio55incfg(&self) -> GPIO55INCFG_R {
        GPIO55INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO54 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio54intd(&self) -> GPIO54INTD_R {
        GPIO54INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO54 output configuration."]
    #[inline(always)]
    pub fn gpio54outcfg(&self) -> GPIO54OUTCFG_R {
        GPIO54OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO54 input enable."]
    #[inline(always)]
    pub fn gpio54incfg(&self) -> GPIO54INCFG_R {
        GPIO54INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO53 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio53intd(&self) -> GPIO53INTD_R {
        GPIO53INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO53 output configuration."]
    #[inline(always)]
    pub fn gpio53outcfg(&self) -> GPIO53OUTCFG_R {
        GPIO53OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO53 input enable."]
    #[inline(always)]
    pub fn gpio53incfg(&self) -> GPIO53INCFG_R {
        GPIO53INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO52 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio52intd(&self) -> GPIO52INTD_R {
        GPIO52INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO52 output configuration."]
    #[inline(always)]
    pub fn gpio52outcfg(&self) -> GPIO52OUTCFG_R {
        GPIO52OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO52 input enable."]
    #[inline(always)]
    pub fn gpio52incfg(&self) -> GPIO52INCFG_R {
        GPIO52INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO51 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio51intd(&self) -> GPIO51INTD_R {
        GPIO51INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO51 output configuration."]
    #[inline(always)]
    pub fn gpio51outcfg(&self) -> GPIO51OUTCFG_R {
        GPIO51OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO51 input enable."]
    #[inline(always)]
    pub fn gpio51incfg(&self) -> GPIO51INCFG_R {
        GPIO51INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO50 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio50intd(&self) -> GPIO50INTD_R {
        GPIO50INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO50 output configuration."]
    #[inline(always)]
    pub fn gpio50outcfg(&self) -> GPIO50OUTCFG_R {
        GPIO50OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO50 input enable."]
    #[inline(always)]
    pub fn gpio50incfg(&self) -> GPIO50INCFG_R {
        GPIO50INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO49 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio49intd(&self) -> GPIO49INTD_R {
        GPIO49INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&self) -> GPIO49OUTCFG_R {
        GPIO49OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&self) -> GPIO49INCFG_R {
        GPIO49INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio48intd(&self) -> GPIO48INTD_R {
        GPIO48INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&self) -> GPIO48OUTCFG_R {
        GPIO48OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&self) -> GPIO48INCFG_R {
        GPIO48INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO55 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio55intd(&mut self) -> GPIO55INTD_W {
        GPIO55INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO55 output configuration."]
    #[inline(always)]
    pub fn gpio55outcfg(&mut self) -> GPIO55OUTCFG_W {
        GPIO55OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO55 input enable."]
    #[inline(always)]
    pub fn gpio55incfg(&mut self) -> GPIO55INCFG_W {
        GPIO55INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO54 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio54intd(&mut self) -> GPIO54INTD_W {
        GPIO54INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO54 output configuration."]
    #[inline(always)]
    pub fn gpio54outcfg(&mut self) -> GPIO54OUTCFG_W {
        GPIO54OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO54 input enable."]
    #[inline(always)]
    pub fn gpio54incfg(&mut self) -> GPIO54INCFG_W {
        GPIO54INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO53 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio53intd(&mut self) -> GPIO53INTD_W {
        GPIO53INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO53 output configuration."]
    #[inline(always)]
    pub fn gpio53outcfg(&mut self) -> GPIO53OUTCFG_W {
        GPIO53OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO53 input enable."]
    #[inline(always)]
    pub fn gpio53incfg(&mut self) -> GPIO53INCFG_W {
        GPIO53INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO52 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio52intd(&mut self) -> GPIO52INTD_W {
        GPIO52INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO52 output configuration."]
    #[inline(always)]
    pub fn gpio52outcfg(&mut self) -> GPIO52OUTCFG_W {
        GPIO52OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO52 input enable."]
    #[inline(always)]
    pub fn gpio52incfg(&mut self) -> GPIO52INCFG_W {
        GPIO52INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO51 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio51intd(&mut self) -> GPIO51INTD_W {
        GPIO51INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO51 output configuration."]
    #[inline(always)]
    pub fn gpio51outcfg(&mut self) -> GPIO51OUTCFG_W {
        GPIO51OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO51 input enable."]
    #[inline(always)]
    pub fn gpio51incfg(&mut self) -> GPIO51INCFG_W {
        GPIO51INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO50 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio50intd(&mut self) -> GPIO50INTD_W {
        GPIO50INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO50 output configuration."]
    #[inline(always)]
    pub fn gpio50outcfg(&mut self) -> GPIO50OUTCFG_W {
        GPIO50OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO50 input enable."]
    #[inline(always)]
    pub fn gpio50incfg(&mut self) -> GPIO50INCFG_W {
        GPIO50INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO49 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio49intd(&mut self) -> GPIO49INTD_W {
        GPIO49INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&mut self) -> GPIO49OUTCFG_W {
        GPIO49OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&mut self) -> GPIO49INCFG_W {
        GPIO49INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio48intd(&mut self) -> GPIO48INTD_W {
        GPIO48INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&mut self) -> GPIO48OUTCFG_W {
        GPIO48OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&mut self) -> GPIO48INCFG_W {
        GPIO48INCFG_W { w: self }
    }
}
