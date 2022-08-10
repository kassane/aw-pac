#[doc = "Register `csic_dma_buf_addr_fifo%s_entry` reader"]
pub struct R(crate::R<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_buf_addr_fifo%s_entry` writer"]
pub struct W(crate::W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csic_dma_buf_addr_fifo_entry` reader - FIFO Entry of Buffer Address FIFO\\[i\\]
for input frames to be stored, only used in Buffer Addr FIFO Mode"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `csic_dma_buf_addr_fifo_entry` writer - FIFO Entry of Buffer Address FIFO\\[i\\]
for input frames to be stored, only used in Buffer Addr FIFO Mode"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FIFO Entry of Buffer Address FIFO\\[i\\]
for input frames to be stored, only used in Buffer Addr FIFO Mode"]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_entry(&self) -> CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R {
        CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO Entry of Buffer Address FIFO\\[i\\]
for input frames to be stored, only used in Buffer Addr FIFO Mode"]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_entry(&mut self) -> CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W<0> {
        CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA BUF Address FIFO\\[i\\]
Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_buf_addr_fifo_entry](index.html) module"]
pub struct CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_buf_addr_fifo_entry::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_buf_addr_fifo_entry::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_buf_addr_fifo%s_entry to value 0"]
impl crate::Resettable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
