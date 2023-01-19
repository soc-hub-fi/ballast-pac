#[doc = "Register `CPU_rd_dout_h` reader"]
pub struct R(crate::R<CPU_RD_DOUT_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_RD_DOUT_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_RD_DOUT_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_RD_DOUT_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_rd_dout_h` reader - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub type CPU_RD_DOUT_H_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    #[inline(always)]
    pub fn cpu_rd_dout_h(&self) -> CPU_RD_DOUT_H_R {
        CPU_RD_DOUT_H_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_rd_dout_h](index.html) module"]
pub struct CPU_RD_DOUT_H_SPEC;
impl crate::RegisterSpec for CPU_RD_DOUT_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_rd_dout_h::R](R) reader structure"]
impl crate::Readable for CPU_RD_DOUT_H_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_rd_dout_h to value 0"]
impl crate::Resettable for CPU_RD_DOUT_H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
