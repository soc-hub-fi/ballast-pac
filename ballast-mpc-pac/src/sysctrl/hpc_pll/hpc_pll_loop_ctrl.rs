#[doc = "Register `HPC_PLL_LOOP_CTRL` reader"]
pub struct R(crate::R<HPC_PLL_LOOP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPC_PLL_LOOP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPC_PLL_LOOP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPC_PLL_LOOP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPC_PLL_LOOP_CTRL` writer"]
pub struct W(crate::W<HPC_PLL_LOOP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPC_PLL_LOOP_CTRL_SPEC>;
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
impl From<crate::W<HPC_PLL_LOOP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPC_PLL_LOOP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOP_CTRL` reader - "]
pub struct LOOP_CTRL_R(crate::FieldReader<u32>);
impl LOOP_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOOP_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_CTRL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP_CTRL` writer - "]
pub struct LOOP_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn loop_ctrl(&self) -> LOOP_CTRL_R {
        LOOP_CTRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn loop_ctrl(&mut self) -> LOOP_CTRL_W {
        LOOP_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpc_pll_loop_ctrl](index.html) module"]
pub struct HPC_PLL_LOOP_CTRL_SPEC;
impl crate::RegisterSpec for HPC_PLL_LOOP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpc_pll_loop_ctrl::R](R) reader structure"]
impl crate::Readable for HPC_PLL_LOOP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpc_pll_loop_ctrl::W](W) writer structure"]
impl crate::Writable for HPC_PLL_LOOP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HPC_PLL_LOOP_CTRL to value 0"]
impl crate::Resettable for HPC_PLL_LOOP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
