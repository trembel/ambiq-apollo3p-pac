#[doc = "Reader of register FLASH"]
pub type R = crate::R<u32, super::FLASH>;
#[doc = "Writer for register FLASH"]
pub type W = crate::W<u32, super::FLASH>;
#[doc = "Register FLASH `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XIPENWLAT`"]
pub type XIPENWLAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPENWLAT`"]
pub struct XIPENWLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPENWLAT_W<'a> {
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
#[doc = "Reserved. Set to 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XIPMIXED_A {
    #[doc = "0: Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    NORMAL = 0,
    #[doc = "1: Data operations proceed in dual data rate"]
    D2 = 1,
    #[doc = "3: Address and Data operations proceed in dual data rate"]
    AD2 = 3,
    #[doc = "5: Data operations proceed in quad data rate"]
    D4 = 5,
    #[doc = "7: Address and Data operations proceed in quad data rate"]
    AD4 = 7,
}
impl From<XIPMIXED_A> for u8 {
    #[inline(always)]
    fn from(variant: XIPMIXED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XIPMIXED`"]
pub type XIPMIXED_R = crate::R<u8, XIPMIXED_A>;
impl XIPMIXED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XIPMIXED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XIPMIXED_A::NORMAL),
            1 => Val(XIPMIXED_A::D2),
            3 => Val(XIPMIXED_A::AD2),
            5 => Val(XIPMIXED_A::D4),
            7 => Val(XIPMIXED_A::AD4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == XIPMIXED_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `D2`"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == XIPMIXED_A::D2
    }
    #[doc = "Checks if the value of the field is `AD2`"]
    #[inline(always)]
    pub fn is_ad2(&self) -> bool {
        *self == XIPMIXED_A::AD2
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == XIPMIXED_A::D4
    }
    #[doc = "Checks if the value of the field is `AD4`"]
    #[inline(always)]
    pub fn is_ad4(&self) -> bool {
        *self == XIPMIXED_A::AD4
    }
}
#[doc = "Write proxy for field `XIPMIXED`"]
pub struct XIPMIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPMIXED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIPMIXED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transfers all proceed using the settings in DEVCFG register (everything in the same data rate)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(XIPMIXED_A::NORMAL)
    }
    #[doc = "Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut W {
        self.variant(XIPMIXED_A::D2)
    }
    #[doc = "Address and Data operations proceed in dual data rate"]
    #[inline(always)]
    pub fn ad2(self) -> &'a mut W {
        self.variant(XIPMIXED_A::AD2)
    }
    #[doc = "Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(XIPMIXED_A::D4)
    }
    #[doc = "Address and Data operations proceed in quad data rate"]
    #[inline(always)]
    pub fn ad4(self) -> &'a mut W {
        self.variant(XIPMIXED_A::AD4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `XIPSENDI`"]
pub type XIPSENDI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPSENDI`"]
pub struct XIPSENDI_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPSENDI_W<'a> {
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
#[doc = "Reader of field `XIPSENDA`"]
pub type XIPSENDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPSENDA`"]
pub struct XIPSENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPSENDA_W<'a> {
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
#[doc = "Reader of field `XIPENTURN`"]
pub type XIPENTURN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPENTURN`"]
pub struct XIPENTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPENTURN_W<'a> {
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
#[doc = "Reader of field `XIPBIGENDIAN`"]
pub type XIPBIGENDIAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPBIGENDIAN`"]
pub struct XIPBIGENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPBIGENDIAN_W<'a> {
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
#[doc = "Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XIPACK_A {
    #[doc = "0: No acknowledgment sent. Data IOs are tri-stated the first turnaround cycle"]
    NOACK = 0,
    #[doc = "2: Positive acknowledgment sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode"]
    ACK = 2,
    #[doc = "3: Negative acknowledgment sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be re-enabled for the next transfer"]
    TERMINATE = 3,
}
impl From<XIPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: XIPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XIPACK`"]
pub type XIPACK_R = crate::R<u8, XIPACK_A>;
impl XIPACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XIPACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XIPACK_A::NOACK),
            2 => Val(XIPACK_A::ACK),
            3 => Val(XIPACK_A::TERMINATE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOACK`"]
    #[inline(always)]
    pub fn is_noack(&self) -> bool {
        *self == XIPACK_A::NOACK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == XIPACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `TERMINATE`"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == XIPACK_A::TERMINATE
    }
}
#[doc = "Write proxy for field `XIPACK`"]
pub struct XIPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No acknowledgment sent. Data IOs are tri-stated the first turnaround cycle"]
    #[inline(always)]
    pub fn noack(self) -> &'a mut W {
        self.variant(XIPACK_A::NOACK)
    }
    #[doc = "Positive acknowledgment sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(XIPACK_A::ACK)
    }
    #[doc = "Negative acknowledgment sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be re-enabled for the next transfer"]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut W {
        self.variant(XIPACK_A::TERMINATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `XIPENDCX`"]
pub type XIPENDCX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPENDCX`"]
pub struct XIPENDCX_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPENDCX_W<'a> {
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
#[doc = "Reader of field `XIPEN`"]
pub type XIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XIPEN`"]
pub struct XIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPEN_W<'a> {
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
    #[doc = "Bit 11 - Enable Write Latency counter for XIP write transactions"]
    #[inline(always)]
    pub fn xipenwlat(&self) -> XIPENWLAT_R {
        XIPENWLAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline(always)]
    pub fn xipmixed(&self) -> XIPMIXED_R {
        XIPMIXED_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsendi(&self) -> XIPSENDI_R {
        XIPSENDI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsenda(&self) -> XIPSENDA_R {
        XIPSENDA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    pub fn xipenturn(&self) -> XIPENTURN_R {
        XIPENTURN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    pub fn xipbigendian(&self) -> XIPBIGENDIAN_R {
        XIPBIGENDIAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    pub fn xipack(&self) -> XIPACK_R {
        XIPACK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
    #[inline(always)]
    pub fn xipendcx(&self) -> XIPENDCX_R {
        XIPENDCX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub fn xipen(&self) -> XIPEN_R {
        XIPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Enable Write Latency counter for XIP write transactions"]
    #[inline(always)]
    pub fn xipenwlat(&mut self) -> XIPENWLAT_W {
        XIPENWLAT_W { w: self }
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline(always)]
    pub fn xipmixed(&mut self) -> XIPMIXED_W {
        XIPMIXED_W { w: self }
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsendi(&mut self) -> XIPSENDI_W {
        XIPSENDI_W { w: self }
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsenda(&mut self) -> XIPSENDA_W {
        XIPSENDA_W { w: self }
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    pub fn xipenturn(&mut self) -> XIPENTURN_W {
        XIPENTURN_W { w: self }
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    pub fn xipbigendian(&mut self) -> XIPBIGENDIAN_W {
        XIPBIGENDIAN_W { w: self }
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    pub fn xipack(&mut self) -> XIPACK_W {
        XIPACK_W { w: self }
    }
    #[doc = "Bit 1 - Enable DCX signal on data \\[1\\]
for XIP/DMA operations"]
    #[inline(always)]
    pub fn xipendcx(&mut self) -> XIPENDCX_W {
        XIPENDCX_W { w: self }
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub fn xipen(&mut self) -> XIPEN_W {
        XIPEN_W { w: self }
    }
}
