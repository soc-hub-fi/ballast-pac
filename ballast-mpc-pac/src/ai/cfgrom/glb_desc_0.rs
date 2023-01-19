#[doc = "Register `GLB_DESC_0` reader"]
pub struct R(crate::R<GLB_DESC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLB_DESC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLB_DESC_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLB_DESC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GLB_DESC` reader - "]
pub type GLB_DESC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn glb_desc(&self) -> GLB_DESC_R {
        GLB_DESC_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_desc_0](index.html) module"]
pub struct GLB_DESC_0_SPEC;
impl crate::RegisterSpec for GLB_DESC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_desc_0::R](R) reader structure"]
impl crate::Readable for GLB_DESC_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GLB_DESC_0 to value 0x01"]
impl crate::Resettable for GLB_DESC_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
