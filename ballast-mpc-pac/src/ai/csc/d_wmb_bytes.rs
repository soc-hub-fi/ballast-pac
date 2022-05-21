#[doc = "Register `D_WMB_BYTES` reader"]
pub struct R(crate::R<D_WMB_BYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WMB_BYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WMB_BYTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WMB_BYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WMB_BYTES` reader - "]
pub struct WMB_BYTES_R(crate::FieldReader<u32, u32>);
impl WMB_BYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WMB_BYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMB_BYTES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn wmb_bytes(&self) -> WMB_BYTES_R {
        WMB_BYTES_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
#[doc = "Total bytes of WMB\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_wmb_bytes](index.html) module"]
pub struct D_WMB_BYTES_SPEC;
impl crate::RegisterSpec for D_WMB_BYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_wmb_bytes::R](R) reader structure"]
impl crate::Readable for D_WMB_BYTES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_WMB_BYTES to value 0"]
impl crate::Resettable for D_WMB_BYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
