#[doc = "Register `CPU_rd_dout_l` reader"]
pub struct R(crate::R<CPU_RD_DOUT_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_RD_DOUT_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_RD_DOUT_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_RD_DOUT_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_rd_dout_l` reader - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub struct CPU_RD_DOUT_L_R(crate::FieldReader<u16, u16>);
impl CPU_RD_DOUT_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CPU_RD_DOUT_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_RD_DOUT_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    #[inline(always)]
    pub fn cpu_rd_dout_l(&self) -> CPU_RD_DOUT_L_R {
        CPU_RD_DOUT_L_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_rd_dout_l](index.html) module"]
pub struct CPU_RD_DOUT_L_SPEC;
impl crate::RegisterSpec for CPU_RD_DOUT_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_rd_dout_l::R](R) reader structure"]
impl crate::Readable for CPU_RD_DOUT_L_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_rd_dout_l to value 0"]
impl crate::Resettable for CPU_RD_DOUT_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
