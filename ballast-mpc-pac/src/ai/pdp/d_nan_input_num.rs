#[doc = "Register `D_NAN_INPUT_NUM` reader"]
pub struct R(crate::R<D_NAN_INPUT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_NAN_INPUT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_NAN_INPUT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_NAN_INPUT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NAN_INPUT_NUM` reader - "]
pub type NAN_INPUT_NUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nan_input_num(&self) -> NAN_INPUT_NUM_R {
        NAN_INPUT_NUM_R::new(self.bits)
    }
}
#[doc = "Input NaN element number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_nan_input_num](index.html) module"]
pub struct D_NAN_INPUT_NUM_SPEC;
impl crate::RegisterSpec for D_NAN_INPUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_nan_input_num::R](R) reader structure"]
impl crate::Readable for D_NAN_INPUT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_NAN_INPUT_NUM to value 0"]
impl crate::Resettable for D_NAN_INPUT_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
