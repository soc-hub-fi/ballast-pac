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
pub struct SINGLE_STEP_BREAKPOINT_ENABLE_R(crate::FieldReader<bool>);
impl SINGLE_STEP_BREAKPOINT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_STEP_BREAKPOINT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_STEP_BREAKPOINT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `single_step_breakpoint_enable` writer - "]
pub struct SINGLE_STEP_BREAKPOINT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_STEP_BREAKPOINT_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `enable_breakpoint_1` reader - "]
pub struct ENABLE_BREAKPOINT_1_R(crate::FieldReader<bool>);
impl ENABLE_BREAKPOINT_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_BREAKPOINT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_BREAKPOINT_1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable_breakpoint_1` writer - "]
pub struct ENABLE_BREAKPOINT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BREAKPOINT_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `enable_breakpoint_2` reader - "]
pub struct ENABLE_BREAKPOINT_2_R(crate::FieldReader<bool>);
impl ENABLE_BREAKPOINT_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_BREAKPOINT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_BREAKPOINT_2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable_breakpoint_2` writer - "]
pub struct ENABLE_BREAKPOINT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BREAKPOINT_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
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
    pub fn single_step_breakpoint_enable(&mut self) -> SINGLE_STEP_BREAKPOINT_ENABLE_W {
        SINGLE_STEP_BREAKPOINT_ENABLE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn enable_breakpoint_1(&mut self) -> ENABLE_BREAKPOINT_1_W {
        ENABLE_BREAKPOINT_1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn enable_breakpoint_2(&mut self) -> ENABLE_BREAKPOINT_2_W {
        ENABLE_BREAKPOINT_2_W { w: self }
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
}
#[doc = "`reset()` method sets breakpoint_enable to value 0"]
impl crate::Resettable for BREAKPOINT_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
