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
pub struct RLS_SLICES_R(crate::FieldReader<u16, u16>);
impl RLS_SLICES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RLS_SLICES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLS_SLICES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLS_SLICES` writer - "]
pub struct RLS_SLICES_W<'a> {
    w: &'a mut W,
}
impl<'a> RLS_SLICES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
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
    pub fn rls_slices(&mut self) -> RLS_SLICES_W {
        RLS_SLICES_W { w: self }
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
}
#[doc = "`reset()` method sets D_RELEASE to value 0x01"]
impl crate::Resettable for D_RELEASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
