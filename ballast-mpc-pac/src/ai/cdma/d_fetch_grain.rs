#[doc = "Register `D_FETCH_GRAIN` reader"]
pub struct R(crate::R<D_FETCH_GRAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FETCH_GRAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FETCH_GRAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FETCH_GRAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GRAINS` reader - "]
pub struct GRAINS_R(crate::FieldReader<u16, u16>);
impl GRAINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GRAINS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRAINS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn grains(&self) -> GRAINS_R {
        GRAINS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Number of slices to be fetched before sending update information to CSC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_fetch_grain](index.html) module"]
pub struct D_FETCH_GRAIN_SPEC;
impl crate::RegisterSpec for D_FETCH_GRAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_fetch_grain::R](R) reader structure"]
impl crate::Readable for D_FETCH_GRAIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_FETCH_GRAIN to value 0"]
impl crate::Resettable for D_FETCH_GRAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
