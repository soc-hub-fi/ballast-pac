#[doc = "Register `breakpoint_enable` reader"]
pub struct R(crate::R<BREAKPOINT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREAKPOINT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BREAKPOINT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BREAKPOINT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `breakpoint_enable` writer"]
pub struct W(crate::W<BREAKPOINT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BREAKPOINT_ENABLE_SPEC>;
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
impl From<crate::W<BREAKPOINT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BREAKPOINT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `single_step_breakpoint_enable` reader - "]
pub type SINGLE_STEP_BREAKPOINT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `single_step_breakpoint_enable` writer - "]
pub type SINGLE_STEP_BREAKPOINT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BREAKPOINT_ENABLE_SPEC, bool, O>;
#[doc = "Field `enable_breakpoint_1` reader - "]
pub type ENABLE_BREAKPOINT_1_R = crate::BitReader<bool>;
#[doc = "Field `enable_breakpoint_1` writer - "]
pub type ENABLE_BREAKPOINT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BREAKPOINT_ENABLE_SPEC, bool, O>;
#[doc = "Field `enable_breakpoint_2` reader - "]
pub type ENABLE_BREAKPOINT_2_R = crate::BitReader<bool>;
#[doc = "Field `enable_breakpoint_2` writer - "]
pub type ENABLE_BREAKPOINT_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BREAKPOINT_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn single_step_breakpoint_enable(&self) -> SINGLE_STEP_BREAKPOINT_ENABLE_R {
        SINGLE_STEP_BREAKPOINT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_breakpoint_1(&self) -> ENABLE_BREAKPOINT_1_R {
        ENABLE_BREAKPOINT_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn enable_breakpoint_2(&self) -> ENABLE_BREAKPOINT_2_R {
        ENABLE_BREAKPOINT_2_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn single_step_breakpoint_enable(&mut self) -> SINGLE_STEP_BREAKPOINT_ENABLE_W<2> {
        SINGLE_STEP_BREAKPOINT_ENABLE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_1(&mut self) -> ENABLE_BREAKPOINT_1_W<3> {
        ENABLE_BREAKPOINT_1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn enable_breakpoint_2(&mut self) -> ENABLE_BREAKPOINT_2_W<4> {
        ENABLE_BREAKPOINT_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Breakpoint enable Bit 0-1: Reserved Bit 2: Enable single-stepping breakpoint Bit 3: Enable breakpoint 1 Bit 3: Enable breakpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breakpoint_enable](index.html) module"]
pub struct BREAKPOINT_ENABLE_SPEC;
impl crate::RegisterSpec for BREAKPOINT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [breakpoint_enable::R](R) reader structure"]
impl crate::Readable for BREAKPOINT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [breakpoint_enable::W](W) writer structure"]
impl crate::Writable for BREAKPOINT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets breakpoint_enable to value 0"]
impl crate::Resettable for BREAKPOINT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
