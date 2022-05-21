#[doc = "Register `CID2` reader"]
pub struct R(crate::R<CID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `card_id_word2` reader - "]
pub struct CARD_ID_WORD2_R(crate::FieldReader<u32, u32>);
impl CARD_ID_WORD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CARD_ID_WORD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_ID_WORD2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word2(&self) -> CARD_ID_WORD2_R {
        CARD_ID_WORD2_R::new(self.bits)
    }
}
#[doc = "Card Identification Word 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](index.html) module"]
pub struct CID2_SPEC;
impl crate::RegisterSpec for CID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid2::R](R) reader structure"]
impl crate::Readable for CID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID2 to value 0"]
impl crate::Resettable for CID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
