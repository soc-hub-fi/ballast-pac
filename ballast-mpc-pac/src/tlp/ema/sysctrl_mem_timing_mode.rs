#[doc = "Register `sysctrl_mem_timing_mode` writer"]
pub struct W(crate::W<SYSCTRL_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTRL_MEM_TIMING_MODE_SPEC>;
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
impl From<crate::W<SYSCTRL_MEM_TIMING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTRL_MEM_TIMING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sysctrl_mem_timing_mode` writer - "]
pub struct SYSCTRL_MEM_TIMING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCTRL_MEM_TIMING_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sysctrl_mem_timing_mode(&mut self) -> SYSCTRL_MEM_TIMING_MODE_W {
        SYSCTRL_MEM_TIMING_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctrl_mem_timing_mode](index.html) module"]
pub struct SYSCTRL_MEM_TIMING_MODE_SPEC;
impl crate::RegisterSpec for SYSCTRL_MEM_TIMING_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [sysctrl_mem_timing_mode::W](W) writer structure"]
impl crate::Writable for SYSCTRL_MEM_TIMING_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sysctrl_mem_timing_mode to value 0xffff"]
impl crate::Resettable for SYSCTRL_MEM_TIMING_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
