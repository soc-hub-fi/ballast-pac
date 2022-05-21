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
pub struct CPU_RD_ADDR_R(crate::FieldReader<u8, u8>);
impl CPU_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_RD_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_rd_addr` writer - All statistic counters are stored in a blockram. When you read a counter,you need to write the corresponding address to CPU_rd_addr register and assert CPU_rd_apply signal. When the counter data register CPU_rd_dout is available , the signal CPU_rd_grant will be “1”."]
pub struct CPU_RD_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_RD_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
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
    pub fn cpu_rd_addr(&mut self) -> CPU_RD_ADDR_W {
        CPU_RD_ADDR_W { w: self }
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
}
#[doc = "`reset()` method sets CPU_rd_addr to value 0"]
impl crate::Resettable for CPU_RD_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
