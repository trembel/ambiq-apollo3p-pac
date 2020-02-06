#[doc = "Reader of register AUX0"]
pub type R = crate::R<u32, super::AUX0>;
#[doc = "Writer for register AUX0"]
pub type W = crate::W<u32, super::AUX0>;
#[doc = "Register AUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::AUX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer B0 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0EN23_A {
    #[doc = "1: Disable enhanced functions."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions."]
    EN = 0,
}
impl From<TMRB0EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0EN23`"]
pub type TMRB0EN23_R = crate::R<bool, TMRB0EN23_A>;
impl TMRB0EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0EN23_A {
        match self.bits {
            true => TMRB0EN23_A::DIS,
            false => TMRB0EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB0EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRB0EN23`"]
pub struct TMRB0EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0EN23_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0POL23_A {
    #[doc = "0: Upper output normal polarity"]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity."]
    INV = 1,
}
impl From<TMRB0POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0POL23`"]
pub type TMRB0POL23_R = crate::R<bool, TMRB0POL23_A>;
impl TMRB0POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0POL23_A {
        match self.bits {
            false => TMRB0POL23_A::NORM,
            true => TMRB0POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRB0POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRB0POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRB0POL23`"]
pub struct TMRB0POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity"]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB0POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB0POL23_A::INV)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Counter/Timer B0 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0TINV_A {
    #[doc = "0: Disable invert on trigger"]
    DIS = 0,
    #[doc = "1: Enable invert on trigger"]
    EN = 1,
}
impl From<TMRB0TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0TINV`"]
pub type TMRB0TINV_R = crate::R<bool, TMRB0TINV_A>;
impl TMRB0TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0TINV_A {
        match self.bits {
            false => TMRB0TINV_A::DIS,
            true => TMRB0TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB0TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRB0TINV`"]
pub struct TMRB0TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0TINV_A::EN)
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
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0NOSYNC_A {
    #[doc = "0: Synchronization on source clock"]
    DIS = 0,
    #[doc = "1: No synchronization on source clock"]
    NOSYNC = 1,
}
impl From<TMRB0NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB0NOSYNC`"]
pub type TMRB0NOSYNC_R = crate::R<bool, TMRB0NOSYNC_A>;
impl TMRB0NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0NOSYNC_A {
        match self.bits {
            false => TMRB0NOSYNC_A::DIS,
            true => TMRB0NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB0NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRB0NOSYNC`"]
pub struct TMRB0NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock"]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB0NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B0 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB0TRIG_A {
    #[doc = "0: Trigger source is disabled."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA0 OUT."]
    A0OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERB2 OUT."]
    B2OUT = 4,
    #[doc = "5: Trigger source is CTIMERB5 OUT."]
    B5OUT = 5,
    #[doc = "6: Trigger source is CTIMERA4 OUT."]
    A4OUT = 6,
    #[doc = "7: Trigger source is CTIMERB4 OUT."]
    B4OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERB7 OUT2."]
    B7OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERA2 OUT2."]
    A2OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB5 OUT2, dual edge."]
    B5OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA5 OUT2, dual edge."]
    A5OUT2DUAL = 15,
}
impl From<TMRB0TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB0TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB0TRIG`"]
pub type TMRB0TRIG_R = crate::R<u8, TMRB0TRIG_A>;
impl TMRB0TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0TRIG_A {
        match self.bits {
            0 => TMRB0TRIG_A::DIS,
            1 => TMRB0TRIG_A::A0OUT,
            2 => TMRB0TRIG_A::B3OUT,
            3 => TMRB0TRIG_A::A3OUT,
            4 => TMRB0TRIG_A::B2OUT,
            5 => TMRB0TRIG_A::B5OUT,
            6 => TMRB0TRIG_A::A4OUT,
            7 => TMRB0TRIG_A::B4OUT,
            8 => TMRB0TRIG_A::B3OUT2,
            9 => TMRB0TRIG_A::A3OUT2,
            10 => TMRB0TRIG_A::B7OUT2,
            11 => TMRB0TRIG_A::A2OUT2,
            12 => TMRB0TRIG_A::A6OUT2DUAL,
            13 => TMRB0TRIG_A::A7OUT2DUAL,
            14 => TMRB0TRIG_A::B5OUT2DUAL,
            15 => TMRB0TRIG_A::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB0TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        *self == TMRB0TRIG_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB0TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB0TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == TMRB0TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == TMRB0TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        *self == TMRB0TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        *self == TMRB0TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB0TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB0TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `B7OUT2`"]
    #[inline(always)]
    pub fn is_b7out2(&self) -> bool {
        *self == TMRB0TRIG_A::B7OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRB0TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB0TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB0TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRB0TRIG_A::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRB0TRIG_A::A5OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRB0TRIG`"]
pub struct TMRB0TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA0 OUT."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERB7 OUT2."]
    #[inline(always)]
    pub fn b7out2(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B7OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge."]
    #[inline(always)]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB0TRIG_A::A5OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TMRB0LMT`"]
pub type TMRB0LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRB0LMT`"]
pub struct TMRB0LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A0 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0EN23_A {
    #[doc = "1: Disable enhanced functions."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions."]
    EN = 0,
}
impl From<TMRA0EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0EN23`"]
pub type TMRA0EN23_R = crate::R<bool, TMRA0EN23_A>;
impl TMRA0EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0EN23_A {
        match self.bits {
            true => TMRA0EN23_A::DIS,
            false => TMRA0EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA0EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRA0EN23`"]
pub struct TMRA0EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0EN23_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Counter/Timer A0 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0POL23_A {
    #[doc = "0: Upper output normal polarity"]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity."]
    INV = 1,
}
impl From<TMRA0POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0POL23`"]
pub type TMRA0POL23_R = crate::R<bool, TMRA0POL23_A>;
impl TMRA0POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0POL23_A {
        match self.bits {
            false => TMRA0POL23_A::NORM,
            true => TMRA0POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRA0POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRA0POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRA0POL23`"]
pub struct TMRA0POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity"]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA0POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA0POL23_A::INV)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Counter/Timer A0 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0TINV_A {
    #[doc = "0: Disable invert on trigger"]
    DIS = 0,
    #[doc = "1: Enable invert on trigger"]
    EN = 1,
}
impl From<TMRA0TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0TINV`"]
pub type TMRA0TINV_R = crate::R<bool, TMRA0TINV_A>;
impl TMRA0TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0TINV_A {
        match self.bits {
            false => TMRA0TINV_A::DIS,
            true => TMRA0TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA0TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRA0TINV`"]
pub struct TMRA0TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0TINV_A::EN)
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
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0NOSYNC_A {
    #[doc = "0: Synchronization on source clock"]
    DIS = 0,
    #[doc = "1: No synchronization on source clock"]
    NOSYNC = 1,
}
impl From<TMRA0NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA0NOSYNC`"]
pub type TMRA0NOSYNC_R = crate::R<bool, TMRA0NOSYNC_A>;
impl TMRA0NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0NOSYNC_A {
        match self.bits {
            false => TMRA0NOSYNC_A::DIS,
            true => TMRA0NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA0NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRA0NOSYNC`"]
pub struct TMRA0NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock"]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA0NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A0 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA0TRIG_A {
    #[doc = "0: Trigger source is disabled."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB0 OUT."]
    B0OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA1 OUT."]
    A1OUT = 4,
    #[doc = "5: Trigger source is CTIMERB1 OUT."]
    B1OUT = 5,
    #[doc = "6: Trigger source is CTIMERA5 OUT."]
    A5OUT = 6,
    #[doc = "7: Trigger source is CTIMERB5 OUT."]
    B5OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERB6 OUT2."]
    B6OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERA2 OUT2."]
    A2OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge."]
    A4OUT2DUAL = 15,
}
impl From<TMRA0TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA0TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA0TRIG`"]
pub type TMRA0TRIG_R = crate::R<u8, TMRA0TRIG_A>;
impl TMRA0TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0TRIG_A {
        match self.bits {
            0 => TMRA0TRIG_A::DIS,
            1 => TMRA0TRIG_A::B0OUT,
            2 => TMRA0TRIG_A::B3OUT,
            3 => TMRA0TRIG_A::A3OUT,
            4 => TMRA0TRIG_A::A1OUT,
            5 => TMRA0TRIG_A::B1OUT,
            6 => TMRA0TRIG_A::A5OUT,
            7 => TMRA0TRIG_A::B5OUT,
            8 => TMRA0TRIG_A::B3OUT2,
            9 => TMRA0TRIG_A::A3OUT2,
            10 => TMRA0TRIG_A::B6OUT2,
            11 => TMRA0TRIG_A::A2OUT2,
            12 => TMRA0TRIG_A::A6OUT2DUAL,
            13 => TMRA0TRIG_A::A7OUT2DUAL,
            14 => TMRA0TRIG_A::B4OUT2DUAL,
            15 => TMRA0TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA0TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA0TRIG_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA0TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA0TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == TMRA0TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA0TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == TMRA0TRIG_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA0TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA0TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA0TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline(always)]
    pub fn is_b6out2(&self) -> bool {
        *self == TMRA0TRIG_A::B6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        *self == TMRA0TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA0TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA0TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        *self == TMRA0TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        *self == TMRA0TRIG_A::A4OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRA0TRIG`"]
pub struct TMRA0TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB0 OUT."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERB6 OUT2."]
    #[inline(always)]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B6OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA0TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `TMRA0LMT`"]
pub type TMRA0LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRA0LMT`"]
pub struct TMRA0LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B0 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb0en23(&self) -> TMRB0EN23_R {
        TMRB0EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb0pol23(&self) -> TMRB0POL23_R {
        TMRB0POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B0 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb0tinv(&self) -> TMRB0TINV_R {
        TMRB0TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb0nosync(&self) -> TMRB0NOSYNC_R {
        TMRB0NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B0 Trigger Select."]
    #[inline(always)]
    pub fn tmrb0trig(&self) -> TMRB0TRIG_R {
        TMRB0TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B0 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb0lmt(&self) -> TMRB0LMT_R {
        TMRB0LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A0 Upper compare enable."]
    #[inline(always)]
    pub fn tmra0en23(&self) -> TMRA0EN23_R {
        TMRA0EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A0 Upper output polarity"]
    #[inline(always)]
    pub fn tmra0pol23(&self) -> TMRA0POL23_R {
        TMRA0POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A0 Invert on trigger."]
    #[inline(always)]
    pub fn tmra0tinv(&self) -> TMRA0TINV_R {
        TMRA0TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra0nosync(&self) -> TMRA0NOSYNC_R {
        TMRA0NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A0 Trigger Select."]
    #[inline(always)]
    pub fn tmra0trig(&self) -> TMRA0TRIG_R {
        TMRA0TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A0 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra0lmt(&self) -> TMRA0LMT_R {
        TMRA0LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B0 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb0en23(&mut self) -> TMRB0EN23_W {
        TMRB0EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb0pol23(&mut self) -> TMRB0POL23_W {
        TMRB0POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B0 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb0tinv(&mut self) -> TMRB0TINV_W {
        TMRB0TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb0nosync(&mut self) -> TMRB0NOSYNC_W {
        TMRB0NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B0 Trigger Select."]
    #[inline(always)]
    pub fn tmrb0trig(&mut self) -> TMRB0TRIG_W {
        TMRB0TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B0 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb0lmt(&mut self) -> TMRB0LMT_W {
        TMRB0LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A0 Upper compare enable."]
    #[inline(always)]
    pub fn tmra0en23(&mut self) -> TMRA0EN23_W {
        TMRA0EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A0 Upper output polarity"]
    #[inline(always)]
    pub fn tmra0pol23(&mut self) -> TMRA0POL23_W {
        TMRA0POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A0 Invert on trigger."]
    #[inline(always)]
    pub fn tmra0tinv(&mut self) -> TMRA0TINV_W {
        TMRA0TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra0nosync(&mut self) -> TMRA0NOSYNC_W {
        TMRA0NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A0 Trigger Select."]
    #[inline(always)]
    pub fn tmra0trig(&mut self) -> TMRA0TRIG_W {
        TMRA0TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A0 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra0lmt(&mut self) -> TMRA0LMT_W {
        TMRA0LMT_W { w: self }
    }
}