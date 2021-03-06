#[doc = "Reader of register MEMPWREN"]
pub type R = crate::R<u32, super::MEMPWREN>;
#[doc = "Writer for register MEMPWREN"]
pub type W = crate::W<u32, super::MEMPWREN>;
#[doc = "Register MEMPWREN `reset()`'s with value 0xc000_7fff"]
impl crate::ResetValue for super::MEMPWREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_7fff
    }
}
#[doc = "Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 2, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB2_A {
    #[doc = "1: Power up Cache Bank 2"]
    EN = 1,
    #[doc = "0: Power down Cache Bank 2"]
    DIS = 0,
}
impl From<CACHEB2_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB2`"]
pub type CACHEB2_R = crate::R<bool, CACHEB2_A>;
impl CACHEB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB2_A {
        match self.bits {
            true => CACHEB2_A::EN,
            false => CACHEB2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB2_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB2`"]
pub struct CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up Cache Bank 2"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB2_A::EN)
    }
    #[doc = "Power down Cache Bank 2"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB2_A::DIS)
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
#[doc = "Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 0, cache has to be enabled and this bit has to be set.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEB0_A {
    #[doc = "1: Power up Cache Bank 0"]
    EN = 1,
    #[doc = "0: Power down Cache Bank 0"]
    DIS = 0,
}
impl From<CACHEB0_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEB0`"]
pub type CACHEB0_R = crate::R<bool, CACHEB0_A>;
impl CACHEB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEB0_A {
        match self.bits {
            true => CACHEB0_A::EN,
            false => CACHEB0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == CACHEB0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == CACHEB0_A::DIS
    }
}
#[doc = "Write proxy for field `CACHEB0`"]
pub struct CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up Cache Bank 0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CACHEB0_A::EN)
    }
    #[doc = "Power down Cache Bank 0"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CACHEB0_A::DIS)
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
#[doc = "Power up FLASH group 1 (1MB-2MB)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH1_A {
    #[doc = "1: Power up FLASH group 1 (1MB-2MB)"]
    EN = 1,
    #[doc = "0: Power down FLASH group 1 (1MB-2MB)"]
    DIS = 0,
}
impl From<FLASH1_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH1`"]
pub type FLASH1_R = crate::R<bool, FLASH1_A>;
impl FLASH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH1_A {
        match self.bits {
            true => FLASH1_A::EN,
            false => FLASH1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH1_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH1`"]
pub struct FLASH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up FLASH group 1 (1MB-2MB)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH1_A::EN)
    }
    #[doc = "Power down FLASH group 1 (1MB-2MB)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH1_A::DIS)
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
#[doc = "Power up FLASH group 0 (0MB-1MB)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH0_A {
    #[doc = "1: Power up FLASH group 0 (0MB-1MB)"]
    EN = 1,
    #[doc = "0: Power down FLASH group 0 (0MB-1MB)"]
    DIS = 0,
}
impl From<FLASH0_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLASH0`"]
pub type FLASH0_R = crate::R<bool, FLASH0_A>;
impl FLASH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH0_A {
        match self.bits {
            true => FLASH0_A::EN,
            false => FLASH0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FLASH0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FLASH0_A::DIS
    }
}
#[doc = "Write proxy for field `FLASH0`"]
pub struct FLASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power up FLASH group 0 (0MB-1MB)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FLASH0_A::EN)
    }
    #[doc = "Power down FLASH group 0 (0MB-1MB)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FLASH0_A::DIS)
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
#[doc = "Power up SRAM groups\n\nValue on reset: 1023"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAM_A {
    #[doc = "0: Do not power ON any of the SRAM banks"]
    NONE = 0,
    #[doc = "1: Power ON only SRAM 64KB group0 (addr: 0x10010000 - 0x1001FFFF)"]
    GROUP0 = 1,
    #[doc = "2: Power ON only SRAM 64KB group1 (addr: 0x10020000 - 0x1002FFFF)"]
    GROUP1 = 2,
    #[doc = "4: Power ON only SRAM 64KB group2 (addr: 0x10030000 - 0x1003FFFF)"]
    GROUP2 = 4,
    #[doc = "8: Power ON only SRAM 64KB group3 (addr: 0x10040000 - 0x1004FFFF)"]
    GROUP3 = 8,
    #[doc = "16: Power ON only SRAM 64KB group4 (addr: 0x10050000 - 0x1005FFFF)"]
    GROUP4 = 16,
    #[doc = "32: Power ON only SRAM 64KB group5 (addr: 0x10060000 - 0x1006FFFF)"]
    GROUP5 = 32,
    #[doc = "64: Power ON only SRAM 64KB group6 (addr: 0x10070000 - 0x1007FFFF)"]
    GROUP6 = 64,
    #[doc = "128: Power ON only SRAM 64KB group7 (addr: 0x10080000 - 0x1008FFFF)"]
    GROUP7 = 128,
    #[doc = "256: Power ON only SRAM 96KB group8 (addr: 0x10090000 - 0x100A7FFF)"]
    GROUP8 = 256,
    #[doc = "512: Power ON only SRAM 96KB group9 (addr: 0x100A8000 - 0x100BFFFF)"]
    GROUP9 = 512,
    #[doc = "3: Power ON only lower 128k (addr: 0x10010000 - 0x1002FFFF)"]
    SRAM128K = 3,
    #[doc = "15: Power ON only lower 256k (addr: 0x10010000 - 0x1004FFFF)"]
    SRAM256K = 15,
    #[doc = "255: Power ON only lower 512k (addr: 0x10010000 - 0x1008FFFF)"]
    SRAM512K = 255,
    #[doc = "1023: All SRAM banks (704K) powered ON (addr: 0x10010000 - 0x100BFFFF)"]
    ALL = 1023,
}
impl From<SRAM_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAM`"]
pub type SRAM_R = crate::R<u16, SRAM_A>;
impl SRAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SRAM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAM_A::NONE),
            1 => Val(SRAM_A::GROUP0),
            2 => Val(SRAM_A::GROUP1),
            4 => Val(SRAM_A::GROUP2),
            8 => Val(SRAM_A::GROUP3),
            16 => Val(SRAM_A::GROUP4),
            32 => Val(SRAM_A::GROUP5),
            64 => Val(SRAM_A::GROUP6),
            128 => Val(SRAM_A::GROUP7),
            256 => Val(SRAM_A::GROUP8),
            512 => Val(SRAM_A::GROUP9),
            3 => Val(SRAM_A::SRAM128K),
            15 => Val(SRAM_A::SRAM256K),
            255 => Val(SRAM_A::SRAM512K),
            1023 => Val(SRAM_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRAM_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == SRAM_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == SRAM_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `GROUP2`"]
    #[inline(always)]
    pub fn is_group2(&self) -> bool {
        *self == SRAM_A::GROUP2
    }
    #[doc = "Checks if the value of the field is `GROUP3`"]
    #[inline(always)]
    pub fn is_group3(&self) -> bool {
        *self == SRAM_A::GROUP3
    }
    #[doc = "Checks if the value of the field is `GROUP4`"]
    #[inline(always)]
    pub fn is_group4(&self) -> bool {
        *self == SRAM_A::GROUP4
    }
    #[doc = "Checks if the value of the field is `GROUP5`"]
    #[inline(always)]
    pub fn is_group5(&self) -> bool {
        *self == SRAM_A::GROUP5
    }
    #[doc = "Checks if the value of the field is `GROUP6`"]
    #[inline(always)]
    pub fn is_group6(&self) -> bool {
        *self == SRAM_A::GROUP6
    }
    #[doc = "Checks if the value of the field is `GROUP7`"]
    #[inline(always)]
    pub fn is_group7(&self) -> bool {
        *self == SRAM_A::GROUP7
    }
    #[doc = "Checks if the value of the field is `GROUP8`"]
    #[inline(always)]
    pub fn is_group8(&self) -> bool {
        *self == SRAM_A::GROUP8
    }
    #[doc = "Checks if the value of the field is `GROUP9`"]
    #[inline(always)]
    pub fn is_group9(&self) -> bool {
        *self == SRAM_A::GROUP9
    }
    #[doc = "Checks if the value of the field is `SRAM128K`"]
    #[inline(always)]
    pub fn is_sram128k(&self) -> bool {
        *self == SRAM_A::SRAM128K
    }
    #[doc = "Checks if the value of the field is `SRAM256K`"]
    #[inline(always)]
    pub fn is_sram256k(&self) -> bool {
        *self == SRAM_A::SRAM256K
    }
    #[doc = "Checks if the value of the field is `SRAM512K`"]
    #[inline(always)]
    pub fn is_sram512k(&self) -> bool {
        *self == SRAM_A::SRAM512K
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == SRAM_A::ALL
    }
}
#[doc = "Write proxy for field `SRAM`"]
pub struct SRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not power ON any of the SRAM banks"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRAM_A::NONE)
    }
    #[doc = "Power ON only SRAM 64KB group0 (addr: 0x10010000 - 0x1001FFFF)"]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP0)
    }
    #[doc = "Power ON only SRAM 64KB group1 (addr: 0x10020000 - 0x1002FFFF)"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP1)
    }
    #[doc = "Power ON only SRAM 64KB group2 (addr: 0x10030000 - 0x1003FFFF)"]
    #[inline(always)]
    pub fn group2(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP2)
    }
    #[doc = "Power ON only SRAM 64KB group3 (addr: 0x10040000 - 0x1004FFFF)"]
    #[inline(always)]
    pub fn group3(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP3)
    }
    #[doc = "Power ON only SRAM 64KB group4 (addr: 0x10050000 - 0x1005FFFF)"]
    #[inline(always)]
    pub fn group4(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP4)
    }
    #[doc = "Power ON only SRAM 64KB group5 (addr: 0x10060000 - 0x1006FFFF)"]
    #[inline(always)]
    pub fn group5(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP5)
    }
    #[doc = "Power ON only SRAM 64KB group6 (addr: 0x10070000 - 0x1007FFFF)"]
    #[inline(always)]
    pub fn group6(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP6)
    }
    #[doc = "Power ON only SRAM 64KB group7 (addr: 0x10080000 - 0x1008FFFF)"]
    #[inline(always)]
    pub fn group7(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP7)
    }
    #[doc = "Power ON only SRAM 96KB group8 (addr: 0x10090000 - 0x100A7FFF)"]
    #[inline(always)]
    pub fn group8(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP8)
    }
    #[doc = "Power ON only SRAM 96KB group9 (addr: 0x100A8000 - 0x100BFFFF)"]
    #[inline(always)]
    pub fn group9(self) -> &'a mut W {
        self.variant(SRAM_A::GROUP9)
    }
    #[doc = "Power ON only lower 128k (addr: 0x10010000 - 0x1002FFFF)"]
    #[inline(always)]
    pub fn sram128k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM128K)
    }
    #[doc = "Power ON only lower 256k (addr: 0x10010000 - 0x1004FFFF)"]
    #[inline(always)]
    pub fn sram256k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM256K)
    }
    #[doc = "Power ON only lower 512k (addr: 0x10010000 - 0x1008FFFF)"]
    #[inline(always)]
    pub fn sram512k(self) -> &'a mut W {
        self.variant(SRAM_A::SRAM512K)
    }
    #[doc = "All SRAM banks (704K) powered ON (addr: 0x10010000 - 0x100BFFFF)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAM_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | (((value as u32) & 0x03ff) << 3);
        self.w
    }
}
#[doc = "Power up DTCM\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTCM_A {
    #[doc = "0: Do not enable power to any DTCMs"]
    NONE = 0,
    #[doc = "1: Power ON only 8KB GROUP0_DTCM0 (0 - 8KB, addr: 0x10000000 - 0x10001FFF)"]
    GROUP0DTCM0 = 1,
    #[doc = "2: Power ON only 24KB GROUP0_DTCM1 (8KB - 32KB, addr: 0x10002000 - 0x10007FFF)"]
    GROUP0DTCM1 = 2,
    #[doc = "3: Power ON only DTCMs in 32KB group0 (0 - 32KB, addr: 0x10000000 - 0x10007FFF)"]
    GROUP0 = 3,
    #[doc = "4: Power ON only DTCMs in 32KB group1 (32KB - 64KB, addr: 0x10008000 - 0x1000FFFF)"]
    GROUP1 = 4,
    #[doc = "7: Power ON all DTCMs (0 - 64KB, addr: 0x10000000 - 0x1000FFFF)"]
    ALL = 7,
}
impl From<DTCM_A> for u8 {
    #[inline(always)]
    fn from(variant: DTCM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTCM`"]
pub type DTCM_R = crate::R<u8, DTCM_A>;
impl DTCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTCM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTCM_A::NONE),
            1 => Val(DTCM_A::GROUP0DTCM0),
            2 => Val(DTCM_A::GROUP0DTCM1),
            3 => Val(DTCM_A::GROUP0),
            4 => Val(DTCM_A::GROUP1),
            7 => Val(DTCM_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DTCM_A::NONE
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM0`"]
    #[inline(always)]
    pub fn is_group0dtcm0(&self) -> bool {
        *self == DTCM_A::GROUP0DTCM0
    }
    #[doc = "Checks if the value of the field is `GROUP0DTCM1`"]
    #[inline(always)]
    pub fn is_group0dtcm1(&self) -> bool {
        *self == DTCM_A::GROUP0DTCM1
    }
    #[doc = "Checks if the value of the field is `GROUP0`"]
    #[inline(always)]
    pub fn is_group0(&self) -> bool {
        *self == DTCM_A::GROUP0
    }
    #[doc = "Checks if the value of the field is `GROUP1`"]
    #[inline(always)]
    pub fn is_group1(&self) -> bool {
        *self == DTCM_A::GROUP1
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == DTCM_A::ALL
    }
}
#[doc = "Write proxy for field `DTCM`"]
pub struct DTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not enable power to any DTCMs"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DTCM_A::NONE)
    }
    #[doc = "Power ON only 8KB GROUP0_DTCM0 (0 - 8KB, addr: 0x10000000 - 0x10001FFF)"]
    #[inline(always)]
    pub fn group0dtcm0(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0DTCM0)
    }
    #[doc = "Power ON only 24KB GROUP0_DTCM1 (8KB - 32KB, addr: 0x10002000 - 0x10007FFF)"]
    #[inline(always)]
    pub fn group0dtcm1(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0DTCM1)
    }
    #[doc = "Power ON only DTCMs in 32KB group0 (0 - 32KB, addr: 0x10000000 - 0x10007FFF)"]
    #[inline(always)]
    pub fn group0(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP0)
    }
    #[doc = "Power ON only DTCMs in 32KB group1 (32KB - 64KB, addr: 0x10008000 - 0x1000FFFF)"]
    #[inline(always)]
    pub fn group1(self) -> &'a mut W {
        self.variant(DTCM_A::GROUP1)
    }
    #[doc = "Power ON all DTCMs (0 - 64KB, addr: 0x10000000 - 0x1000FFFF)"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(DTCM_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb2(&self) -> CACHEB2_R {
        CACHEB2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb0(&self) -> CACHEB0_R {
        CACHEB0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Power up FLASH group 1 (1MB-2MB)"]
    #[inline(always)]
    pub fn flash1(&self) -> FLASH1_R {
        FLASH1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Power up FLASH group 0 (0MB-1MB)"]
    #[inline(always)]
    pub fn flash0(&self) -> FLASH0_R {
        FLASH0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Power up Cache Bank 2. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 2, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb2(&mut self) -> CACHEB2_W {
        CACHEB2_W { w: self }
    }
    #[doc = "Bit 30 - Power up Cache Bank 0. This works in conjunction with Cache enable from flash_cache module. To power up cache bank 0, cache has to be enabled and this bit has to be set."]
    #[inline(always)]
    pub fn cacheb0(&mut self) -> CACHEB0_W {
        CACHEB0_W { w: self }
    }
    #[doc = "Bit 14 - Power up FLASH group 1 (1MB-2MB)"]
    #[inline(always)]
    pub fn flash1(&mut self) -> FLASH1_W {
        FLASH1_W { w: self }
    }
    #[doc = "Bit 13 - Power up FLASH group 0 (0MB-1MB)"]
    #[inline(always)]
    pub fn flash0(&mut self) -> FLASH0_W {
        FLASH0_W { w: self }
    }
    #[doc = "Bits 3:12 - Power up SRAM groups"]
    #[inline(always)]
    pub fn sram(&mut self) -> SRAM_W {
        SRAM_W { w: self }
    }
    #[doc = "Bits 0:2 - Power up DTCM"]
    #[inline(always)]
    pub fn dtcm(&mut self) -> DTCM_W {
        DTCM_W { w: self }
    }
}
