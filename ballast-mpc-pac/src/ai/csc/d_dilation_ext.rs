#[doc = "Register `D_DILATION_EXT` reader"]
pub struct R(crate::R<D_DILATION_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DILATION_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DILATION_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DILATION_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DILATION_EXT` writer"]
pub struct W(crate::W<D_DILATION_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DILATION_EXT_SPEC>;
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
impl From<crate::W<D_DILATION_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DILATION_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X_DILATION_EXT` reader - "]
pub type X_DILATION_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X_DILATION_EXT` writer - "]
pub type X_DILATION_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DILATION_EXT_SPEC, u8, u8, 5, O>;
#[doc = "Field `Y_DILATION_EXT` reader - "]
pub type Y_DILATION_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Y_DILATION_EXT` writer - "]
pub type Y_DILATION_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DILATION_EXT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn x_dilation_ext(&self) -> X_DILATION_EXT_R {
        X_DILATION_EXT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn y_dilation_ext(&self) -> Y_DILATION_EXT_R {
        Y_DILATION_EXT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn x_dilation_ext(&mut self) -> X_DILATION_EXT_W<0> {
        X_DILATION_EXT_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn y_dilation_ext(&mut self) -> Y_DILATION_EXT_W<16> {
        Y_DILATION_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dilation parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dilation_ext](index.html) module"]
pub struct D_DILATION_EXT_SPEC;
impl crate::RegisterSpec for D_DILATION_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dilation_ext::R](R) reader structure"]
impl crate::Readable for D_DILATION_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dilation_ext::W](W) writer structure"]
impl crate::Writable for D_DILATION_EXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DILATION_EXT to value 0"]
impl crate::Resettable for D_DILATION_EXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
