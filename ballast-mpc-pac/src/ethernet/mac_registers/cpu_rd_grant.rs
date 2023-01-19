#[doc = "Register `CPU_rd_grant` reader"]
pub struct R(crate::R<CPU_RD_GRANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_RD_GRANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_RD_GRANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_RD_GRANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_rd_grant` reader - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub type CPU_RD_GRANT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    #[inline(always)]
    pub fn cpu_rd_grant(&self) -> CPU_RD_GRANT_R {
        CPU_RD_GRANT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_rd_grant](index.html) module"]
pub struct CPU_RD_GRANT_SPEC;
impl crate::RegisterSpec for CPU_RD_GRANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_rd_grant::R](R) reader structure"]
impl crate::Readable for CPU_RD_GRANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_rd_grant to value 0"]
impl crate::Resettable for CPU_RD_GRANT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
