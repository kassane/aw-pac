#[doc = "Register `tvd_wb3` reader"]
pub struct R(crate::R<TVD_WB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVD_WB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVD_WB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVD_WB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tvd_wb3` writer"]
pub struct W(crate::W<TVD_WB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVD_WB3_SPEC>;
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
impl From<crate::W<TVD_WB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVD_WB3_SPEC>) -> Self {
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
#[doc = "TVD WB DMA CONTROL Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tvd_wb3](index.html) module"]
pub struct TVD_WB3_SPEC;
impl crate::RegisterSpec for TVD_WB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tvd_wb3::R](R) reader structure"]
impl crate::Readable for TVD_WB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tvd_wb3::W](W) writer structure"]
impl crate::Writable for TVD_WB3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tvd_wb3 to value 0"]
impl crate::Resettable for TVD_WB3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
