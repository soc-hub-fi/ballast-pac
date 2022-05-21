#[doc = "Register `D_PERF_ENABLE` reader"]
pub struct R(crate::R<D_PERF_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PERF_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PERF_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PERF_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERF_DMA_EN_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<PERF_DMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_DMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERF_DMA_EN` reader - "]
pub struct PERF_DMA_EN_R(crate::FieldReader<bool, PERF_DMA_EN_A>);
impl PERF_DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERF_DMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_DMA_EN_A {
        match self.bits {
            true => PERF_DMA_EN_A::YES,
            false => PERF_DMA_EN_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == PERF_DMA_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PERF_DMA_EN_A::NO
    }
}
impl core::ops::Deref for PERF_DMA_EN_R {
    type Target = crate::FieldReader<bool, PERF_DMA_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERF_LUT_EN_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<PERF_LUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_LUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERF_LUT_EN` reader - "]
pub struct PERF_LUT_EN_R(crate::FieldReader<bool, PERF_LUT_EN_A>);
impl PERF_LUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERF_LUT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_LUT_EN_A {
        match self.bits {
            true => PERF_LUT_EN_A::YES,
            false => PERF_LUT_EN_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == PERF_LUT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PERF_LUT_EN_A::NO
    }
}
impl core::ops::Deref for PERF_LUT_EN_R {
    type Target = crate::FieldReader<bool, PERF_LUT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERF_SAT_EN_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<PERF_SAT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_SAT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERF_SAT_EN` reader - "]
pub struct PERF_SAT_EN_R(crate::FieldReader<bool, PERF_SAT_EN_A>);
impl PERF_SAT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERF_SAT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_SAT_EN_A {
        match self.bits {
            true => PERF_SAT_EN_A::YES,
            false => PERF_SAT_EN_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == PERF_SAT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PERF_SAT_EN_A::NO
    }
}
impl core::ops::Deref for PERF_SAT_EN_R {
    type Target = crate::FieldReader<bool, PERF_SAT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERF_NAN_INF_COUNT_EN_A {
    #[doc = "1: `1`"]
    YES = 1,
    #[doc = "0: `0`"]
    NO = 0,
}
impl From<PERF_NAN_INF_COUNT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PERF_NAN_INF_COUNT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERF_NAN_INF_COUNT_EN` reader - "]
pub struct PERF_NAN_INF_COUNT_EN_R(crate::FieldReader<bool, PERF_NAN_INF_COUNT_EN_A>);
impl PERF_NAN_INF_COUNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PERF_NAN_INF_COUNT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERF_NAN_INF_COUNT_EN_A {
        match self.bits {
            true => PERF_NAN_INF_COUNT_EN_A::YES,
            false => PERF_NAN_INF_COUNT_EN_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == PERF_NAN_INF_COUNT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == PERF_NAN_INF_COUNT_EN_A::NO
    }
}
impl core::ops::Deref for PERF_NAN_INF_COUNT_EN_R {
    type Target = crate::FieldReader<bool, PERF_NAN_INF_COUNT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn perf_dma_en(&self) -> PERF_DMA_EN_R {
        PERF_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn perf_lut_en(&self) -> PERF_LUT_EN_R {
        PERF_LUT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn perf_sat_en(&self) -> PERF_SAT_EN_R {
        PERF_SAT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn perf_nan_inf_count_en(&self) -> PERF_NAN_INF_COUNT_EN_R {
        PERF_NAN_INF_COUNT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Enable/Disable performance counting\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_enable](index.html) module"]
pub struct D_PERF_ENABLE_SPEC;
impl crate::RegisterSpec for D_PERF_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_enable::R](R) reader structure"]
impl crate::Readable for D_PERF_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PERF_ENABLE to value 0"]
impl crate::Resettable for D_PERF_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
