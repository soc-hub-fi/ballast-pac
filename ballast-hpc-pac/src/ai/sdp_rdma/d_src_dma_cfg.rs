#[doc = "Register `D_SRC_DMA_CFG` reader"]
pub struct R(crate::R<D_SRC_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SRC_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SRC_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SRC_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_SRC_DMA_CFG` writer"]
pub struct W(crate::W<D_SRC_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SRC_DMA_CFG_SPEC>;
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
impl From<crate::W<D_SRC_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SRC_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_RAM_TYPE` reader - "]
pub type SRC_RAM_TYPE_R = crate::BitReader<SRC_RAM_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_RAM_TYPE_A {
    #[doc = "1: `1`"]
    MC = 1,
    #[doc = "0: `0`"]
    CV = 0,
}
impl From<SRC_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_RAM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_RAM_TYPE_A {
        match self.bits {
            true => SRC_RAM_TYPE_A::MC,
            false => SRC_RAM_TYPE_A::CV,
        }
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        *self == SRC_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        *self == SRC_RAM_TYPE_A::CV
    }
}
#[doc = "Field `SRC_RAM_TYPE` writer - "]
pub type SRC_RAM_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_SRC_DMA_CFG_SPEC, SRC_RAM_TYPE_A, O>;
impl<'a, const O: u8> SRC_RAM_TYPE_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(SRC_RAM_TYPE_A::MC)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(SRC_RAM_TYPE_A::CV)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn src_ram_type(&self) -> SRC_RAM_TYPE_R {
        SRC_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn src_ram_type(&mut self) -> SRC_RAM_TYPE_W<0> {
        SRC_RAM_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_src_dma_cfg](index.html) module"]
pub struct D_SRC_DMA_CFG_SPEC;
impl crate::RegisterSpec for D_SRC_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_src_dma_cfg::R](R) reader structure"]
impl crate::Readable for D_SRC_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_src_dma_cfg::W](W) writer structure"]
impl crate::Writable for D_SRC_DMA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_SRC_DMA_CFG to value 0"]
impl crate::Resettable for D_SRC_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
