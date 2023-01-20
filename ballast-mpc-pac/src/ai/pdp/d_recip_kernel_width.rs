#[doc = "Register `D_RECIP_KERNEL_WIDTH` reader"]
pub struct R(crate::R<D_RECIP_KERNEL_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_RECIP_KERNEL_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_RECIP_KERNEL_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_RECIP_KERNEL_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_RECIP_KERNEL_WIDTH` writer"]
pub struct W(crate::W<D_RECIP_KERNEL_WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_RECIP_KERNEL_WIDTH_SPEC>;
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
impl From<crate::W<D_RECIP_KERNEL_WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_RECIP_KERNEL_WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECIP_KERNEL_WIDTH` reader - "]
pub type RECIP_KERNEL_WIDTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RECIP_KERNEL_WIDTH` writer - "]
pub type RECIP_KERNEL_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_RECIP_KERNEL_WIDTH_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn recip_kernel_width(&self) -> RECIP_KERNEL_WIDTH_R {
        RECIP_KERNEL_WIDTH_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    #[must_use]
    pub fn recip_kernel_width(&mut self) -> RECIP_KERNEL_WIDTH_W<0> {
        RECIP_KERNEL_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reciprocal of pooling kernel width, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_recip_kernel_width](index.html) module"]
pub struct D_RECIP_KERNEL_WIDTH_SPEC;
impl crate::RegisterSpec for D_RECIP_KERNEL_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_recip_kernel_width::R](R) reader structure"]
impl crate::Readable for D_RECIP_KERNEL_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_recip_kernel_width::W](W) writer structure"]
impl crate::Writable for D_RECIP_KERNEL_WIDTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_RECIP_KERNEL_WIDTH to value 0"]
impl crate::Resettable for D_RECIP_KERNEL_WIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
