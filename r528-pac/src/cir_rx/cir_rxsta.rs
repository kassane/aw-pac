#[doc = "Register `CIR_RXSTA` reader"]
pub struct R(crate::R<CIR_RXSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_RXSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_RXSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_RXSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_RXSTA` writer"]
pub struct W(crate::W<CIR_RXSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_RXSTA_SPEC>;
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
impl From<crate::W<CIR_RXSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_RXSTA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Receiver Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_rxsta](index.html) module"]
pub struct CIR_RXSTA_SPEC;
impl crate::RegisterSpec for CIR_RXSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_rxsta::R](R) reader structure"]
impl crate::Readable for CIR_RXSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_rxsta::W](W) writer structure"]
impl crate::Writable for CIR_RXSTA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_RXSTA to value 0"]
impl crate::Resettable for CIR_RXSTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
