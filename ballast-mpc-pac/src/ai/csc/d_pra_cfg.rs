#[doc = "Register `D_PRA_CFG` reader"]
pub struct R(crate::R<D_PRA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PRA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PRA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PRA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRA_TRUNCATE` reader - "]
pub struct PRA_TRUNCATE_R(crate::FieldReader<u8, u8>);
impl PRA_TRUNCATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRA_TRUNCATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRA_TRUNCATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pra_truncate(&self) -> PRA_TRUNCATE_R {
        PRA_TRUNCATE_R::new((self.bits & 3) as u8)
    }
}
#[doc = "PRA truncate in Winograd mode, range: 0~2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pra_cfg](index.html) module"]
pub struct D_PRA_CFG_SPEC;
impl crate::RegisterSpec for D_PRA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pra_cfg::R](R) reader structure"]
impl crate::Readable for D_PRA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_PRA_CFG to value 0"]
impl crate::Resettable for D_PRA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
