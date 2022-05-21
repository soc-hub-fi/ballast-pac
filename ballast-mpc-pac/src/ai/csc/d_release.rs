#[doc = "Register `D_RELEASE` reader"]
pub struct R(crate::R<D_RELEASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_RELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_RELEASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_RELEASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RLS_SLICES` reader - "]
pub struct RLS_SLICES_R(crate::FieldReader<u16, u16>);
impl RLS_SLICES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RLS_SLICES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLS_SLICES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rls_slices(&self) -> RLS_SLICES_R {
        RLS_SLICES_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slices of CBUF to be released at the end of current layer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_release](index.html) module"]
pub struct D_RELEASE_SPEC;
impl crate::RegisterSpec for D_RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_release::R](R) reader structure"]
impl crate::Readable for D_RELEASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_RELEASE to value 0x01"]
impl crate::Resettable for D_RELEASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
