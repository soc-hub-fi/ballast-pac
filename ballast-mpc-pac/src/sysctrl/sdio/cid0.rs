#[doc = "Register `CID0` reader"]
pub struct R(crate::R<CID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `card_id_word0` reader - "]
pub type CARD_ID_WORD0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn card_id_word0(&self) -> CARD_ID_WORD0_R {
        CARD_ID_WORD0_R::new(self.bits)
    }
}
#[doc = "Card Identification Word0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](index.html) module"]
pub struct CID0_SPEC;
impl crate::RegisterSpec for CID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid0::R](R) reader structure"]
impl crate::Readable for CID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID0 to value 0"]
impl crate::Resettable for CID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
