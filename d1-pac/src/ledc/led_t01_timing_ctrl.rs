#[doc = "Register `LED_T01_TIMING_CTRL` reader"]
pub struct R(crate::R<LED_T01_TIMING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LED_T01_TIMING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LED_T01_TIMING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LED_T01_TIMING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LED_T01_TIMING_CTRL` writer"]
pub struct W(crate::W<LED_T01_TIMING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LED_T01_TIMING_CTRL_SPEC>;
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
impl From<crate::W<LED_T01_TIMING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LED_T01_TIMING_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1H_TIME` reader - "]
pub type T1H_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T1H_TIME` writer - "]
pub type T1H_TIME_W<'a> = crate::FieldWriterSafe<'a, u32, LED_T01_TIMING_CTRL_SPEC, u8, u8, 6, 21>;
#[doc = "Field `T1L_TIME` reader - "]
pub type T1L_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T1L_TIME` writer - "]
pub type T1L_TIME_W<'a> = crate::FieldWriterSafe<'a, u32, LED_T01_TIMING_CTRL_SPEC, u8, u8, 5, 16>;
#[doc = "Field `T0H_TIME` reader - "]
pub type T0H_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T0H_TIME` writer - "]
pub type T0H_TIME_W<'a> = crate::FieldWriterSafe<'a, u32, LED_T01_TIMING_CTRL_SPEC, u8, u8, 5, 6>;
#[doc = "Field `T0L_TIME` reader - "]
pub type T0L_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T0L_TIME` writer - "]
pub type T0L_TIME_W<'a> = crate::FieldWriter<'a, u32, LED_T01_TIMING_CTRL_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 21:26"]
    #[inline(always)]
    pub fn t1h_time(&self) -> T1H_TIME_R {
        T1H_TIME_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn t1l_time(&self) -> T1L_TIME_R {
        T1L_TIME_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn t0h_time(&self) -> T0H_TIME_R {
        T0H_TIME_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn t0l_time(&self) -> T0L_TIME_R {
        T0L_TIME_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 21:26"]
    #[inline(always)]
    pub fn t1h_time(&mut self) -> T1H_TIME_W {
        T1H_TIME_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn t1l_time(&mut self) -> T1L_TIME_W {
        T1L_TIME_W::new(self)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn t0h_time(&mut self) -> T0H_TIME_W {
        T0H_TIME_W::new(self)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn t0l_time(&mut self) -> T0L_TIME_W {
        T0L_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC T0 T1 Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_t01_timing_ctrl](index.html) module"]
pub struct LED_T01_TIMING_CTRL_SPEC;
impl crate::RegisterSpec for LED_T01_TIMING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [led_t01_timing_ctrl::R](R) reader structure"]
impl crate::Readable for LED_T01_TIMING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [led_t01_timing_ctrl::W](W) writer structure"]
impl crate::Writable for LED_T01_TIMING_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LED_T01_TIMING_CTRL to value 0"]
impl crate::Resettable for LED_T01_TIMING_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
