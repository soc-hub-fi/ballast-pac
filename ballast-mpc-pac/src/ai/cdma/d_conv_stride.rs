#[doc = "Register `D_CONV_STRIDE` reader"]
pub struct R(crate::R<D_CONV_STRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CONV_STRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CONV_STRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CONV_STRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_CONV_STRIDE` writer"]
pub struct W(crate::W<D_CONV_STRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CONV_STRIDE_SPEC>;
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
impl From<crate::W<D_CONV_STRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CONV_STRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_X_STRIDE` reader - "]
pub type CONV_X_STRIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONV_X_STRIDE` writer - "]
pub type CONV_X_STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CONV_STRIDE_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONV_Y_STRIDE` reader - "]
pub type CONV_Y_STRIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONV_Y_STRIDE` writer - "]
pub type CONV_Y_STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CONV_STRIDE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn conv_x_stride(&self) -> CONV_X_STRIDE_R {
        CONV_X_STRIDE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn conv_y_stride(&self) -> CONV_Y_STRIDE_R {
        CONV_Y_STRIDE_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn conv_x_stride(&mut self) -> CONV_X_STRIDE_W<0> {
        CONV_X_STRIDE_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn conv_y_stride(&mut self) -> CONV_Y_STRIDE_W<16> {
        CONV_Y_STRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Convolution x stride and convolution y stride\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_conv_stride](index.html) module"]
pub struct D_CONV_STRIDE_SPEC;
impl crate::RegisterSpec for D_CONV_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_conv_stride::R](R) reader structure"]
impl crate::Readable for D_CONV_STRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_conv_stride::W](W) writer structure"]
impl crate::Writable for D_CONV_STRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_CONV_STRIDE to value 0"]
impl crate::Resettable for D_CONV_STRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
