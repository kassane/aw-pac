#[doc = "Register `TP_CALI_DATA` reader"]
pub struct R(crate::R<TP_CALI_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CALI_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CALI_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CALI_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_CALI_DATA` writer"]
pub struct W(crate::W<TP_CALI_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CALI_DATA_SPEC>;
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
impl From<crate::W<TP_CALI_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CALI_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TP_CDAT` reader - TP Common Data"]
pub struct TP_CDAT_R(crate::FieldReader<u16>);
impl TP_CDAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TP_CDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TP_CDAT_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_CDAT` writer - TP Common Data"]
pub struct TP_CDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_CDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    pub fn tp_cdat(&self) -> TP_CDAT_R {
        TP_CDAT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TP Common Data"]
    #[inline(always)]
    pub fn tp_cdat(&mut self) -> TP_CDAT_W {
        TP_CDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Calibration Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_cali_data](index.html) module"]
pub struct TP_CALI_DATA_SPEC;
impl crate::RegisterSpec for TP_CALI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_cali_data::R](R) reader structure"]
impl crate::Readable for TP_CALI_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_cali_data::W](W) writer structure"]
impl crate::Writable for TP_CALI_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_CALI_DATA to value 0"]
impl crate::Resettable for TP_CALI_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
