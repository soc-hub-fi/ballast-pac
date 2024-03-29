#[doc = "Register `CBUF_CAP_INCOMPAT_0` reader"]
pub struct R(crate::R<CBUF_CAP_INCOMPAT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBUF_CAP_INCOMPAT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBUF_CAP_INCOMPAT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBUF_CAP_INCOMPAT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CBUF_CAP_INCOMPAT` reader - "]
pub type CBUF_CAP_INCOMPAT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cbuf_cap_incompat(&self) -> CBUF_CAP_INCOMPAT_R {
        CBUF_CAP_INCOMPAT_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbuf_cap_incompat_0](index.html) module"]
pub struct CBUF_CAP_INCOMPAT_0_SPEC;
impl crate::RegisterSpec for CBUF_CAP_INCOMPAT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbuf_cap_incompat_0::R](R) reader structure"]
impl crate::Readable for CBUF_CAP_INCOMPAT_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CBUF_CAP_INCOMPAT_0 to value 0"]
impl crate::Resettable for CBUF_CAP_INCOMPAT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
