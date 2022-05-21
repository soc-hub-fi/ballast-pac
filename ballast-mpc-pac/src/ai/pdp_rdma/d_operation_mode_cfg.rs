#[doc = "Register `D_OPERATION_MODE_CFG` reader"]
pub struct R(crate::R<D_OPERATION_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_OPERATION_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_OPERATION_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_OPERATION_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPLIT_NUM` reader - "]
pub struct SPLIT_NUM_R(crate::FieldReader<u8, u8>);
impl SPLIT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPLIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLIT_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn split_num(&self) -> SPLIT_NUM_R {
        SPLIT_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_operation_mode_cfg](index.html) module"]
pub struct D_OPERATION_MODE_CFG_SPEC;
impl crate::RegisterSpec for D_OPERATION_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_operation_mode_cfg::R](R) reader structure"]
impl crate::Readable for D_OPERATION_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_OPERATION_MODE_CFG to value 0"]
impl crate::Resettable for D_OPERATION_MODE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
