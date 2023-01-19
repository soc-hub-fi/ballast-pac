#[doc = "Register `D_BATCH_NUMBER` reader"]
pub struct R(crate::R<D_BATCH_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BATCH_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BATCH_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BATCH_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_BATCH_NUMBER` writer"]
pub struct W(crate::W<D_BATCH_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_BATCH_NUMBER_SPEC>;
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
impl From<crate::W<D_BATCH_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_BATCH_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BATCHES` reader - "]
pub type BATCHES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BATCHES` writer - "]
pub type BATCHES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_BATCH_NUMBER_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn batches(&self) -> BATCHES_R {
        BATCHES_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn batches(&mut self) -> BATCHES_W<0> {
        BATCHES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of batches\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_batch_number](index.html) module"]
pub struct D_BATCH_NUMBER_SPEC;
impl crate::RegisterSpec for D_BATCH_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_batch_number::R](R) reader structure"]
impl crate::Readable for D_BATCH_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_batch_number::W](W) writer structure"]
impl crate::Writable for D_BATCH_NUMBER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_BATCH_NUMBER to value 0"]
impl crate::Resettable for D_BATCH_NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
