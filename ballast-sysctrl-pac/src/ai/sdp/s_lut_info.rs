#[doc = "Register `S_LUT_INFO` reader"]
pub struct R(crate::R<S_LUT_INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_INFO` writer"]
pub struct W(crate::W<S_LUT_INFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_INFO_SPEC>;
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
impl From<crate::W<S_LUT_INFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_INFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LE_INDEX_OFFSET` reader - "]
pub struct LUT_LE_INDEX_OFFSET_R(crate::FieldReader<u8, u8>);
impl LUT_LE_INDEX_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_INDEX_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_INDEX_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_INDEX_OFFSET` writer - "]
pub struct LUT_LE_INDEX_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_INDEX_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `LUT_LE_INDEX_SELECT` reader - "]
pub struct LUT_LE_INDEX_SELECT_R(crate::FieldReader<u8, u8>);
impl LUT_LE_INDEX_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_INDEX_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_INDEX_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_INDEX_SELECT` writer - "]
pub struct LUT_LE_INDEX_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LE_INDEX_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `LUT_LO_INDEX_SELECT` reader - "]
pub struct LUT_LO_INDEX_SELECT_R(crate::FieldReader<u8, u8>);
impl LUT_LO_INDEX_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LO_INDEX_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_INDEX_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LO_INDEX_SELECT` writer - "]
pub struct LUT_LO_INDEX_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_LO_INDEX_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lut_le_index_offset(&self) -> LUT_LE_INDEX_OFFSET_R {
        LUT_LE_INDEX_OFFSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lut_le_index_select(&self) -> LUT_LE_INDEX_SELECT_R {
        LUT_LE_INDEX_SELECT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn lut_lo_index_select(&self) -> LUT_LO_INDEX_SELECT_R {
        LUT_LO_INDEX_SELECT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lut_le_index_offset(&mut self) -> LUT_LE_INDEX_OFFSET_W {
        LUT_LE_INDEX_OFFSET_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn lut_le_index_select(&mut self) -> LUT_LE_INDEX_SELECT_W {
        LUT_LE_INDEX_SELECT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn lut_lo_index_select(&mut self) -> LUT_LO_INDEX_SELECT_W {
        LUT_LO_INDEX_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LE and LO LUT index offset and selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_info](index.html) module"]
pub struct S_LUT_INFO_SPEC;
impl crate::RegisterSpec for S_LUT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_info::R](R) reader structure"]
impl crate::Readable for S_LUT_INFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_info::W](W) writer structure"]
impl crate::Writable for S_LUT_INFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S_LUT_INFO to value 0"]
impl crate::Resettable for S_LUT_INFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
