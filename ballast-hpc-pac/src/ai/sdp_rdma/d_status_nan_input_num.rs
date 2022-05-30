#[doc = "Register `D_STATUS_NAN_INPUT_NUM` reader"]
pub struct R(crate::R<D_STATUS_NAN_INPUT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_STATUS_NAN_INPUT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_STATUS_NAN_INPUT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_STATUS_NAN_INPUT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_STATUS_NAN_INPUT_NUM` writer"]
pub struct W(crate::W<D_STATUS_NAN_INPUT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_STATUS_NAN_INPUT_NUM_SPEC>;
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
impl From<crate::W<D_STATUS_NAN_INPUT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_STATUS_NAN_INPUT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS_NAN_INPUT_NUM` reader - "]
pub struct STATUS_NAN_INPUT_NUM_R(crate::FieldReader<u32>);
impl STATUS_NAN_INPUT_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        STATUS_NAN_INPUT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_NAN_INPUT_NUM_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_NAN_INPUT_NUM` writer - "]
pub struct STATUS_NAN_INPUT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_NAN_INPUT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status_nan_input_num(&self) -> STATUS_NAN_INPUT_NUM_R {
        STATUS_NAN_INPUT_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status_nan_input_num(&mut self) -> STATUS_NAN_INPUT_NUM_W {
        STATUS_NAN_INPUT_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_status_nan_input_num](index.html) module"]
pub struct D_STATUS_NAN_INPUT_NUM_SPEC;
impl crate::RegisterSpec for D_STATUS_NAN_INPUT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_status_nan_input_num::R](R) reader structure"]
impl crate::Readable for D_STATUS_NAN_INPUT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_status_nan_input_num::W](W) writer structure"]
impl crate::Writable for D_STATUS_NAN_INPUT_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_STATUS_NAN_INPUT_NUM to value 0"]
impl crate::Resettable for D_STATUS_NAN_INPUT_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
