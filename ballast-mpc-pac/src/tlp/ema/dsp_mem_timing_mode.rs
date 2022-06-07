#[doc = "Register `dsp_mem_timing_mode` reader"]
pub struct R(crate::R<DSP_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_MEM_TIMING_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_MEM_TIMING_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_MEM_TIMING_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsp_mem_timing_mode` writer"]
pub struct W(crate::W<DSP_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_MEM_TIMING_MODE_SPEC>;
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
impl From<crate::W<DSP_MEM_TIMING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_MEM_TIMING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dsp_mem_timing_mode` reader - "]
pub struct DSP_MEM_TIMING_MODE_R(crate::FieldReader<u16>);
impl DSP_MEM_TIMING_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DSP_MEM_TIMING_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSP_MEM_TIMING_MODE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dsp_mem_timing_mode` writer - "]
pub struct DSP_MEM_TIMING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSP_MEM_TIMING_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dsp_mem_timing_mode(&self) -> DSP_MEM_TIMING_MODE_R {
        DSP_MEM_TIMING_MODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dsp_mem_timing_mode(&mut self) -> DSP_MEM_TIMING_MODE_W {
        DSP_MEM_TIMING_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_mem_timing_mode](index.html) module"]
pub struct DSP_MEM_TIMING_MODE_SPEC;
impl crate::RegisterSpec for DSP_MEM_TIMING_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dsp_mem_timing_mode::R](R) reader structure"]
impl crate::Readable for DSP_MEM_TIMING_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_mem_timing_mode::W](W) writer structure"]
impl crate::Writable for DSP_MEM_TIMING_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dsp_mem_timing_mode to value 0xffff"]
impl crate::Resettable for DSP_MEM_TIMING_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
