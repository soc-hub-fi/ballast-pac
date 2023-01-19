#[doc = "Register `bus_trace_reg_1` reader"]
pub struct R(crate::R<BUS_TRACE_REG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TRACE_REG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TRACE_REG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TRACE_REG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `bus_trace_reg_2` reader - "]
pub type BUS_TRACE_REG_2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_trace_reg_2(&self) -> BUS_TRACE_REG_2_R {
        BUS_TRACE_REG_2_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_trace_reg_1](index.html) module"]
pub struct BUS_TRACE_REG_1_SPEC;
impl crate::RegisterSpec for BUS_TRACE_REG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_trace_reg_1::R](R) reader structure"]
impl crate::Readable for BUS_TRACE_REG_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bus_trace_reg_1 to value 0"]
impl crate::Resettable for BUS_TRACE_REG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
