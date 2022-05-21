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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SRC_RAM_TYPE` reader - "]
pub struct SRC_RAM_TYPE_R(crate::FieldReader<bool, SRC_RAM_TYPE_A>);
impl SRC_RAM_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRC_RAM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SRC_RAM_TYPE_A::MC
    }
    #[doc = "Checks if the value of the field is `CV`"]
    #[inline(always)]
    pub fn is_cv(&self) -> bool {
        **self == SRC_RAM_TYPE_A::CV
    }
}
impl core::ops::Deref for SRC_RAM_TYPE_R {
    type Target = crate::FieldReader<bool, SRC_RAM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn src_ram_type(&self) -> SRC_RAM_TYPE_R {
        SRC_RAM_TYPE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_src_dma_cfg](index.html) module"]
pub struct D_SRC_DMA_CFG_SPEC;
impl crate::RegisterSpec for D_SRC_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_src_dma_cfg::R](R) reader structure"]
impl crate::Readable for D_SRC_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_SRC_DMA_CFG to value 0"]
impl crate::Resettable for D_SRC_DMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
