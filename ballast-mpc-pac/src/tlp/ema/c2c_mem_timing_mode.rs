#[doc = "Register `C2C_mem_timing_mode` reader"]
pub struct R(crate::R<C2C_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2C_MEM_TIMING_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2C_MEM_TIMING_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2C_MEM_TIMING_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2C_mem_timing_mode` writer"]
pub struct W(crate::W<C2C_MEM_TIMING_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2C_MEM_TIMING_MODE_SPEC>;
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
impl From<crate::W<C2C_MEM_TIMING_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2C_MEM_TIMING_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `c2c_mem_timing_mode` reader - "]
pub type C2C_MEM_TIMING_MODE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `c2c_mem_timing_mode` writer - "]
pub type C2C_MEM_TIMING_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, C2C_MEM_TIMING_MODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn c2c_mem_timing_mode(&self) -> C2C_MEM_TIMING_MODE_R {
        C2C_MEM_TIMING_MODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn c2c_mem_timing_mode(&mut self) -> C2C_MEM_TIMING_MODE_W<0> {
        C2C_MEM_TIMING_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2c_mem_timing_mode](index.html) module"]
pub struct C2C_MEM_TIMING_MODE_SPEC;
impl crate::RegisterSpec for C2C_MEM_TIMING_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [c2c_mem_timing_mode::R](R) reader structure"]
impl crate::Readable for C2C_MEM_TIMING_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2c_mem_timing_mode::W](W) writer structure"]
impl crate::Writable for C2C_MEM_TIMING_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2C_mem_timing_mode to value 0xffff"]
impl crate::Resettable for C2C_MEM_TIMING_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
