#[doc = "Reader of register DMASRAMREADPROTECT0"]
pub type R = crate::R<u32, super::DMASRAMREADPROTECT0>;
#[doc = "Writer for register DMASRAMREADPROTECT0"]
pub type W = crate::W<u32, super::DMASRAMREADPROTECT0>;
#[doc = "Register DMASRAMREADPROTECT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMASRAMREADPROTECT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_RPROT0`"]
pub type DMA_RPROT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_RPROT0`"]
pub struct DMA_RPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RPROT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot0(&self) -> DMA_RPROT0_R {
        DMA_RPROT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read protect SRAM from DMA. Each bit provides write protection for an 8KB region of memory. When set to 1, the region will be protected from DMA reads, when set to 0, DMA may read the region."]
    #[inline(always)]
    pub fn dma_rprot0(&mut self) -> DMA_RPROT0_W {
        DMA_RPROT0_W { w: self }
    }
}
