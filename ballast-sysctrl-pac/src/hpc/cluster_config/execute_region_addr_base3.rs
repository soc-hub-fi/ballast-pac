#[doc = "Register `execute_region_addr_base3` reader"]
pub struct R(crate::R<EXECUTE_REGION_ADDR_BASE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXECUTE_REGION_ADDR_BASE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXECUTE_REGION_ADDR_BASE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXECUTE_REGION_ADDR_BASE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `execute_region_addr_base3` writer"]
pub struct W(crate::W<EXECUTE_REGION_ADDR_BASE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXECUTE_REGION_ADDR_BASE3_SPEC>;
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
impl From<crate::W<EXECUTE_REGION_ADDR_BASE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXECUTE_REGION_ADDR_BASE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `execute_region_addr_base3` reader - "]
pub struct EXECUTE_REGION_ADDR_BASE3_R(crate::FieldReader<u64, u64>);
impl EXECUTE_REGION_ADDR_BASE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        EXECUTE_REGION_ADDR_BASE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXECUTE_REGION_ADDR_BASE3_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `execute_region_addr_base3` writer - "]
pub struct EXECUTE_REGION_ADDR_BASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTE_REGION_ADDR_BASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn execute_region_addr_base3(&self) -> EXECUTE_REGION_ADDR_BASE3_R {
        EXECUTE_REGION_ADDR_BASE3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn execute_region_addr_base3(&mut self) -> EXECUTE_REGION_ADDR_BASE3_W {
        EXECUTE_REGION_ADDR_BASE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [execute_region_addr_base3](index.html) module"]
pub struct EXECUTE_REGION_ADDR_BASE3_SPEC;
impl crate::RegisterSpec for EXECUTE_REGION_ADDR_BASE3_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [execute_region_addr_base3::R](R) reader structure"]
impl crate::Readable for EXECUTE_REGION_ADDR_BASE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [execute_region_addr_base3::W](W) writer structure"]
impl crate::Writable for EXECUTE_REGION_ADDR_BASE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets execute_region_addr_base3 to value 0"]
impl crate::Resettable for EXECUTE_REGION_ADDR_BASE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}