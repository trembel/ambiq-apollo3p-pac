#[doc = "Reader of register CFGJ"]
pub type R = crate::R<u32, super::CFGJ>;
#[doc = "Writer for register CFGJ"]
pub type W = crate::W<u32, super::CFGJ>;
#[doc = "Register CFGJ `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO73 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO73INTD_A {
    #[doc = "0: Applies when PAD73FNCSEL = NCE73 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD73FNCSEL = NCE73 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO73INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO73INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO73INTD`"]
pub type GPIO73INTD_R = crate::R<bool, GPIO73INTD_A>;
impl GPIO73INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO73INTD_A {
        match self.bits {
            false => GPIO73INTD_A::NCELOW,
            true => GPIO73INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO73INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO73INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO73INTD`"]
pub struct GPIO73INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO73INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO73INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD73FNCSEL = NCE73 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO73INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD73FNCSEL = NCE73 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO73INTD_A::NCEHIGH)
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
#[doc = "GPIO73 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO73OUTCFG_A {
    #[doc = "0: Applies when PAD73FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD73FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD73FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD73FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO73OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO73OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO73OUTCFG`"]
pub type GPIO73OUTCFG_R = crate::R<u8, GPIO73OUTCFG_A>;
impl GPIO73OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO73OUTCFG_A {
        match self.bits {
            0 => GPIO73OUTCFG_A::DIS,
            1 => GPIO73OUTCFG_A::PUSHPULL,
            2 => GPIO73OUTCFG_A::OD,
            3 => GPIO73OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO73OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO73OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO73OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO73OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO73OUTCFG`"]
pub struct GPIO73OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO73OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO73OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD73FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO73OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD73FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO73OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD73FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO73OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD73FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO73OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO73 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO73INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO73INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO73INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO73INCFG`"]
pub type GPIO73INCFG_R = crate::R<bool, GPIO73INCFG_A>;
impl GPIO73INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO73INCFG_A {
        match self.bits {
            false => GPIO73INCFG_A::READ,
            true => GPIO73INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO73INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO73INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO73INCFG`"]
pub struct GPIO73INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO73INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO73INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO73INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO73INCFG_A::RDZERO)
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
#[doc = "GPIO72 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO72INTD_A {
    #[doc = "0: Applies when PAD72FNCSEL = NCE72 - nCE polarity active low"]
    NCELOW = 0,
    #[doc = "1: Applies when PAD72FNCSEL = NCE72 - nCE polarity active high"]
    NCEHIGH = 1,
}
impl From<GPIO72INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO72INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO72INTD`"]
pub type GPIO72INTD_R = crate::R<bool, GPIO72INTD_A>;
impl GPIO72INTD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO72INTD_A {
        match self.bits {
            false => GPIO72INTD_A::NCELOW,
            true => GPIO72INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        *self == GPIO72INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        *self == GPIO72INTD_A::NCEHIGH
    }
}
#[doc = "Write proxy for field `GPIO72INTD`"]
pub struct GPIO72INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO72INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO72INTD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Applies when PAD72FNCSEL = NCE72 - nCE polarity active low"]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO72INTD_A::NCELOW)
    }
    #[doc = "Applies when PAD72FNCSEL = NCE72 - nCE polarity active high"]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO72INTD_A::NCEHIGH)
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
#[doc = "GPIO72 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO72OUTCFG_A {
    #[doc = "0: Applies when PAD72FNCSEL = GPIO - Output disabled"]
    DIS = 0,
    #[doc = "1: Applies when PAD72FNCSEL = GPIO - Output is push-pull"]
    PUSHPULL = 1,
    #[doc = "2: Applies when PAD72FNCSEL = GPIO - Output is open drain"]
    OD = 2,
    #[doc = "3: Applies when PAD72FNCSEL = GPIO - Output is tri-state"]
    TS = 3,
}
impl From<GPIO72OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO72OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO72OUTCFG`"]
pub type GPIO72OUTCFG_R = crate::R<u8, GPIO72OUTCFG_A>;
impl GPIO72OUTCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO72OUTCFG_A {
        match self.bits {
            0 => GPIO72OUTCFG_A::DIS,
            1 => GPIO72OUTCFG_A::PUSHPULL,
            2 => GPIO72OUTCFG_A::OD,
            3 => GPIO72OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GPIO72OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == GPIO72OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        *self == GPIO72OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == GPIO72OUTCFG_A::TS
    }
}
#[doc = "Write proxy for field `GPIO72OUTCFG`"]
pub struct GPIO72OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO72OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO72OUTCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Applies when PAD72FNCSEL = GPIO - Output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO72OUTCFG_A::DIS)
    }
    #[doc = "Applies when PAD72FNCSEL = GPIO - Output is push-pull"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO72OUTCFG_A::PUSHPULL)
    }
    #[doc = "Applies when PAD72FNCSEL = GPIO - Output is open drain"]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO72OUTCFG_A::OD)
    }
    #[doc = "Applies when PAD72FNCSEL = GPIO - Output is tri-state"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO72OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO72 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO72INCFG_A {
    #[doc = "0: Read the GPIO pin data"]
    READ = 0,
    #[doc = "1: INTD = 0 - Read-back will always be zero"]
    RDZERO = 1,
}
impl From<GPIO72INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO72INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIO72INCFG`"]
pub type GPIO72INCFG_R = crate::R<bool, GPIO72INCFG_A>;
impl GPIO72INCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO72INCFG_A {
        match self.bits {
            false => GPIO72INCFG_A::READ,
            true => GPIO72INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == GPIO72INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        *self == GPIO72INCFG_A::RDZERO
    }
}
#[doc = "Write proxy for field `GPIO72INCFG`"]
pub struct GPIO72INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO72INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO72INCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read the GPIO pin data"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO72INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Read-back will always be zero"]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO72INCFG_A::RDZERO)
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
    #[doc = "Bit 7 - GPIO73 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio73intd(&self) -> GPIO73INTD_R {
        GPIO73INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO73 output configuration."]
    #[inline(always)]
    pub fn gpio73outcfg(&self) -> GPIO73OUTCFG_R {
        GPIO73OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO73 input enable."]
    #[inline(always)]
    pub fn gpio73incfg(&self) -> GPIO73INCFG_R {
        GPIO73INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO72 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio72intd(&self) -> GPIO72INTD_R {
        GPIO72INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO72 output configuration."]
    #[inline(always)]
    pub fn gpio72outcfg(&self) -> GPIO72OUTCFG_R {
        GPIO72OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO72 input enable."]
    #[inline(always)]
    pub fn gpio72incfg(&self) -> GPIO72INCFG_R {
        GPIO72INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - GPIO73 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio73intd(&mut self) -> GPIO73INTD_W {
        GPIO73INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO73 output configuration."]
    #[inline(always)]
    pub fn gpio73outcfg(&mut self) -> GPIO73OUTCFG_W {
        GPIO73OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO73 input enable."]
    #[inline(always)]
    pub fn gpio73incfg(&mut self) -> GPIO73INCFG_W {
        GPIO73INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO72 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio72intd(&mut self) -> GPIO72INTD_W {
        GPIO72INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO72 output configuration."]
    #[inline(always)]
    pub fn gpio72outcfg(&mut self) -> GPIO72OUTCFG_W {
        GPIO72OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO72 input enable."]
    #[inline(always)]
    pub fn gpio72incfg(&mut self) -> GPIO72INCFG_W {
        GPIO72INCFG_W { w: self }
    }
}
