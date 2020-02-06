#[doc = "Reader of register DMATHRESH"]
pub type R = crate::R<u32, super::DMATHRESH>;
#[doc = "Writer for register DMATHRESH"]
pub type W = crate::W<u32, super::DMATHRESH>;
#[doc = "Register DMATHRESH `reset()`'s with value 0x0808"]
impl crate::ResetValue for super::DMATHRESH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0808
    }
}
#[doc = "Reader of field `DMARXTHRESH`"]
pub type DMARXTHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMARXTHRESH`"]
pub struct DMARXTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DMATXTHRESH`"]
pub type DMATXTHRESH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATXTHRESH`"]
pub struct DMATXTHRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXTHRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmarxthresh(&self) -> DMARXTHRESH_R {
        DMARXTHRESH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmatxthresh(&self) -> DMATXTHRESH_R {
        DMATXTHRESH_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmarxthresh(&mut self) -> DMARXTHRESH_W {
        DMARXTHRESH_W { w: self }
    }
    #[doc = "Bits 0:4 - DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub fn dmatxthresh(&mut self) -> DMATXTHRESH_W {
        DMATXTHRESH_W { w: self }
    }
}
