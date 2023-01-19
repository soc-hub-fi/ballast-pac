#[doc = "Register `mpc_mem_timing_mode` writer"]
pub struct W(crate::W<MPC_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPC_MEM_TIMING_MODE_SPEC>;
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
impl From<crate::W<MPC_MEM_TIMING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPC_MEM_TIMING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mpc_mem_timing_mode` writer - "]
pub type MPC_MEM_TIMING_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, MPC_MEM_TIMING_MODE_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn mpc_mem_timing_mode(&mut self) -> MPC_MEM_TIMING_MODE_W<0> {
        MPC_MEM_TIMING_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpc_mem_timing_mode](index.html) module"]
pub struct MPC_MEM_TIMING_MODE_SPEC;
impl crate::RegisterSpec for MPC_MEM_TIMING_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [mpc_mem_timing_mode::W](W) writer structure"]
impl crate::Writable for MPC_MEM_TIMING_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mpc_mem_timing_mode to value 0xffff"]
impl crate::Resettable for MPC_MEM_TIMING_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
