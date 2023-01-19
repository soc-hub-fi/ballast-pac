#[doc = "Register `D_DST_DMA_CFG` reader"]
pub struct R(crate::R<D_DST_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DST_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DST_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DST_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DST_DMA_CFG` writer"]
pub struct W(crate::W<D_DST_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DST_DMA_CFG_SPEC>;
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
impl From<crate::W<D_DST_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DST_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST_RAM_TYPE` reader - "]
pub type DST_RAM_TYPE_R = crate::BitReader<DST_RAM_TYPE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_RAM_TYPE_A {
    #[doc = "0: `0`"]
    CV = 0,
    #[doc = "1: `1`"]
    MC = 1,
}
impl From<DST_RAM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DST_RAM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_RAM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_RAM_TYPE_A {
        match self.bits {
            false => DST_RAM_TYPE_A::CV,
            true => DST_RAM_TYPE_A::MC,
        }
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        *self == DST_RAM_TYPE_A::CV
    }
    #[doc = "Checks if the value of the field is `MC`"]
    #[inline(always)]
    pub fn is_mc(&self) -> bool {
        *self == DST_RAM_TYPE_A::MC
    }
}
#[doc = "Field `DST_RAM_TYPE` writer - "]
pub type DST_RAM_TYPE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, D_DST_DMA_CFG_SPEC, DST_RAM_TYPE_A, O>;
impl<'a, const O: u8> DST_RAM_TYPE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cv(self) -> &'a mut W {
        self.variant(DST_RAM_TYPE_A::CV)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mc(self) -> &'a mut W {
        self.variant(DST_RAM_TYPE_A::MC)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dst_ram_type(&self) -> DST_RAM_TYPE_R {
        DST_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dst_ram_type(&mut self) -> DST_RAM_TYPE_W<0> {
        DST_RAM_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination RAM type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dst_dma_cfg](index.html) module"]
pub struct D_DST_DMA_CFG_SPEC;
impl crate::RegisterSpec for D_DST_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dst_dma_cfg::R](R) reader structure"]
impl crate::Readable for D_DST_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dst_dma_cfg::W](W) writer structure"]
impl crate::Writable for D_DST_DMA_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DST_DMA_CFG to value 0"]
impl crate::Resettable for D_DST_DMA_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
