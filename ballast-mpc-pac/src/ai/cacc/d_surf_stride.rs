#[doc = "Register `D_SURF_STRIDE` reader"]
pub struct R(crate::R<D_SURF_STRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SURF_STRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SURF_STRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SURF_STRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_SURF_STRIDE` writer"]
pub struct W(crate::W<D_SURF_STRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SURF_STRIDE_SPEC>;
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
impl From<crate::W<D_SURF_STRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SURF_STRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SURF_STRIDE` reader - "]
pub type SURF_STRIDE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SURF_STRIDE` writer - "]
pub type SURF_STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_SURF_STRIDE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn surf_stride(&self) -> SURF_STRIDE_R {
        SURF_STRIDE_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn surf_stride(&mut self) -> SURF_STRIDE_W<0> {
        SURF_STRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line stride of surface cube\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_surf_stride](index.html) module"]
pub struct D_SURF_STRIDE_SPEC;
impl crate::RegisterSpec for D_SURF_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_surf_stride::R](R) reader structure"]
impl crate::Readable for D_SURF_STRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_surf_stride::W](W) writer structure"]
impl crate::Writable for D_SURF_STRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_SURF_STRIDE to value 0"]
impl crate::Resettable for D_SURF_STRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
