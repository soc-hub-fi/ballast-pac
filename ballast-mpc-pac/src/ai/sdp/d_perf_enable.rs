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
#[doc = "Register `D_PERF_ENABLE` writer"]
pub struct W(crate::W<D_PERF_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_PERF_ENABLE_SPEC>;
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
impl From<crate::W<D_PERF_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_PERF_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERF_DMA_EN` reader - "]
pub type PERF_DMA_EN_R = crate::BitReader<PERF_DMA_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PERF_DMA_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PERF_DMA_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PERF_DMA_EN_A::NO
    }
}
#[doc = "Field `PERF_DMA_EN` writer - "]
pub type PERF_DMA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_PERF_ENABLE_SPEC, PERF_DMA_EN_A, O>;
impl<'a, const O: u8> PERF_DMA_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(PERF_DMA_EN_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PERF_DMA_EN_A::NO)
    }
}
#[doc = "Field `PERF_LUT_EN` reader - "]
pub type PERF_LUT_EN_R = crate::BitReader<PERF_LUT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PERF_LUT_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PERF_LUT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PERF_LUT_EN_A::NO
    }
}
#[doc = "Field `PERF_LUT_EN` writer - "]
pub type PERF_LUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_PERF_ENABLE_SPEC, PERF_LUT_EN_A, O>;
impl<'a, const O: u8> PERF_LUT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(PERF_LUT_EN_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PERF_LUT_EN_A::NO)
    }
}
#[doc = "Field `PERF_SAT_EN` reader - "]
pub type PERF_SAT_EN_R = crate::BitReader<PERF_SAT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PERF_SAT_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PERF_SAT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PERF_SAT_EN_A::NO
    }
}
#[doc = "Field `PERF_SAT_EN` writer - "]
pub type PERF_SAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_PERF_ENABLE_SPEC, PERF_SAT_EN_A, O>;
impl<'a, const O: u8> PERF_SAT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(PERF_SAT_EN_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PERF_SAT_EN_A::NO)
    }
}
#[doc = "Field `PERF_NAN_INF_COUNT_EN` reader - "]
pub type PERF_NAN_INF_COUNT_EN_R = crate::BitReader<PERF_NAN_INF_COUNT_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PERF_NAN_INF_COUNT_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PERF_NAN_INF_COUNT_EN_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PERF_NAN_INF_COUNT_EN_A::NO
    }
}
#[doc = "Field `PERF_NAN_INF_COUNT_EN` writer - "]
pub type PERF_NAN_INF_COUNT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_PERF_ENABLE_SPEC, PERF_NAN_INF_COUNT_EN_A, O>;
impl<'a, const O: u8> PERF_NAN_INF_COUNT_EN_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(PERF_NAN_INF_COUNT_EN_A::YES)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PERF_NAN_INF_COUNT_EN_A::NO)
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn perf_dma_en(&mut self) -> PERF_DMA_EN_W<0> {
        PERF_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn perf_lut_en(&mut self) -> PERF_LUT_EN_W<1> {
        PERF_LUT_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn perf_sat_en(&mut self) -> PERF_SAT_EN_W<2> {
        PERF_SAT_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn perf_nan_inf_count_en(&mut self) -> PERF_NAN_INF_COUNT_EN_W<3> {
        PERF_NAN_INF_COUNT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable/Disable performance counting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_enable](index.html) module"]
pub struct D_PERF_ENABLE_SPEC;
impl crate::RegisterSpec for D_PERF_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_perf_enable::R](R) reader structure"]
impl crate::Readable for D_PERF_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_perf_enable::W](W) writer structure"]
impl crate::Writable for D_PERF_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_PERF_ENABLE to value 0"]
impl crate::Resettable for D_PERF_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
