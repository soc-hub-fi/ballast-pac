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
#[doc = "Field `DMA_EN` reader - "]
pub struct DMA_EN_R(crate::FieldReader<bool>);
impl DMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_EN` writer - "]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable/disable performance counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_perf_enable](index.html) module"]
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
}
#[doc = "`reset()` method sets D_PERF_ENABLE to value 0"]
impl crate::Resettable for D_PERF_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
