#[doc = "Reader of register CQCFG"]
pub type R = crate::R<u32, super::CQCFG>;
#[doc = "Writer for register CQCFG"]
pub type W = crate::W<u32, super::CQCFG>;
#[doc = "Register CQCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CQCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSPIFLGSEL_A {
    #[doc = "0: Selects MPSI0 as source of signals used in CGFLAG\\[11:8\\]."]
    MSPI0FLGSEL = 0,
    #[doc = "1: Selects MPSI1 as source of signals used in CGFLAG\\[11:8\\]."]
    MSPI1FLGSEL = 1,
    #[doc = "2: Selects MPSI2 as source of signals used in CGFLAG\\[11:8\\]."]
    MSPI2FLGSEL = 2,
}
impl From<MSPIFLGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSPIFLGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSPIFLGSEL`"]
pub type MSPIFLGSEL_R = crate::R<u8, MSPIFLGSEL_A>;
impl MSPIFLGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSPIFLGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSPIFLGSEL_A::MSPI0FLGSEL),
            1 => Val(MSPIFLGSEL_A::MSPI1FLGSEL),
            2 => Val(MSPIFLGSEL_A::MSPI2FLGSEL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MSPI0FLGSEL`"]
    #[inline(always)]
    pub fn is_mspi0flgsel(&self) -> bool {
        *self == MSPIFLGSEL_A::MSPI0FLGSEL
    }
    #[doc = "Checks if the value of the field is `MSPI1FLGSEL`"]
    #[inline(always)]
    pub fn is_mspi1flgsel(&self) -> bool {
        *self == MSPIFLGSEL_A::MSPI1FLGSEL
    }
    #[doc = "Checks if the value of the field is `MSPI2FLGSEL`"]
    #[inline(always)]
    pub fn is_mspi2flgsel(&self) -> bool {
        *self == MSPIFLGSEL_A::MSPI2FLGSEL
    }
}
#[doc = "Write proxy for field `MSPIFLGSEL`"]
pub struct MSPIFLGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSPIFLGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSPIFLGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selects MPSI0 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi0flgsel(self) -> &'a mut W {
        self.variant(MSPIFLGSEL_A::MSPI0FLGSEL)
    }
    #[doc = "Selects MPSI1 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi1flgsel(self) -> &'a mut W {
        self.variant(MSPIFLGSEL_A::MSPI1FLGSEL)
    }
    #[doc = "Selects MPSI2 as source of signals used in CGFLAG\\[11:8\\]."]
    #[inline(always)]
    pub fn mspi2flgsel(self) -> &'a mut W {
        self.variant(MSPIFLGSEL_A::MSPI2FLGSEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Sets the Priority of the command queue DMA request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQPRI_A {
    #[doc = "0: Low Priority (service as best effort)"]
    LOW = 0,
    #[doc = "1: High Priority (service immediately)"]
    HIGH = 1,
}
impl From<CQPRI_A> for bool {
    #[inline(always)]
    fn from(variant: CQPRI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CQPRI`"]
pub type CQPRI_R = crate::R<bool, CQPRI_A>;
impl CQPRI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQPRI_A {
        match self.bits {
            false => CQPRI_A::LOW,
            true => CQPRI_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CQPRI_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CQPRI_A::HIGH
    }
}
#[doc = "Write proxy for field `CQPRI`"]
pub struct CQPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPRI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQPRI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low Priority (service as best effort)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CQPRI_A::LOW)
    }
    #[doc = "High Priority (service immediately)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CQPRI_A::HIGH)
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
#[doc = "Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CQEN_A {
    #[doc = "0: Disable CQ Function"]
    DIS = 0,
    #[doc = "1: Enable CQ Function"]
    EN = 1,
}
impl From<CQEN_A> for bool {
    #[inline(always)]
    fn from(variant: CQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CQEN`"]
pub type CQEN_R = crate::R<bool, CQEN_A>;
impl CQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CQEN_A {
        match self.bits {
            false => CQEN_A::DIS,
            true => CQEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CQEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CQEN_A::EN
    }
}
#[doc = "Write proxy for field `CQEN`"]
pub struct CQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CQ Function"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CQEN_A::DIS)
    }
    #[doc = "Enable CQ Function"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CQEN_A::EN)
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
    #[doc = "Bits 2:3 - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
    #[inline(always)]
    pub fn mspiflgsel(&self) -> MSPIFLGSEL_R {
        MSPIFLGSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue DMA request"]
    #[inline(always)]
    pub fn cqpri(&self) -> CQPRI_R {
        CQPRI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline(always)]
    pub fn cqen(&self) -> CQEN_R {
        CQEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Selects the MPSI modules used for sourcing the CQFLAG \\[11:8\\]."]
    #[inline(always)]
    pub fn mspiflgsel(&mut self) -> MSPIFLGSEL_W {
        MSPIFLGSEL_W { w: self }
    }
    #[doc = "Bit 1 - Sets the Priority of the command queue DMA request"]
    #[inline(always)]
    pub fn cqpri(&mut self) -> CQPRI_W {
        CQPRI_W { w: self }
    }
    #[doc = "Bit 0 - Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[inline(always)]
    pub fn cqen(&mut self) -> CQEN_W {
        CQEN_W { w: self }
    }
}
