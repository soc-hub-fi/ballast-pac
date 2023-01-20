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
pub type LUT_LE_INDEX_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_LE_INDEX_OFFSET` writer - "]
pub type LUT_LE_INDEX_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_INFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `LUT_LE_INDEX_SELECT` reader - "]
pub type LUT_LE_INDEX_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_LE_INDEX_SELECT` writer - "]
pub type LUT_LE_INDEX_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_INFO_SPEC, u8, u8, 8, O>;
#[doc = "Field `LUT_LO_INDEX_SELECT` reader - "]
pub type LUT_LO_INDEX_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_LO_INDEX_SELECT` writer - "]
pub type LUT_LO_INDEX_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_INFO_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn lut_le_index_offset(&mut self) -> LUT_LE_INDEX_OFFSET_W<0> {
        LUT_LE_INDEX_OFFSET_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn lut_le_index_select(&mut self) -> LUT_LE_INDEX_SELECT_W<8> {
        LUT_LE_INDEX_SELECT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn lut_lo_index_select(&mut self) -> LUT_LO_INDEX_SELECT_W<16> {
        LUT_LO_INDEX_SELECT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_INFO to value 0"]
impl crate::Resettable for S_LUT_INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
