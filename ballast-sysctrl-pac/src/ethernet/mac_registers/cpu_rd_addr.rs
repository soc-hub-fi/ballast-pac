#[doc = "Register `CPU_rd_addr` reader"]
pub struct R(crate::R<CPU_RD_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_RD_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_RD_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_RD_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_rd_addr` writer"]
pub struct W(crate::W<CPU_RD_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_RD_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CPU_RD_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_RD_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_rd_addr` reader - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub type CPU_RD_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_rd_addr` writer - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub type CPU_RD_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_RD_ADDR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    #[inline(always)]
    pub fn cpu_rd_addr(&self) -> CPU_RD_ADDR_R {
        CPU_RD_ADDR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_rd_addr(&mut self) -> CPU_RD_ADDR_W<0> {
        CPU_RD_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_rd_addr](index.html) module"]
pub struct CPU_RD_ADDR_SPEC;
impl crate::RegisterSpec for CPU_RD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_rd_addr::R](R) reader structure"]
impl crate::Readable for CPU_RD_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_rd_addr::W](W) writer structure"]
impl crate::Writable for CPU_RD_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_rd_addr to value 0"]
impl crate::Resettable for CPU_RD_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
