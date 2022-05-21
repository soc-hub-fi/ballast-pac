#[doc = "Register `CID3` reader"]
pub struct R(crate::R<CID3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `card_id_word3` reader - "]
pub struct CARD_ID_WORD3_R(crate::FieldReader<u32, u32>);
impl CARD_ID_WORD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CARD_ID_WORD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_ID_WORD3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word3(&self) -> CARD_ID_WORD3_R {
        CARD_ID_WORD3_R::new(self.bits)
    }
}
#[doc = "Card Identification Word 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](index.html) module"]
pub struct CID3_SPEC;
impl crate::RegisterSpec for CID3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid3::R](R) reader structure"]
impl crate::Readable for CID3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID3 to value 0"]
impl crate::Resettable for CID3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
