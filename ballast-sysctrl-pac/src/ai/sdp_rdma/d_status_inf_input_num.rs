#[doc = "Register `D_STATUS_INF_INPUT_NUM` reader"]
pub struct R(crate::R<D_STATUS_INF_INPUT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_STATUS_INF_INPUT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_STATUS_INF_INPUT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_STATUS_INF_INPUT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_STATUS_INF_INPUT_NUM` writer"]
pub struct W(crate::W<D_STATUS_INF_INPUT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_STATUS_INF_INPUT_NUM_SPEC>;
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
impl From<crate::W<D_STATUS_INF_INPUT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_STATUS_INF_INPUT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS_INF_INPUT_NUM` reader - "]
pub type STATUS_INF_INPUT_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATUS_INF_INPUT_NUM` writer - "]
pub type STATUS_INF_INPUT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_STATUS_INF_INPUT_NUM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status_inf_input_num(&self) -> STATUS_INF_INPUT_NUM_R {
        STATUS_INF_INPUT_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn status_inf_input_num(&mut self) -> STATUS_INF_INPUT_NUM_W<0> {
        STATUS_INF_INPUT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_status_inf_input_num](index.html) module"]
pub struct D_STATUS_INF_INPUT_NUM_SPEC;
impl crate::RegisterSpec for D_STATUS_INF_INPUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_status_inf_input_num::R](R) reader structure"]
impl crate::Readable for D_STATUS_INF_INPUT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_status_inf_input_num::W](W) writer structure"]
impl crate::Writable for D_STATUS_INF_INPUT_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_STATUS_INF_INPUT_NUM to value 0"]
impl crate::Resettable for D_STATUS_INF_INPUT_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
