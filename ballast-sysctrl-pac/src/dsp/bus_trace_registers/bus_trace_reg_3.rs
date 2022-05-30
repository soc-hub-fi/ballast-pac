#[doc = "Register `bus_trace_reg_3` reader"]
pub struct R(crate::R<BUS_TRACE_REG_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TRACE_REG_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TRACE_REG_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TRACE_REG_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `bus_trace_reg_3` reader - "]
pub struct BUS_TRACE_REG_3_R(crate::FieldReader<u32>);
impl BUS_TRACE_REG_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BUS_TRACE_REG_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_TRACE_REG_3_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_trace_reg_3(&self) -> BUS_TRACE_REG_3_R {
        BUS_TRACE_REG_3_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_trace_reg_3](index.html) module"]
pub struct BUS_TRACE_REG_3_SPEC;
impl crate::RegisterSpec for BUS_TRACE_REG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_trace_reg_3::R](R) reader structure"]
impl crate::Readable for BUS_TRACE_REG_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bus_trace_reg_3 to value 0"]
impl crate::Resettable for BUS_TRACE_REG_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
