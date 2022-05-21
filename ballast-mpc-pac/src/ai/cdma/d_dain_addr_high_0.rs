#[doc = "Register `D_DAIN_ADDR_HIGH_0` reader"]
pub struct R(crate::R<D_DAIN_ADDR_HIGH_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DAIN_ADDR_HIGH_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DAIN_ADDR_HIGH_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DAIN_ADDR_HIGH_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATAIN_ADDR_HIGH_0` reader - "]
pub struct DATAIN_ADDR_HIGH_0_R(crate::FieldReader<u32, u32>);
impl DATAIN_ADDR_HIGH_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DATAIN_ADDR_HIGH_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIN_ADDR_HIGH_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn datain_addr_high_0(&self) -> DATAIN_ADDR_HIGH_0_R {
        DATAIN_ADDR_HIGH_0_R::new(self.bits)
    }
}
#[doc = "Higher 32bits of input data address when axi araddr is 64bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dain_addr_high_0](index.html) module"]
pub struct D_DAIN_ADDR_HIGH_0_SPEC;
impl crate::RegisterSpec for D_DAIN_ADDR_HIGH_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dain_addr_high_0::R](R) reader structure"]
impl crate::Readable for D_DAIN_ADDR_HIGH_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets D_DAIN_ADDR_HIGH_0 to value 0"]
impl crate::Resettable for D_DAIN_ADDR_HIGH_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
