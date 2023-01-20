#[doc = "Register `D_RELEASE` reader"]
pub struct R(crate::R<D_RELEASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_RELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_RELEASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_RELEASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_RELEASE` writer"]
pub struct W(crate::W<D_RELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_RELEASE_SPEC>;
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
impl From<crate::W<D_RELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_RELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLS_SLICES` reader - "]
pub type RLS_SLICES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RLS_SLICES` writer - "]
pub type RLS_SLICES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_RELEASE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rls_slices(&self) -> RLS_SLICES_R {
        RLS_SLICES_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn rls_slices(&mut self) -> RLS_SLICES_W<0> {
        RLS_SLICES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slices of CBUF to be released at the end of current layer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_release](index.html) module"]
pub struct D_RELEASE_SPEC;
impl crate::RegisterSpec for D_RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_release::R](R) reader structure"]
impl crate::Readable for D_RELEASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_release::W](W) writer structure"]
impl crate::Writable for D_RELEASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_RELEASE to value 0x01"]
impl crate::Resettable for D_RELEASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
