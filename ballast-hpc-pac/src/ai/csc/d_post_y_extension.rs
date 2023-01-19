#[doc = "Register `D_POST_Y_EXTENSION` reader"]
pub struct R(crate::R<D_POST_Y_EXTENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POST_Y_EXTENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POST_Y_EXTENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POST_Y_EXTENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_POST_Y_EXTENSION` writer"]
pub struct W(crate::W<D_POST_Y_EXTENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_POST_Y_EXTENSION_SPEC>;
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
impl From<crate::W<D_POST_Y_EXTENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_POST_Y_EXTENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y_EXTENSION` reader - "]
pub type Y_EXTENSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y_EXTENSION` writer - "]
pub type Y_EXTENSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POST_Y_EXTENSION_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn y_extension(&self) -> Y_EXTENSION_R {
        Y_EXTENSION_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn y_extension(&mut self) -> Y_EXTENSION_W<0> {
        Y_EXTENSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Post extension parameter for image-in\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_post_y_extension](index.html) module"]
pub struct D_POST_Y_EXTENSION_SPEC;
impl crate::RegisterSpec for D_POST_Y_EXTENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_post_y_extension::R](R) reader structure"]
impl crate::Readable for D_POST_Y_EXTENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_post_y_extension::W](W) writer structure"]
impl crate::Writable for D_POST_Y_EXTENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_POST_Y_EXTENSION to value 0"]
impl crate::Resettable for D_POST_Y_EXTENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
