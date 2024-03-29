#[doc = "Register `D_FETCH_GRAIN` reader"]
pub struct R(crate::R<D_FETCH_GRAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_FETCH_GRAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_FETCH_GRAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_FETCH_GRAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_FETCH_GRAIN` writer"]
pub struct W(crate::W<D_FETCH_GRAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_FETCH_GRAIN_SPEC>;
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
impl From<crate::W<D_FETCH_GRAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_FETCH_GRAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRAINS` reader - "]
pub type GRAINS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GRAINS` writer - "]
pub type GRAINS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_FETCH_GRAIN_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn grains(&self) -> GRAINS_R {
        GRAINS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn grains(&mut self) -> GRAINS_W<0> {
        GRAINS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of slices to be fetched before sending update information to CSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_fetch_grain](index.html) module"]
pub struct D_FETCH_GRAIN_SPEC;
impl crate::RegisterSpec for D_FETCH_GRAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_fetch_grain::R](R) reader structure"]
impl crate::Readable for D_FETCH_GRAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_fetch_grain::W](W) writer structure"]
impl crate::Writable for D_FETCH_GRAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_FETCH_GRAIN to value 0"]
impl crate::Resettable for D_FETCH_GRAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
