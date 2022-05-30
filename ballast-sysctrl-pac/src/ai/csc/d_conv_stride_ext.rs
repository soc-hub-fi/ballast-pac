#[doc = "Register `D_CONV_STRIDE_EXT` reader"]
pub struct R(crate::R<D_CONV_STRIDE_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CONV_STRIDE_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CONV_STRIDE_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CONV_STRIDE_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_CONV_STRIDE_EXT` writer"]
pub struct W(crate::W<D_CONV_STRIDE_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CONV_STRIDE_EXT_SPEC>;
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
impl From<crate::W<D_CONV_STRIDE_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CONV_STRIDE_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_X_STRIDE_EXT` reader - "]
pub struct CONV_X_STRIDE_EXT_R(crate::FieldReader<u8>);
impl CONV_X_STRIDE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONV_X_STRIDE_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONV_X_STRIDE_EXT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONV_X_STRIDE_EXT` writer - "]
pub struct CONV_X_STRIDE_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_X_STRIDE_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `CONV_Y_STRIDE_EXT` reader - "]
pub struct CONV_Y_STRIDE_EXT_R(crate::FieldReader<u8>);
impl CONV_Y_STRIDE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONV_Y_STRIDE_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONV_Y_STRIDE_EXT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONV_Y_STRIDE_EXT` writer - "]
pub struct CONV_Y_STRIDE_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_Y_STRIDE_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn conv_x_stride_ext(&self) -> CONV_X_STRIDE_EXT_R {
        CONV_X_STRIDE_EXT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn conv_y_stride_ext(&self) -> CONV_Y_STRIDE_EXT_R {
        CONV_Y_STRIDE_EXT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn conv_x_stride_ext(&mut self) -> CONV_X_STRIDE_EXT_W {
        CONV_X_STRIDE_EXT_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn conv_y_stride_ext(&mut self) -> CONV_Y_STRIDE_EXT_W {
        CONV_Y_STRIDE_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Convolution x stride and convolution y stride after extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_conv_stride_ext](index.html) module"]
pub struct D_CONV_STRIDE_EXT_SPEC;
impl crate::RegisterSpec for D_CONV_STRIDE_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_conv_stride_ext::R](R) reader structure"]
impl crate::Readable for D_CONV_STRIDE_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_conv_stride_ext::W](W) writer structure"]
impl crate::Writable for D_CONV_STRIDE_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_CONV_STRIDE_EXT to value 0"]
impl crate::Resettable for D_CONV_STRIDE_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
