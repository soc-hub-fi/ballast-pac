#[doc = "Register `breakpoint_1_addr` reader"]
pub struct R(crate::R<BREAKPOINT_1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREAKPOINT_1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BREAKPOINT_1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BREAKPOINT_1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `breakpoint_1_addr` writer"]
pub struct W(crate::W<BREAKPOINT_1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BREAKPOINT_1_ADDR_SPEC>;
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
impl From<crate::W<BREAKPOINT_1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BREAKPOINT_1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `breakpoint_1_addr` reader - "]
pub struct BREAKPOINT_1_ADDR_R(crate::FieldReader<u32>);
impl BREAKPOINT_1_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BREAKPOINT_1_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREAKPOINT_1_ADDR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `breakpoint_1_addr` writer - "]
pub struct BREAKPOINT_1_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAKPOINT_1_ADDR_W<'a> {
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
    pub fn breakpoint_1_addr(&self) -> BREAKPOINT_1_ADDR_R {
        BREAKPOINT_1_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn breakpoint_1_addr(&mut self) -> BREAKPOINT_1_ADDR_W {
        BREAKPOINT_1_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breakpoint_1_addr](index.html) module"]
pub struct BREAKPOINT_1_ADDR_SPEC;
impl crate::RegisterSpec for BREAKPOINT_1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [breakpoint_1_addr::R](R) reader structure"]
impl crate::Readable for BREAKPOINT_1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [breakpoint_1_addr::W](W) writer structure"]
impl crate::Writable for BREAKPOINT_1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets breakpoint_1_addr to value 0"]
impl crate::Resettable for BREAKPOINT_1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
