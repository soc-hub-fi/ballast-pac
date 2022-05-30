#[doc = "Register `MDIO_RdData` reader"]
pub struct R(crate::R<MDIO_RDDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIO_RDDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIO_RDDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIO_RDDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDIO_RdData` reader - "]
pub struct MDIO_RDDATA_R(crate::FieldReader<u16>);
impl MDIO_RDDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MDIO_RDDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIO_RDDATA_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mdio_rd_data(&self) -> MDIO_RDDATA_R {
        MDIO_RDDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "After MDIO read transaction has completed, this register is updated with the read result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdio_rd_data](index.html) module"]
pub struct MDIO_RDDATA_SPEC;
impl crate::RegisterSpec for MDIO_RDDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdio_rd_data::R](R) reader structure"]
impl crate::Readable for MDIO_RDDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIO_RdData to value 0"]
impl crate::Resettable for MDIO_RDDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
