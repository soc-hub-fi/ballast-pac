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
pub struct X_DILATION_EXT_R(crate::FieldReader<u8, u8>);
impl X_DILATION_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        X_DILATION_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for X_DILATION_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `X_DILATION_EXT` writer - "]
pub struct X_DILATION_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> X_DILATION_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `Y_DILATION_EXT` reader - "]
pub struct Y_DILATION_EXT_R(crate::FieldReader<u8, u8>);
impl Y_DILATION_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        Y_DILATION_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Y_DILATION_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Y_DILATION_EXT` writer - "]
pub struct Y_DILATION_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_DILATION_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
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
    pub fn x_dilation_ext(&mut self) -> X_DILATION_EXT_W {
        X_DILATION_EXT_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn y_dilation_ext(&mut self) -> Y_DILATION_EXT_W {
        Y_DILATION_EXT_W { w: self }
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
}
#[doc = "`reset()` method sets D_DILATION_EXT to value 0"]
impl crate::Resettable for D_DILATION_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
