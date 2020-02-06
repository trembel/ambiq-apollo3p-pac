#[doc = "Reader of register INT1CLR"]
pub type R = crate::R<u32, super::INT1CLR>;
#[doc = "Writer for register INT1CLR"]
pub type W = crate::W<u32, super::INT1CLR>;
#[doc = "Register INT1CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT1CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO63`"]
pub type GPIO63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO63`"]
pub struct GPIO63_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO63_W<'a> {
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
#[doc = "Reader of field `GPIO62`"]
pub type GPIO62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO62`"]
pub struct GPIO62_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO62_W<'a> {
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
#[doc = "Reader of field `GPIO61`"]
pub type GPIO61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO61`"]
pub struct GPIO61_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO61_W<'a> {
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
#[doc = "Reader of field `GPIO60`"]
pub type GPIO60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO60`"]
pub struct GPIO60_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO60_W<'a> {
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
#[doc = "Reader of field `GPIO59`"]
pub type GPIO59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO59`"]
pub struct GPIO59_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO59_W<'a> {
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
#[doc = "Reader of field `GPIO58`"]
pub type GPIO58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO58`"]
pub struct GPIO58_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO58_W<'a> {
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
#[doc = "Reader of field `GPIO57`"]
pub type GPIO57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO57`"]
pub struct GPIO57_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO57_W<'a> {
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
#[doc = "Reader of field `GPIO56`"]
pub type GPIO56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO56`"]
pub struct GPIO56_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO56_W<'a> {
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
#[doc = "Reader of field `GPIO55`"]
pub type GPIO55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO55`"]
pub struct GPIO55_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO55_W<'a> {
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
#[doc = "Reader of field `GPIO54`"]
pub type GPIO54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO54`"]
pub struct GPIO54_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO54_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `GPIO53`"]
pub type GPIO53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO53`"]
pub struct GPIO53_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO53_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `GPIO52`"]
pub type GPIO52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO52`"]
pub struct GPIO52_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO52_W<'a> {
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
#[doc = "Reader of field `GPIO51`"]
pub type GPIO51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO51`"]
pub struct GPIO51_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO51_W<'a> {
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
#[doc = "Reader of field `GPIO50`"]
pub type GPIO50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO50`"]
pub struct GPIO50_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO50_W<'a> {
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
#[doc = "Reader of field `GPIO49`"]
pub type GPIO49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO49`"]
pub struct GPIO49_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49_W<'a> {
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
#[doc = "Reader of field `GPIO48`"]
pub type GPIO48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO48`"]
pub struct GPIO48_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48_W<'a> {
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
#[doc = "Reader of field `GPIO47`"]
pub type GPIO47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO47`"]
pub struct GPIO47_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47_W<'a> {
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
#[doc = "Reader of field `GPIO46`"]
pub type GPIO46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO46`"]
pub struct GPIO46_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46_W<'a> {
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
#[doc = "Reader of field `GPIO45`"]
pub type GPIO45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO45`"]
pub struct GPIO45_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45_W<'a> {
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
#[doc = "Reader of field `GPIO44`"]
pub type GPIO44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO44`"]
pub struct GPIO44_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44_W<'a> {
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
#[doc = "Reader of field `GPIO43`"]
pub type GPIO43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO43`"]
pub struct GPIO43_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43_W<'a> {
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
#[doc = "Reader of field `GPIO42`"]
pub type GPIO42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO42`"]
pub struct GPIO42_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42_W<'a> {
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
#[doc = "Reader of field `GPIO41`"]
pub type GPIO41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO41`"]
pub struct GPIO41_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41_W<'a> {
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
#[doc = "Reader of field `GPIO40`"]
pub type GPIO40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO40`"]
pub struct GPIO40_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40_W<'a> {
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
#[doc = "Reader of field `GPIO39`"]
pub type GPIO39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO39`"]
pub struct GPIO39_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO39_W<'a> {
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
#[doc = "Reader of field `GPIO38`"]
pub type GPIO38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO38`"]
pub struct GPIO38_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO38_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GPIO37`"]
pub type GPIO37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO37`"]
pub struct GPIO37_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO37_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GPIO36`"]
pub type GPIO36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO36`"]
pub struct GPIO36_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO36_W<'a> {
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
#[doc = "Reader of field `GPIO35`"]
pub type GPIO35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO35`"]
pub struct GPIO35_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO35_W<'a> {
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
#[doc = "Reader of field `GPIO34`"]
pub type GPIO34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO34`"]
pub struct GPIO34_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO34_W<'a> {
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
#[doc = "Reader of field `GPIO33`"]
pub type GPIO33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO33`"]
pub struct GPIO33_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO33_W<'a> {
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
#[doc = "Reader of field `GPIO32`"]
pub type GPIO32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO32`"]
pub struct GPIO32_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO32_W<'a> {
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
    #[doc = "Bit 31 - GPIO63 interrupt."]
    #[inline(always)]
    pub fn gpio63(&self) -> GPIO63_R {
        GPIO63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIO62 interrupt."]
    #[inline(always)]
    pub fn gpio62(&self) -> GPIO62_R {
        GPIO62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIO61 interrupt."]
    #[inline(always)]
    pub fn gpio61(&self) -> GPIO61_R {
        GPIO61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPIO60 interrupt."]
    #[inline(always)]
    pub fn gpio60(&self) -> GPIO60_R {
        GPIO60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO59 interrupt."]
    #[inline(always)]
    pub fn gpio59(&self) -> GPIO59_R {
        GPIO59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GPIO58 interrupt."]
    #[inline(always)]
    pub fn gpio58(&self) -> GPIO58_R {
        GPIO58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GPIO57 interrupt."]
    #[inline(always)]
    pub fn gpio57(&self) -> GPIO57_R {
        GPIO57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GPIO56 interrupt."]
    #[inline(always)]
    pub fn gpio56(&self) -> GPIO56_R {
        GPIO56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO55 interrupt."]
    #[inline(always)]
    pub fn gpio55(&self) -> GPIO55_R {
        GPIO55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GPIO54 interrupt."]
    #[inline(always)]
    pub fn gpio54(&self) -> GPIO54_R {
        GPIO54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIO53 interrupt."]
    #[inline(always)]
    pub fn gpio53(&self) -> GPIO53_R {
        GPIO53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO52 interrupt."]
    #[inline(always)]
    pub fn gpio52(&self) -> GPIO52_R {
        GPIO52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO51 interrupt."]
    #[inline(always)]
    pub fn gpio51(&self) -> GPIO51_R {
        GPIO51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GPIO50 interrupt."]
    #[inline(always)]
    pub fn gpio50(&self) -> GPIO50_R {
        GPIO50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO49 interrupt."]
    #[inline(always)]
    pub fn gpio49(&self) -> GPIO49_R {
        GPIO49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO48 interrupt."]
    #[inline(always)]
    pub fn gpio48(&self) -> GPIO48_R {
        GPIO48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO47 interrupt."]
    #[inline(always)]
    pub fn gpio47(&self) -> GPIO47_R {
        GPIO47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO46 interrupt."]
    #[inline(always)]
    pub fn gpio46(&self) -> GPIO46_R {
        GPIO46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO45 interrupt."]
    #[inline(always)]
    pub fn gpio45(&self) -> GPIO45_R {
        GPIO45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO44 interrupt."]
    #[inline(always)]
    pub fn gpio44(&self) -> GPIO44_R {
        GPIO44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO43 interrupt."]
    #[inline(always)]
    pub fn gpio43(&self) -> GPIO43_R {
        GPIO43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO42 interrupt."]
    #[inline(always)]
    pub fn gpio42(&self) -> GPIO42_R {
        GPIO42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO41 interrupt."]
    #[inline(always)]
    pub fn gpio41(&self) -> GPIO41_R {
        GPIO41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO40 interrupt."]
    #[inline(always)]
    pub fn gpio40(&self) -> GPIO40_R {
        GPIO40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO39 interrupt."]
    #[inline(always)]
    pub fn gpio39(&self) -> GPIO39_R {
        GPIO39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO38 interrupt."]
    #[inline(always)]
    pub fn gpio38(&self) -> GPIO38_R {
        GPIO38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO37 interrupt."]
    #[inline(always)]
    pub fn gpio37(&self) -> GPIO37_R {
        GPIO37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO36 interrupt."]
    #[inline(always)]
    pub fn gpio36(&self) -> GPIO36_R {
        GPIO36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO35 interrupt."]
    #[inline(always)]
    pub fn gpio35(&self) -> GPIO35_R {
        GPIO35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO34 interrupt."]
    #[inline(always)]
    pub fn gpio34(&self) -> GPIO34_R {
        GPIO34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO33 interrupt."]
    #[inline(always)]
    pub fn gpio33(&self) -> GPIO33_R {
        GPIO33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO32 interrupt."]
    #[inline(always)]
    pub fn gpio32(&self) -> GPIO32_R {
        GPIO32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO63 interrupt."]
    #[inline(always)]
    pub fn gpio63(&mut self) -> GPIO63_W {
        GPIO63_W { w: self }
    }
    #[doc = "Bit 30 - GPIO62 interrupt."]
    #[inline(always)]
    pub fn gpio62(&mut self) -> GPIO62_W {
        GPIO62_W { w: self }
    }
    #[doc = "Bit 29 - GPIO61 interrupt."]
    #[inline(always)]
    pub fn gpio61(&mut self) -> GPIO61_W {
        GPIO61_W { w: self }
    }
    #[doc = "Bit 28 - GPIO60 interrupt."]
    #[inline(always)]
    pub fn gpio60(&mut self) -> GPIO60_W {
        GPIO60_W { w: self }
    }
    #[doc = "Bit 27 - GPIO59 interrupt."]
    #[inline(always)]
    pub fn gpio59(&mut self) -> GPIO59_W {
        GPIO59_W { w: self }
    }
    #[doc = "Bit 26 - GPIO58 interrupt."]
    #[inline(always)]
    pub fn gpio58(&mut self) -> GPIO58_W {
        GPIO58_W { w: self }
    }
    #[doc = "Bit 25 - GPIO57 interrupt."]
    #[inline(always)]
    pub fn gpio57(&mut self) -> GPIO57_W {
        GPIO57_W { w: self }
    }
    #[doc = "Bit 24 - GPIO56 interrupt."]
    #[inline(always)]
    pub fn gpio56(&mut self) -> GPIO56_W {
        GPIO56_W { w: self }
    }
    #[doc = "Bit 23 - GPIO55 interrupt."]
    #[inline(always)]
    pub fn gpio55(&mut self) -> GPIO55_W {
        GPIO55_W { w: self }
    }
    #[doc = "Bit 22 - GPIO54 interrupt."]
    #[inline(always)]
    pub fn gpio54(&mut self) -> GPIO54_W {
        GPIO54_W { w: self }
    }
    #[doc = "Bit 21 - GPIO53 interrupt."]
    #[inline(always)]
    pub fn gpio53(&mut self) -> GPIO53_W {
        GPIO53_W { w: self }
    }
    #[doc = "Bit 20 - GPIO52 interrupt."]
    #[inline(always)]
    pub fn gpio52(&mut self) -> GPIO52_W {
        GPIO52_W { w: self }
    }
    #[doc = "Bit 19 - GPIO51 interrupt."]
    #[inline(always)]
    pub fn gpio51(&mut self) -> GPIO51_W {
        GPIO51_W { w: self }
    }
    #[doc = "Bit 18 - GPIO50 interrupt."]
    #[inline(always)]
    pub fn gpio50(&mut self) -> GPIO50_W {
        GPIO50_W { w: self }
    }
    #[doc = "Bit 17 - GPIO49 interrupt."]
    #[inline(always)]
    pub fn gpio49(&mut self) -> GPIO49_W {
        GPIO49_W { w: self }
    }
    #[doc = "Bit 16 - GPIO48 interrupt."]
    #[inline(always)]
    pub fn gpio48(&mut self) -> GPIO48_W {
        GPIO48_W { w: self }
    }
    #[doc = "Bit 15 - GPIO47 interrupt."]
    #[inline(always)]
    pub fn gpio47(&mut self) -> GPIO47_W {
        GPIO47_W { w: self }
    }
    #[doc = "Bit 14 - GPIO46 interrupt."]
    #[inline(always)]
    pub fn gpio46(&mut self) -> GPIO46_W {
        GPIO46_W { w: self }
    }
    #[doc = "Bit 13 - GPIO45 interrupt."]
    #[inline(always)]
    pub fn gpio45(&mut self) -> GPIO45_W {
        GPIO45_W { w: self }
    }
    #[doc = "Bit 12 - GPIO44 interrupt."]
    #[inline(always)]
    pub fn gpio44(&mut self) -> GPIO44_W {
        GPIO44_W { w: self }
    }
    #[doc = "Bit 11 - GPIO43 interrupt."]
    #[inline(always)]
    pub fn gpio43(&mut self) -> GPIO43_W {
        GPIO43_W { w: self }
    }
    #[doc = "Bit 10 - GPIO42 interrupt."]
    #[inline(always)]
    pub fn gpio42(&mut self) -> GPIO42_W {
        GPIO42_W { w: self }
    }
    #[doc = "Bit 9 - GPIO41 interrupt."]
    #[inline(always)]
    pub fn gpio41(&mut self) -> GPIO41_W {
        GPIO41_W { w: self }
    }
    #[doc = "Bit 8 - GPIO40 interrupt."]
    #[inline(always)]
    pub fn gpio40(&mut self) -> GPIO40_W {
        GPIO40_W { w: self }
    }
    #[doc = "Bit 7 - GPIO39 interrupt."]
    #[inline(always)]
    pub fn gpio39(&mut self) -> GPIO39_W {
        GPIO39_W { w: self }
    }
    #[doc = "Bit 6 - GPIO38 interrupt."]
    #[inline(always)]
    pub fn gpio38(&mut self) -> GPIO38_W {
        GPIO38_W { w: self }
    }
    #[doc = "Bit 5 - GPIO37 interrupt."]
    #[inline(always)]
    pub fn gpio37(&mut self) -> GPIO37_W {
        GPIO37_W { w: self }
    }
    #[doc = "Bit 4 - GPIO36 interrupt."]
    #[inline(always)]
    pub fn gpio36(&mut self) -> GPIO36_W {
        GPIO36_W { w: self }
    }
    #[doc = "Bit 3 - GPIO35 interrupt."]
    #[inline(always)]
    pub fn gpio35(&mut self) -> GPIO35_W {
        GPIO35_W { w: self }
    }
    #[doc = "Bit 2 - GPIO34 interrupt."]
    #[inline(always)]
    pub fn gpio34(&mut self) -> GPIO34_W {
        GPIO34_W { w: self }
    }
    #[doc = "Bit 1 - GPIO33 interrupt."]
    #[inline(always)]
    pub fn gpio33(&mut self) -> GPIO33_W {
        GPIO33_W { w: self }
    }
    #[doc = "Bit 0 - GPIO32 interrupt."]
    #[inline(always)]
    pub fn gpio32(&mut self) -> GPIO32_W {
        GPIO32_W { w: self }
    }
}
