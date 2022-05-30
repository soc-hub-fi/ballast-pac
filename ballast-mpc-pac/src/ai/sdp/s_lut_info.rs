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
#[doc = "Field `LUT_LE_INDEX_OFFSET` reader - "]
pub struct LUT_LE_INDEX_OFFSET_R(crate::FieldReader<u8>);
impl LUT_LE_INDEX_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_INDEX_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_INDEX_OFFSET_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LE_INDEX_SELECT` reader - "]
pub struct LUT_LE_INDEX_SELECT_R(crate::FieldReader<u8>);
impl LUT_LE_INDEX_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LE_INDEX_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LE_INDEX_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUT_LO_INDEX_SELECT` reader - "]
pub struct LUT_LO_INDEX_SELECT_R(crate::FieldReader<u8>);
impl LUT_LO_INDEX_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUT_LO_INDEX_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LUT_LO_INDEX_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "LE and LO LUT index offset and selection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_info](index.html) module"]
pub struct S_LUT_INFO_SPEC;
impl crate::RegisterSpec for S_LUT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_info::R](R) reader structure"]
impl crate::Readable for S_LUT_INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S_LUT_INFO to value 0"]
impl crate::Resettable for S_LUT_INFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
