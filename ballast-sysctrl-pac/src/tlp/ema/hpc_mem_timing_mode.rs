#[doc = "Register `hpc_mem_timing_mode` reader"]
pub struct R(crate::R<HPC_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPC_MEM_TIMING_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPC_MEM_TIMING_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPC_MEM_TIMING_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hpc_mem_timing_mode` writer"]
pub struct W(crate::W<HPC_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPC_MEM_TIMING_MODE_SPEC>;
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
impl From<crate::W<HPC_MEM_TIMING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPC_MEM_TIMING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hpc_mem_timing_mode` reader - "]
pub type HPC_MEM_TIMING_MODE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `hpc_mem_timing_mode` writer - "]
pub type HPC_MEM_TIMING_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, HPC_MEM_TIMING_MODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn hpc_mem_timing_mode(&self) -> HPC_MEM_TIMING_MODE_R {
        HPC_MEM_TIMING_MODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn hpc_mem_timing_mode(&mut self) -> HPC_MEM_TIMING_MODE_W<0> {
        HPC_MEM_TIMING_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpc_mem_timing_mode](index.html) module"]
pub struct HPC_MEM_TIMING_MODE_SPEC;
impl crate::RegisterSpec for HPC_MEM_TIMING_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [hpc_mem_timing_mode::R](R) reader structure"]
impl crate::Readable for HPC_MEM_TIMING_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpc_mem_timing_mode::W](W) writer structure"]
impl crate::Writable for HPC_MEM_TIMING_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hpc_mem_timing_mode to value 0xffff"]
impl crate::Resettable for HPC_MEM_TIMING_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
