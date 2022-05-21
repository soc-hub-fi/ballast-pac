#[doc = "Register `REG_HPC_PLL_LOOP_CTRL` reader"]
pub struct R(crate::R<REG_HPC_PLL_LOOP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_HPC_PLL_LOOP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_HPC_PLL_LOOP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_HPC_PLL_LOOP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_HPC_PLL_LOOP_CTRL` writer"]
pub struct W(crate::W<REG_HPC_PLL_LOOP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_HPC_PLL_LOOP_CTRL_SPEC>;
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
impl From<crate::W<REG_HPC_PLL_LOOP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_HPC_PLL_LOOP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOP_CTRL` reader - "]
pub struct LOOP_CTRL_R(crate::FieldReader<u32, u32>);
impl LOOP_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOOP_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOP_CTRL_R {
    type Target = crate::FieldReader<u32, u32>;
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_hpc_pll_loop_ctrl](index.html) module"]
pub struct REG_HPC_PLL_LOOP_CTRL_SPEC;
impl crate::RegisterSpec for REG_HPC_PLL_LOOP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_hpc_pll_loop_ctrl::R](R) reader structure"]
impl crate::Readable for REG_HPC_PLL_LOOP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_hpc_pll_loop_ctrl::W](W) writer structure"]
impl crate::Writable for REG_HPC_PLL_LOOP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_HPC_PLL_LOOP_CTRL to value 0"]
impl crate::Resettable for REG_HPC_PLL_LOOP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
