#[doc = "Register `TP_CTRL1` reader"]
pub struct R(crate::R<TP_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_CTRL1` writer"]
pub struct W(crate::W<TP_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CTRL1_SPEC>;
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
impl From<crate::W<TP_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STYLUS_UP_DEBOUNCE` reader - Stylus Up Debounce Time Setting"]
pub struct STYLUS_UP_DEBOUNCE_R(crate::FieldReader<u8>);
impl STYLUS_UP_DEBOUNCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STYLUS_UP_DEBOUNCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STYLUS_UP_DEBOUNCE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STYLUS_UP_DEBOUNCE` writer - Stylus Up Debounce Time Setting"]
pub struct STYLUS_UP_DEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> STYLUS_UP_DEBOUNCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Stylus Up Debounce Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STYLUS_UP_DEBOUNCE_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STYLUS_UP_DEBOUNCE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: STYLUS_UP_DEBOUNCE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STYLUS_UP_DEBOUNCE_EN` reader - Stylus Up Debounce Function Select"]
pub struct STYLUS_UP_DEBOUNCE_EN_R(crate::FieldReader<bool>);
impl STYLUS_UP_DEBOUNCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STYLUS_UP_DEBOUNCE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STYLUS_UP_DEBOUNCE_EN_A {
        match self.bits {
            false => STYLUS_UP_DEBOUNCE_EN_A::DISABLE,
            true => STYLUS_UP_DEBOUNCE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == STYLUS_UP_DEBOUNCE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == STYLUS_UP_DEBOUNCE_EN_A::ENABLE
    }
}
impl core::ops::Deref for STYLUS_UP_DEBOUNCE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STYLUS_UP_DEBOUNCE_EN` writer - Stylus Up Debounce Function Select"]
pub struct STYLUS_UP_DEBOUNCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STYLUS_UP_DEBOUNCE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STYLUS_UP_DEBOUNCE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STYLUS_UP_DEBOUNCE_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STYLUS_UP_DEBOUNCE_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "T-sensor Chopping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOPPER_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CHOPPER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CHOPPER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHOPPER_EN` reader - T-sensor Chopping Enable"]
pub struct CHOPPER_EN_R(crate::FieldReader<bool>);
impl CHOPPER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHOPPER_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHOPPER_EN_A {
        match self.bits {
            false => CHOPPER_EN_A::DISABLE,
            true => CHOPPER_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHOPPER_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CHOPPER_EN_A::ENABLE
    }
}
impl core::ops::Deref for CHOPPER_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHOPPER_EN` writer - T-sensor Chopping Enable"]
pub struct CHOPPER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHOPPER_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHOPPER_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHOPPER_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHOPPER_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Touch Panel Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUCH_PAN_CALI_EN_A {
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TOUCH_PAN_CALI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TOUCH_PAN_CALI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOUCH_PAN_CALI_EN` reader - Touch Panel Calibration"]
pub struct TOUCH_PAN_CALI_EN_R(crate::FieldReader<bool>);
impl TOUCH_PAN_CALI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_PAN_CALI_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOUCH_PAN_CALI_EN_A> {
        match self.bits {
            true => Some(TOUCH_PAN_CALI_EN_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == TOUCH_PAN_CALI_EN_A::START
    }
}
impl core::ops::Deref for TOUCH_PAN_CALI_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_PAN_CALI_EN` writer - Touch Panel Calibration"]
pub struct TOUCH_PAN_CALI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAN_CALI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUCH_PAN_CALI_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TOUCH_PAN_CALI_EN_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Touch Panel Double Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_DUAL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DUAL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DUAL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_DUAL_EN` reader - Touch Panel Double Point Enable"]
pub struct TP_DUAL_EN_R(crate::FieldReader<bool>);
impl TP_DUAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_DUAL_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DUAL_EN_A {
        match self.bits {
            false => TP_DUAL_EN_A::DISABLE,
            true => TP_DUAL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_DUAL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_DUAL_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_DUAL_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_DUAL_EN` writer - Touch Panel Double Point Enable"]
pub struct TP_DUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_DUAL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_DUAL_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DUAL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DUAL_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "TP Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_EN` reader - TP Function Enable"]
pub struct TP_EN_R(crate::FieldReader<bool>);
impl TP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_EN_A {
        match self.bits {
            false => TP_EN_A::DISABLE,
            true => TP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TP_EN_A::ENABLE
    }
}
impl core::ops::Deref for TP_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_EN` writer - TP Function Enable"]
pub struct TP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Touch Panel Mode and Auxiliary ADC Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP_MODE_SELECT_A {
    #[doc = "0: `0`"]
    TP = 0,
    #[doc = "1: `1`"]
    AUXILIARYADC = 1,
}
impl From<TP_MODE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TP_MODE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TP_MODE_SELECT` reader - Touch Panel Mode and Auxiliary ADC Mode Select"]
pub struct TP_MODE_SELECT_R(crate::FieldReader<bool>);
impl TP_MODE_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_MODE_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_MODE_SELECT_A {
        match self.bits {
            false => TP_MODE_SELECT_A::TP,
            true => TP_MODE_SELECT_A::AUXILIARYADC,
        }
    }
    #[doc = "Checks if the value of the field is `TP`"]
    #[inline(always)]
    pub fn is_tp(&self) -> bool {
        **self == TP_MODE_SELECT_A::TP
    }
    #[doc = "Checks if the value of the field is `AUXILIARYADC`"]
    #[inline(always)]
    pub fn is_auxiliary_adc(&self) -> bool {
        **self == TP_MODE_SELECT_A::AUXILIARYADC
    }
}
impl core::ops::Deref for TP_MODE_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP_MODE_SELECT` writer - Touch Panel Mode and Auxiliary ADC Mode Select"]
pub struct TP_MODE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_MODE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TP_MODE_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tp(self) -> &'a mut W {
        self.variant(TP_MODE_SELECT_A::TP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn auxiliary_adc(self) -> &'a mut W {
        self.variant(TP_MODE_SELECT_A::AUXILIARYADC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Analog Input Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CHAN_SELECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ADC_CHAN_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_CHAN_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `ADC_CHAN(0-3)_SELECT` reader - Analog Input Channel Select"]
pub struct ADC_CHAN_SELECT_R(crate::FieldReader<bool>);
impl ADC_CHAN_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CHAN_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_CHAN_SELECT_A {
        match self.bits {
            false => ADC_CHAN_SELECT_A::DISABLE,
            true => ADC_CHAN_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADC_CHAN_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADC_CHAN_SELECT_A::ENABLE
    }
}
impl core::ops::Deref for ADC_CHAN_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `ADC_CHAN(0-3)_SELECT` const generic writer - Analog Input Channel Select"]
pub struct ADC_CHAN_SELECT_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> ADC_CHAN_SELECT_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_CHAN_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_CHAN_SELECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_CHAN_SELECT_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:19 - Stylus Up Debounce Time Setting"]
    #[inline(always)]
    pub fn stylus_up_debounce(&self) -> STYLUS_UP_DEBOUNCE_R {
        STYLUS_UP_DEBOUNCE_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 9 - Stylus Up Debounce Function Select"]
    #[inline(always)]
    pub fn stylus_up_debounce_en(&self) -> STYLUS_UP_DEBOUNCE_EN_R {
        STYLUS_UP_DEBOUNCE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - T-sensor Chopping Enable"]
    #[inline(always)]
    pub fn chopper_en(&self) -> CHOPPER_EN_R {
        CHOPPER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Touch Panel Calibration"]
    #[inline(always)]
    pub fn touch_pan_cali_en(&self) -> TOUCH_PAN_CALI_EN_R {
        TOUCH_PAN_CALI_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Touch Panel Double Point Enable"]
    #[inline(always)]
    pub fn tp_dual_en(&self) -> TP_DUAL_EN_R {
        TP_DUAL_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - TP Function Enable"]
    #[inline(always)]
    pub fn tp_en(&self) -> TP_EN_R {
        TP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Touch Panel Mode and Auxiliary ADC Mode Select"]
    #[inline(always)]
    pub fn tp_mode_select(&self) -> TP_MODE_SELECT_R {
        TP_MODE_SELECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    pub unsafe fn adc_chan_select(&self, n: usize) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan0_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan1_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan2_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan3_select(&self) -> ADC_CHAN_SELECT_R {
        ADC_CHAN_SELECT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:19 - Stylus Up Debounce Time Setting"]
    #[inline(always)]
    pub fn stylus_up_debounce(&mut self) -> STYLUS_UP_DEBOUNCE_W {
        STYLUS_UP_DEBOUNCE_W { w: self }
    }
    #[doc = "Bit 9 - Stylus Up Debounce Function Select"]
    #[inline(always)]
    pub fn stylus_up_debounce_en(&mut self) -> STYLUS_UP_DEBOUNCE_EN_W {
        STYLUS_UP_DEBOUNCE_EN_W { w: self }
    }
    #[doc = "Bit 8 - T-sensor Chopping Enable"]
    #[inline(always)]
    pub fn chopper_en(&mut self) -> CHOPPER_EN_W {
        CHOPPER_EN_W { w: self }
    }
    #[doc = "Bit 7 - Touch Panel Calibration"]
    #[inline(always)]
    pub fn touch_pan_cali_en(&mut self) -> TOUCH_PAN_CALI_EN_W {
        TOUCH_PAN_CALI_EN_W { w: self }
    }
    #[doc = "Bit 6 - Touch Panel Double Point Enable"]
    #[inline(always)]
    pub fn tp_dual_en(&mut self) -> TP_DUAL_EN_W {
        TP_DUAL_EN_W { w: self }
    }
    #[doc = "Bit 5 - TP Function Enable"]
    #[inline(always)]
    pub fn tp_en(&mut self) -> TP_EN_W {
        TP_EN_W { w: self }
    }
    #[doc = "Bit 4 - Touch Panel Mode and Auxiliary ADC Mode Select"]
    #[inline(always)]
    pub fn tp_mode_select(&mut self) -> TP_MODE_SELECT_W {
        TP_MODE_SELECT_W { w: self }
    }
    #[doc = "Analog Input Channel Select"]
    #[inline(always)]
    pub unsafe fn adc_chan_select<const O: usize>(&mut self) -> ADC_CHAN_SELECT_W<O> {
        ADC_CHAN_SELECT_W { w: self }
    }
    #[doc = "Bit 0 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan0_select(&mut self) -> ADC_CHAN_SELECT_W<0> {
        ADC_CHAN_SELECT_W { w: self }
    }
    #[doc = "Bit 1 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan1_select(&mut self) -> ADC_CHAN_SELECT_W<1> {
        ADC_CHAN_SELECT_W { w: self }
    }
    #[doc = "Bit 2 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan2_select(&mut self) -> ADC_CHAN_SELECT_W<2> {
        ADC_CHAN_SELECT_W { w: self }
    }
    #[doc = "Bit 3 - Analog Input Channel Select"]
    #[inline(always)]
    pub fn adc_chan3_select(&mut self) -> ADC_CHAN_SELECT_W<3> {
        ADC_CHAN_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_ctrl1](index.html) module"]
pub struct TP_CTRL1_SPEC;
impl crate::RegisterSpec for TP_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_ctrl1::R](R) reader structure"]
impl crate::Readable for TP_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_ctrl1::W](W) writer structure"]
impl crate::Writable for TP_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_CTRL1 to value 0"]
impl crate::Resettable for TP_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
