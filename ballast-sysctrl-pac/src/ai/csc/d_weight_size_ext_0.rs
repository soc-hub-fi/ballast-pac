#[doc = "Register `D_WEIGHT_SIZE_EXT_0` reader"]
pub struct R(crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_WEIGHT_SIZE_EXT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_WEIGHT_SIZE_EXT_0` writer"]
pub struct W(crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>;
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
impl From<crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_WEIGHT_SIZE_EXT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEIGHT_WIDTH_EXT` reader - "]
pub struct WEIGHT_WIDTH_EXT_R(crate::FieldReader<u8, u8>);
impl WEIGHT_WIDTH_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_WIDTH_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_WIDTH_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_WIDTH_EXT` writer - "]
pub struct WEIGHT_WIDTH_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_WIDTH_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `WEIGHT_HEIGHT_EXT` reader - "]
pub struct WEIGHT_HEIGHT_EXT_R(crate::FieldReader<u8, u8>);
impl WEIGHT_HEIGHT_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_HEIGHT_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_HEIGHT_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_HEIGHT_EXT` writer - "]
pub struct WEIGHT_HEIGHT_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_HEIGHT_EXT_W<'a> {
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
    pub fn weight_width_ext(&self) -> WEIGHT_WIDTH_EXT_R {
        WEIGHT_WIDTH_EXT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_height_ext(&self) -> WEIGHT_HEIGHT_EXT_R {
        WEIGHT_HEIGHT_EXT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn weight_width_ext(&mut self) -> WEIGHT_WIDTH_EXT_W {
        WEIGHT_WIDTH_EXT_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_height_ext(&mut self) -> WEIGHT_HEIGHT_EXT_W {
        WEIGHT_HEIGHT_EXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Weight’s width and height after extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_weight_size_ext_0](index.html) module"]
pub struct D_WEIGHT_SIZE_EXT_0_SPEC;
impl crate::RegisterSpec for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_weight_size_ext_0::R](R) reader structure"]
impl crate::Readable for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_weight_size_ext_0::W](W) writer structure"]
impl crate::Writable for D_WEIGHT_SIZE_EXT_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_WEIGHT_SIZE_EXT_0 to value 0"]
impl crate::Resettable for D_WEIGHT_SIZE_EXT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
