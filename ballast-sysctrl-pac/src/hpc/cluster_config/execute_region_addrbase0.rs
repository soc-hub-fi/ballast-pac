#[doc = "Register `execute_region_addrbase0` reader"]
pub struct R(crate::R<EXECUTE_REGION_ADDRBASE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXECUTE_REGION_ADDRBASE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXECUTE_REGION_ADDRBASE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXECUTE_REGION_ADDRBASE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `execute_region_addrbase0` writer"]
pub struct W(crate::W<EXECUTE_REGION_ADDRBASE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXECUTE_REGION_ADDRBASE0_SPEC>;
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
impl From<crate::W<EXECUTE_REGION_ADDRBASE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXECUTE_REGION_ADDRBASE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `execute_region_addrbase0` reader - "]
pub struct EXECUTE_REGION_ADDRBASE0_R(crate::FieldReader<u64>);
impl EXECUTE_REGION_ADDRBASE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        EXECUTE_REGION_ADDRBASE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXECUTE_REGION_ADDRBASE0_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `execute_region_addrbase0` writer - "]
pub struct EXECUTE_REGION_ADDRBASE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTE_REGION_ADDRBASE0_W<'a> {
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
    pub fn execute_region_addrbase0(&self) -> EXECUTE_REGION_ADDRBASE0_R {
        EXECUTE_REGION_ADDRBASE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn execute_region_addrbase0(&mut self) -> EXECUTE_REGION_ADDRBASE0_W {
        EXECUTE_REGION_ADDRBASE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Execution region base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [execute_region_addrbase0](index.html) module"]
pub struct EXECUTE_REGION_ADDRBASE0_SPEC;
impl crate::RegisterSpec for EXECUTE_REGION_ADDRBASE0_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [execute_region_addrbase0::R](R) reader structure"]
impl crate::Readable for EXECUTE_REGION_ADDRBASE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [execute_region_addrbase0::W](W) writer structure"]
impl crate::Writable for EXECUTE_REGION_ADDRBASE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets execute_region_addrbase0 to value 0"]
impl crate::Resettable for EXECUTE_REGION_ADDRBASE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
