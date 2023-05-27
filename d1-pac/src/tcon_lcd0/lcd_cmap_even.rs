#[doc = "Register `lcd_cmap_even%s` reader"]
pub struct R(crate::R<LCD_CMAP_EVEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_CMAP_EVEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_CMAP_EVEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_CMAP_EVEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_cmap_even%s` writer"]
pub struct W(crate::W<LCD_CMAP_EVEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_CMAP_EVEN_SPEC>;
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
impl From<crate::W<LCD_CMAP_EVEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_CMAP_EVEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `out_even[0-1]` reader - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
pub type OUT_EVEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `out_even[0-1]` writer - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
pub type OUT_EVEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_CMAP_EVEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    pub unsafe fn out_even(&self, n: u8) -> OUT_EVEN_R {
        OUT_EVEN_R::new(((self.bits >> (n * 16)) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    pub fn out_even0(&self) -> OUT_EVEN_R {
        OUT_EVEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    pub fn out_even1(&self) -> OUT_EVEN_R {
        OUT_EVEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn out_even<const O: u8>(&mut self) -> OUT_EVEN_W<O> {
        OUT_EVEN_W::new(self)
    }
    #[doc = "Bits 0:15 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn out_even0(&mut self) -> OUT_EVEN_W<0> {
        OUT_EVEN_W::new(self)
    }
    #[doc = "Bits 16:31 - OUT_EVEN\\[2i + j\\]\n\nIndicates the output order of components.\n\nbit15-12: Reserved\n\nbit11-08: Out_Even\\[23:16\\]\n\nbit07-04: Out_Even0\\[15:8\\]\n\nbit03-00: Out_Even0\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn out_even1(&mut self) -> OUT_EVEN_W<16> {
        OUT_EVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Color Map Even Line Register\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_cmap_even](index.html) module"]
pub struct LCD_CMAP_EVEN_SPEC;
impl crate::RegisterSpec for LCD_CMAP_EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_cmap_even::R](R) reader structure"]
impl crate::Readable for LCD_CMAP_EVEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_cmap_even::W](W) writer structure"]
impl crate::Writable for LCD_CMAP_EVEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_cmap_even%s to value 0"]
impl crate::Resettable for LCD_CMAP_EVEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
