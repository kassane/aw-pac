#[doc = "Register `PDZCR45` reader"]
pub struct R(crate::R<PDZCR45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDZCR45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDZCR45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDZCR45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDZCR45` writer"]
pub struct W(crate::W<PDZCR45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDZCR45_SPEC>;
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
impl From<crate::W<PDZCR45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDZCR45_SPEC>) -> Self {
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
#[doc = "PWM45 Dead Zone Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdzcr45](index.html) module"]
pub struct PDZCR45_SPEC;
impl crate::RegisterSpec for PDZCR45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdzcr45::R](R) reader structure"]
impl crate::Readable for PDZCR45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdzcr45::W](W) writer structure"]
impl crate::Writable for PDZCR45_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDZCR45 to value 0"]
impl crate::Resettable for PDZCR45_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}