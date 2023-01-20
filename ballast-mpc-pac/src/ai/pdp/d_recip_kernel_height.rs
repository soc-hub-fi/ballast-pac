#[doc = "Register `D_RECIP_KERNEL_HEIGHT` reader"]
pub struct R(crate::R<D_RECIP_KERNEL_HEIGHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_RECIP_KERNEL_HEIGHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_RECIP_KERNEL_HEIGHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_RECIP_KERNEL_HEIGHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_RECIP_KERNEL_HEIGHT` writer"]
pub struct W(crate::W<D_RECIP_KERNEL_HEIGHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_RECIP_KERNEL_HEIGHT_SPEC>;
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
impl From<crate::W<D_RECIP_KERNEL_HEIGHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_RECIP_KERNEL_HEIGHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECIP_KERNEL_HEIGHT` reader - "]
pub type RECIP_KERNEL_HEIGHT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RECIP_KERNEL_HEIGHT` writer - "]
pub type RECIP_KERNEL_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_RECIP_KERNEL_HEIGHT_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn recip_kernel_height(&self) -> RECIP_KERNEL_HEIGHT_R {
        RECIP_KERNEL_HEIGHT_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    #[must_use]
    pub fn recip_kernel_height(&mut self) -> RECIP_KERNEL_HEIGHT_W<0> {
        RECIP_KERNEL_HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reciprocal of pooling kernel height, set to actual value * (2^16) when INT8/INT16 format enabled. and set to actual value for fp16 precision mode with fp17 data format.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_recip_kernel_height](index.html) module"]
pub struct D_RECIP_KERNEL_HEIGHT_SPEC;
impl crate::RegisterSpec for D_RECIP_KERNEL_HEIGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_recip_kernel_height::R](R) reader structure"]
impl crate::Readable for D_RECIP_KERNEL_HEIGHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_recip_kernel_height::W](W) writer structure"]
impl crate::Writable for D_RECIP_KERNEL_HEIGHT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_RECIP_KERNEL_HEIGHT to value 0"]
impl crate::Resettable for D_RECIP_KERNEL_HEIGHT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
