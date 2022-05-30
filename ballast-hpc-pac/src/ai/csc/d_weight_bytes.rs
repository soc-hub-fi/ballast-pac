#[doc = "Register `D_WEIGHT_BYTES` reader"]
pub struct R(crate::R<D_WEIGHT_BYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_BYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_BYTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_BYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_BYTES` writer"]
pub struct W(crate::W<D_WEIGHT_BYTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_BYTES_SPEC>;
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
impl From<crate::W<D_WEIGHT_BYTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_BYTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEIGHT_BYTES` reader - "]
pub struct WEIGHT_BYTES_R(crate::FieldReader<u32>);
impl WEIGHT_BYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WEIGHT_BYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_BYTES_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_BYTES` writer - "]
pub struct WEIGHT_BYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_BYTES_W<'a> {
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
    pub fn weight_bytes(&self) -> WEIGHT_BYTES_R {
        WEIGHT_BYTES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn weight_bytes(&mut self) -> WEIGHT_BYTES_W {
        WEIGHT_BYTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Total bytes of Weight\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_bytes](index.html) module"]
pub struct D_WEIGHT_BYTES_SPEC;
impl crate::RegisterSpec for D_WEIGHT_BYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_bytes::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_BYTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_bytes::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_BYTES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_WEIGHT_BYTES to value 0"]
impl crate::Resettable for D_WEIGHT_BYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
