#[doc = "Register `lcd_ceu_coef_add%s` reader"]
pub struct R(crate::R<LCD_CEU_COEF_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CEU_COEF_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CEU_COEF_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CEU_COEF_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_ceu_coef_add%s` writer"]
pub struct W(crate::W<LCD_CEU_COEF_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CEU_COEF_ADD_SPEC>;
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
impl From<crate::W<LCD_CEU_COEF_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CEU_COEF_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ceu_coef_add_value` reader - Signed 19-bit value, range of (-16384, 16384)."]
pub type CEU_COEF_ADD_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ceu_coef_add_value` writer - Signed 19-bit value, range of (-16384, 16384)."]
pub type CEU_COEF_ADD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CEU_COEF_ADD_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:18 - Signed 19-bit value, range of (-16384, 16384)."]
    #[inline(always)]
    pub fn ceu_coef_add_value(&self) -> CEU_COEF_ADD_VALUE_R {
        CEU_COEF_ADD_VALUE_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - Signed 19-bit value, range of (-16384, 16384)."]
    #[inline(always)]
    #[must_use]
    pub fn ceu_coef_add_value(&mut self) -> CEU_COEF_ADD_VALUE_W<0> {
        CEU_COEF_ADD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD CEU Coefficient Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_ceu_coef_add](index.html) module"]
pub struct LCD_CEU_COEF_ADD_SPEC;
impl crate::RegisterSpec for LCD_CEU_COEF_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_ceu_coef_add::R](R) reader structure"]
impl crate::Readable for LCD_CEU_COEF_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_ceu_coef_add::W](W) writer structure"]
impl crate::Writable for LCD_CEU_COEF_ADD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_ceu_coef_add%s to value 0"]
impl crate::Resettable for LCD_CEU_COEF_ADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
