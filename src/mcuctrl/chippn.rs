#[doc = "Reader of register CHIPPN"]
pub type R = crate::R<u32, super::CHIPPN>;
#[doc = "Writer for register CHIPPN"]
pub type W = crate::W<u32, super::CHIPPN>;
#[doc = "Register CHIPPN `reset()`'s with value 0x0600_0000"]
impl crate::ResetValue for super::CHIPPN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600_0000
    }
}
#[doc = "BCD part number.\n\nValue on reset: 100663296"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PARTNUM_A {
    #[doc = "117440512: Apollo3 Blue Plus part number is 0x07xxxxxx."]
    APOLLO3P = 117440512,
    #[doc = "100663296: Apollo3 Blue part number is 0x06xxxxxx."]
    APOLLO3 = 100663296,
    #[doc = "50331648: Apollo2 part number is 0x03xxxxxx."]
    APOLLO2 = 50331648,
    #[doc = "16777216: Apollo part number is 0x01xxxxxx."]
    APOLLO = 16777216,
    #[doc = "4278190080: Mask for the part number field."]
    PN_M = 4278190080,
    #[doc = "24: Bit position for the part number field."]
    PN_S = 24,
    #[doc = "15728640: Mask for the FLASH_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 2MB\n8: 1.5MB"]
    FLASHSIZE_M = 15728640,
    #[doc = "20: Bit position for the FLASH_SIZE field."]
    FLASHSIZE_S = 20,
    #[doc = "983040: Mask for the SRAM_SIZE field.\nValues:\n0: 16KB\n1: 32KB\n2: 64KB\n3: 128KB\n4: 256KB\n5: 512KB\n6: 1MB\n7: 384KB\n8: 768KB"]
    SRAMSIZE_M = 983040,
    #[doc = "16: Bit position for the SRAM_SIZE field."]
    SRAMSIZE_S = 16,
    #[doc = "65280: Mask for the revision field. Bits \\[15:12\\]
are major rev, \\[11:8\\]
are minor rev.\nValues:\n0: Major Rev A, Minor Rev 0\n1: Major Rev B, Minor Rev 1"]
    REV_M = 65280,
    #[doc = "8: Bit position for the revision field."]
    REV_S = 8,
    #[doc = "192: Mask for the package field.\nValues:\n0: SIP\n1: QFN\n2: BGA\n3: CSP"]
    PKG_M = 192,
    #[doc = "6: Bit position for the package field."]
    PKG_S = 6,
    #[doc = "56: Mask for the pins field.\nValues:\n0: 25 pins\n1: 49 pins\n2: 64 pins\n3: 81 pins"]
    PINS_M = 56,
    #[doc = "3: Bit position for the pins field."]
    PINS_S = 3,
    #[doc = "1: Bit position for the temperature field."]
    TEMP_S = 1,
    #[doc = "0: Bit position for the qualified field."]
    QUAL_S = 0,
}
impl From<PARTNUM_A> for u32 {
    #[inline(always)]
    fn from(variant: PARTNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PARTNUM`"]
pub type PARTNUM_R = crate::R<u32, PARTNUM_A>;
impl PARTNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PARTNUM_A> {
        use crate::Variant::*;
        match self.bits {
            117440512 => Val(PARTNUM_A::APOLLO3P),
            100663296 => Val(PARTNUM_A::APOLLO3),
            50331648 => Val(PARTNUM_A::APOLLO2),
            16777216 => Val(PARTNUM_A::APOLLO),
            4278190080 => Val(PARTNUM_A::PN_M),
            24 => Val(PARTNUM_A::PN_S),
            15728640 => Val(PARTNUM_A::FLASHSIZE_M),
            20 => Val(PARTNUM_A::FLASHSIZE_S),
            983040 => Val(PARTNUM_A::SRAMSIZE_M),
            16 => Val(PARTNUM_A::SRAMSIZE_S),
            65280 => Val(PARTNUM_A::REV_M),
            8 => Val(PARTNUM_A::REV_S),
            192 => Val(PARTNUM_A::PKG_M),
            6 => Val(PARTNUM_A::PKG_S),
            56 => Val(PARTNUM_A::PINS_M),
            3 => Val(PARTNUM_A::PINS_S),
            1 => Val(PARTNUM_A::TEMP_S),
            0 => Val(PARTNUM_A::QUAL_S),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3P`"]
    #[inline(always)]
    pub fn is_apollo3p(&self) -> bool {
        *self == PARTNUM_A::APOLLO3P
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        *self == PARTNUM_A::APOLLO3
    }
    #[doc = "Checks if the value of the field is `APOLLO2`"]
    #[inline(always)]
    pub fn is_apollo2(&self) -> bool {
        *self == PARTNUM_A::APOLLO2
    }
    #[doc = "Checks if the value of the field is `APOLLO`"]
    #[inline(always)]
    pub fn is_apollo(&self) -> bool {
        *self == PARTNUM_A::APOLLO
    }
    #[doc = "Checks if the value of the field is `PN_M`"]
    #[inline(always)]
    pub fn is_pn_m(&self) -> bool {
        *self == PARTNUM_A::PN_M
    }
    #[doc = "Checks if the value of the field is `PN_S`"]
    #[inline(always)]
    pub fn is_pn_s(&self) -> bool {
        *self == PARTNUM_A::PN_S
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_M`"]
    #[inline(always)]
    pub fn is_flashsize_m(&self) -> bool {
        *self == PARTNUM_A::FLASHSIZE_M
    }
    #[doc = "Checks if the value of the field is `FLASHSIZE_S`"]
    #[inline(always)]
    pub fn is_flashsize_s(&self) -> bool {
        *self == PARTNUM_A::FLASHSIZE_S
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_M`"]
    #[inline(always)]
    pub fn is_sramsize_m(&self) -> bool {
        *self == PARTNUM_A::SRAMSIZE_M
    }
    #[doc = "Checks if the value of the field is `SRAMSIZE_S`"]
    #[inline(always)]
    pub fn is_sramsize_s(&self) -> bool {
        *self == PARTNUM_A::SRAMSIZE_S
    }
    #[doc = "Checks if the value of the field is `REV_M`"]
    #[inline(always)]
    pub fn is_rev_m(&self) -> bool {
        *self == PARTNUM_A::REV_M
    }
    #[doc = "Checks if the value of the field is `REV_S`"]
    #[inline(always)]
    pub fn is_rev_s(&self) -> bool {
        *self == PARTNUM_A::REV_S
    }
    #[doc = "Checks if the value of the field is `PKG_M`"]
    #[inline(always)]
    pub fn is_pkg_m(&self) -> bool {
        *self == PARTNUM_A::PKG_M
    }
    #[doc = "Checks if the value of the field is `PKG_S`"]
    #[inline(always)]
    pub fn is_pkg_s(&self) -> bool {
        *self == PARTNUM_A::PKG_S
    }
    #[doc = "Checks if the value of the field is `PINS_M`"]
    #[inline(always)]
    pub fn is_pins_m(&self) -> bool {
        *self == PARTNUM_A::PINS_M
    }
    #[doc = "Checks if the value of the field is `PINS_S`"]
    #[inline(always)]
    pub fn is_pins_s(&self) -> bool {
        *self == PARTNUM_A::PINS_S
    }
    #[doc = "Checks if the value of the field is `TEMP_S`"]
    #[inline(always)]
    pub fn is_temp_s(&self) -> bool {
        *self == PARTNUM_A::TEMP_S
    }
    #[doc = "Checks if the value of the field is `QUAL_S`"]
    #[inline(always)]
    pub fn is_qual_s(&self) -> bool {
        *self == PARTNUM_A::QUAL_S
    }
}
#[doc = "Write proxy for field `PARTNUM`"]
pub struct PARTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARTNUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 Blue Plus part number is 0x07xxxxxx."]
    #[inline(always)]
    pub fn apollo3p(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO3P)
    }
    #[doc = "Apollo3 Blue part number is 0x06xxxxxx."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO3)
    }
    #[doc = "Apollo2 part number is 0x03xxxxxx."]
    #[inline(always)]
    pub fn apollo2(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO2)
    }
    #[doc = "Apollo part number is 0x01xxxxxx."]
    #[inline(always)]
    pub fn apollo(self) -> &'a mut W {
        self.variant(PARTNUM_A::APOLLO)
    }
    #[doc = "Mask for the part number field."]
    #[inline(always)]
    pub fn pn_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PN_M)
    }
    #[doc = "Bit position for the part number field."]
    #[inline(always)]
    pub fn pn_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PN_S)
    }
    #[doc = "Mask for the FLASH_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 2MB 8: 1.5MB"]
    #[inline(always)]
    pub fn flashsize_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::FLASHSIZE_M)
    }
    #[doc = "Bit position for the FLASH_SIZE field."]
    #[inline(always)]
    pub fn flashsize_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::FLASHSIZE_S)
    }
    #[doc = "Mask for the SRAM_SIZE field. Values: 0: 16KB 1: 32KB 2: 64KB 3: 128KB 4: 256KB 5: 512KB 6: 1MB 7: 384KB 8: 768KB"]
    #[inline(always)]
    pub fn sramsize_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::SRAMSIZE_M)
    }
    #[doc = "Bit position for the SRAM_SIZE field."]
    #[inline(always)]
    pub fn sramsize_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::SRAMSIZE_S)
    }
    #[doc = "Mask for the revision field. Bits \\[15:12\\]
are major rev, \\[11:8\\]
are minor rev. Values: 0: Major Rev A, Minor Rev 0 1: Major Rev B, Minor Rev 1"]
    #[inline(always)]
    pub fn rev_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::REV_M)
    }
    #[doc = "Bit position for the revision field."]
    #[inline(always)]
    pub fn rev_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::REV_S)
    }
    #[doc = "Mask for the package field. Values: 0: SIP 1: QFN 2: BGA 3: CSP"]
    #[inline(always)]
    pub fn pkg_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PKG_M)
    }
    #[doc = "Bit position for the package field."]
    #[inline(always)]
    pub fn pkg_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PKG_S)
    }
    #[doc = "Mask for the pins field. Values: 0: 25 pins 1: 49 pins 2: 64 pins 3: 81 pins"]
    #[inline(always)]
    pub fn pins_m(self) -> &'a mut W {
        self.variant(PARTNUM_A::PINS_M)
    }
    #[doc = "Bit position for the pins field."]
    #[inline(always)]
    pub fn pins_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::PINS_S)
    }
    #[doc = "Bit position for the temperature field."]
    #[inline(always)]
    pub fn temp_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::TEMP_S)
    }
    #[doc = "Bit position for the qualified field."]
    #[inline(always)]
    pub fn qual_s(self) -> &'a mut W {
        self.variant(PARTNUM_A::QUAL_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BCD part number."]
    #[inline(always)]
    pub fn partnum(&mut self) -> PARTNUM_W {
        PARTNUM_W { w: self }
    }
}
